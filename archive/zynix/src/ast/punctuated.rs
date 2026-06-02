use std::ops::{Index, IndexMut};
use std::{slice, vec};

use crate::parse::{ParseError, ParseStream};
use crate::token::ToTokens;
use crate::{Parse, TokenStream};

pub struct Punctuated<T, P> {
    inner: Vec<(T, P)>,
    last: Option<Box<T>>,
}

impl<T, P> Punctuated<T, P> {
    pub const fn new() -> Self {
        Self {
            inner: Vec::new(),
            last: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty() && self.last.is_none()
    }

    pub fn len(&self) -> usize {
        self.inner.len() + if self.last.is_some() { 1 } else { 0 }
    }

    pub fn first(&self) -> Option<&T> {
        self.iter().next()
    }

    pub fn first_mut(&mut self) -> Option<&mut T> {
        self.iter_mut().next()
    }

    pub fn last(&self) -> Option<&T> {
        self.iter().next_back()
    }

    pub fn last_mut(&mut self) -> Option<&mut T> {
        self.iter_mut().next_back()
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if let Some((value, _)) = self.inner.get(index) {
            Some(value)
        } else if index == self.inner.len() {
            self.last.as_deref()
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        let inner_len = self.inner.len();
        if let Some((value, _)) = self.inner.get_mut(index) {
            Some(value)
        } else if index == inner_len {
            self.last.as_deref_mut()
        } else {
            None
        }
    }

    pub fn trailing_punct(&self) -> bool {
        self.last.is_none() && !self.inner.is_empty()
    }

    pub fn empty_or_trailing(&self) -> bool {
        self.last.is_none()
    }

    pub fn push_value(&mut self, value: T) {
        assert!(
            self.empty_or_trailing(),
            "Punctuated::push_value: cannot push value if Punctuated is missing trailing punctuation",
        );
        self.last = Some(Box::new(value));
    }

    pub fn push_punct(&mut self, punct: P) {
        assert!(
            self.last.is_some(),
            "Punctuated::push_punct: cannot push punctuation if Punctuated is empty or already has trailing punctuation",
        );
        let last = self.last.take().unwrap();
        self.inner.push((*last, punct));
    }

    pub fn push(&mut self, value: T)
    where
        P: Default,
    {
        if !self.empty_or_trailing() {
            self.push_punct(P::default());
        }

        self.push_value(value);
    }

    pub fn insert(&mut self, index: usize, value: T)
    where
        P: Default,
    {
        assert!(index <= self.len(), "Punctuated::insert: index out of range");

        if index == self.len() {
            self.push(value);
        } else {
            self.inner.insert(index, (value, P::default()));
        }
    }

    pub fn pop(&mut self) -> Option<Pair<T, P>> {
        if self.last.is_some() {
            self.last.take().map(|t| Pair::End(*t))
        } else {
            self.inner.pop().map(|(t, p)| Pair::Punctuated(t, p))
        }
    }

    pub fn pop_punct(&mut self) -> Option<P> {
        if self.last.is_some() {
            None
        } else {
            let (t, p) = self.inner.pop()?;
            self.last = Some(Box::new(t));
            Some(p)
        }
    }

    pub fn clear(&mut self) {
        self.inner.clear();
        self.last = None;
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            inner: Box::new(PrivateIter {
                inner: self.inner.iter(),
                last: self.last.as_ref().map(Box::as_ref).into_iter(),
            }),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            inner: Box::new(PrivateIterMut {
                inner: self.inner.iter_mut(),
                last: self.last.as_mut().map(Box::as_mut).into_iter(),
            }),
        }
    }

    pub fn pairs(&self) -> Pairs<'_, T, P> {
        Pairs {
            inner: self.inner.iter(),
            last: self.last.as_ref().map(Box::as_ref).into_iter(),
        }
    }

    pub fn pairs_mut(&mut self) -> PairsMut<'_, T, P> {
        PairsMut {
            inner: self.inner.iter_mut(),
            last: self.last.as_mut().map(Box::as_mut).into_iter(),
        }
    }

    pub fn into_pairs(self) -> IntoPairs<T, P> {
        IntoPairs {
            inner: self.inner.into_iter(),
            last: self.last.map(|t| *t).into_iter(),
        }
    }
}

impl<T: Parse, P: Parse> Punctuated<T, P> {
    pub fn parse_terminated(stream: &mut ParseStream<'_>) -> Result<Self, ParseError> {
        let mut punctuated = Punctuated::new();
        loop {
            if stream.is_empty() {
                break;
            }

            punctuated.push_value(T::parse(stream)?);

            if stream.is_empty() {
                break;
            }

            punctuated.push_punct(P::parse(stream)?);
        }

        Ok(punctuated)
    }

    pub fn parse_separated_nonempty(stream: &mut ParseStream<'_>) -> Result<Self, ParseError> {
        let mut punctuated = Punctuated::new();

        loop {
            punctuated.push_value(T::parse(stream)?);
            let mut fork = stream.fork();

            match P::parse(&mut fork) {
                Err(_) => break,
                Ok(p) => {
                    stream.seek(&fork);
                    punctuated.push_punct(p);
                }
            }
        }

        Ok(punctuated)
    }
}

impl<T: Clone, P: Clone> Clone for Punctuated<T, P> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            last: self.last.clone(),
        }
    }
}

impl<T, P> Default for Punctuated<T, P> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: std::fmt::Debug, P: std::fmt::Debug> std::fmt::Debug for Punctuated<T, P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut list = f.debug_list();

        for (t, p) in &self.inner {
            list.entry(t);
            list.entry(p);
        }

        if let Some(last) = &self.last {
            list.entry(last);
        }

        list.finish()
    }
}

impl<T: PartialEq, P: PartialEq> PartialEq for Punctuated<T, P> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner && self.last == other.last
    }
}

impl<T: Eq, P: Eq> Eq for Punctuated<T, P> {}

impl<T: std::hash::Hash, P: std::hash::Hash> std::hash::Hash for Punctuated<T, P> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.inner.hash(state);
        self.last.hash(state);
    }
}

impl<T, P> Index<usize> for Punctuated<T, P> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if index.checked_add(1) == Some(self.len()) {
            match &self.last {
                Some(t) => t,
                None => &self.inner[index].0,
            }
        } else {
            &self.inner[index].0
        }
    }
}

impl<T, P> IndexMut<usize> for Punctuated<T, P> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index.checked_add(1) == Some(self.len()) {
            match &mut self.last {
                Some(t) => t,
                None => &mut self.inner[index].0,
            }
        } else {
            &mut self.inner[index].0
        }
    }
}

impl<T, P: Default> FromIterator<T> for Punctuated<T, P> {
    fn from_iter<I: IntoIterator<Item = T>>(i: I) -> Self {
        let mut ret = Punctuated::new();
        ret.extend(i);
        ret
    }
}

impl<T, P: Default> Extend<T> for Punctuated<T, P> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, i: I) {
        for value in i {
            self.push(value);
        }
    }
}

impl<T, P> FromIterator<Pair<T, P>> for Punctuated<T, P> {
    fn from_iter<I: IntoIterator<Item = Pair<T, P>>>(i: I) -> Self {
        let mut ret = Punctuated::new();
        let mut nomore = false;

        for pair in i {
            assert!(!nomore, "Punctuated extended with items after a Pair::End");

            match pair {
                Pair::Punctuated(t, p) => ret.inner.push((t, p)),
                Pair::End(t) => {
                    ret.last = Some(Box::new(t));
                    nomore = true;
                }
            }
        }

        ret
    }
}

impl<T, P: Default> Extend<Pair<T, P>> for Punctuated<T, P> {
    fn extend<I: IntoIterator<Item = Pair<T, P>>>(&mut self, i: I) {
        if !self.empty_or_trailing() {
            self.push_punct(P::default());
        }

        let mut nomore = false;

        for pair in i {
            assert!(!nomore, "Punctuated extended with items after a Pair::End");

            match pair {
                Pair::Punctuated(t, p) => self.inner.push((t, p)),
                Pair::End(t) => {
                    self.last = Some(Box::new(t));
                    nomore = true;
                }
            }
        }
    }
}

impl<T, P> IntoIterator for Punctuated<T, P> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        let mut elements = Vec::with_capacity(self.len());

        for (t, _) in self.inner {
            elements.push(t);
        }

        if let Some(t) = self.last {
            elements.push(*t);
        }

        IntoIter {
            inner: elements.into_iter(),
        }
    }
}

impl<'a, T, P> IntoIterator for &'a Punctuated<T, P> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T, P> IntoIterator for &'a mut Punctuated<T, P> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T: ToTokens, P: ToTokens> ToTokens for Punctuated<T, P> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for (t, p) in &self.inner {
            t.to_tokens(tokens);
            p.to_tokens(tokens);
        }

        if let Some(last) = &self.last {
            last.to_tokens(tokens);
        }
    }
}

pub enum Pair<T, P> {
    Punctuated(T, P),
    End(T),
}

impl<T, P> Pair<T, P> {
    pub fn new(t: T, p: Option<P>) -> Self {
        match p {
            Some(p) => Pair::Punctuated(t, p),
            None => Pair::End(t),
        }
    }

    pub fn value(&self) -> &T {
        match self {
            Pair::Punctuated(t, _) | Pair::End(t) => t,
        }
    }

    pub fn value_mut(&mut self) -> &mut T {
        match self {
            Pair::Punctuated(t, _) | Pair::End(t) => t,
        }
    }

    pub fn punct(&self) -> Option<&P> {
        match self {
            Pair::Punctuated(_, p) => Some(p),
            Pair::End(_) => None,
        }
    }

    pub fn punct_mut(&mut self) -> Option<&mut P> {
        match self {
            Pair::Punctuated(_, p) => Some(p),
            Pair::End(_) => None,
        }
    }

    pub fn into_value(self) -> T {
        match self {
            Pair::Punctuated(t, _) | Pair::End(t) => t,
        }
    }

    pub fn into_tuple(self) -> (T, Option<P>) {
        match self {
            Pair::Punctuated(t, p) => (t, Some(p)),
            Pair::End(t) => (t, None),
        }
    }
}

impl<T: Clone, P: Clone> Clone for Pair<T, P> {
    fn clone(&self) -> Self {
        match self {
            Pair::Punctuated(t, p) => Pair::Punctuated(t.clone(), p.clone()),
            Pair::End(t) => Pair::End(t.clone()),
        }
    }
}

impl<T: ToTokens, P: ToTokens> ToTokens for Pair<T, P> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Pair::Punctuated(t, p) => {
                t.to_tokens(tokens);
                p.to_tokens(tokens);
            }
            Pair::End(t) => t.to_tokens(tokens),
        }
    }
}

pub struct IntoIter<T> {
    inner: vec::IntoIter<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.inner.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(), Some(self.len()))
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<T> {
        self.inner.next_back()
    }
}

impl<T> ExactSizeIterator for IntoIter<T> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<T: Clone> Clone for IntoIter<T> {
    fn clone(&self) -> Self {
        IntoIter {
            inner: self.inner.clone(),
        }
    }
}

// Iter — hides P via trait object

trait IterTrait<'a, T: 'a>: Iterator<Item = &'a T> + DoubleEndedIterator + ExactSizeIterator {
    fn clone_box(&self) -> Box<dyn IterTrait<'a, T> + 'a>;
}

struct PrivateIter<'a, T: 'a, P: 'a> {
    inner: slice::Iter<'a, (T, P)>,
    last: std::option::IntoIter<&'a T>,
}

impl<'a, T, P> Iterator for PrivateIter<'a, T, P> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        self.inner.next().map(|(t, _)| t).or_else(|| self.last.next())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(), Some(self.len()))
    }
}

impl<'a, T, P> DoubleEndedIterator for PrivateIter<'a, T, P> {
    fn next_back(&mut self) -> Option<&'a T> {
        self.last.next().or_else(|| self.inner.next_back().map(|(t, _)| t))
    }
}

impl<'a, T, P> ExactSizeIterator for PrivateIter<'a, T, P> {
    fn len(&self) -> usize {
        self.inner.len() + self.last.len()
    }
}

impl<'a, T, P> Clone for PrivateIter<'a, T, P> {
    fn clone(&self) -> Self {
        PrivateIter {
            inner: self.inner.clone(),
            last: self.last.clone(),
        }
    }
}

impl<'a, T: 'a, I> IterTrait<'a, T> for I
where
    I: Iterator<Item = &'a T> + DoubleEndedIterator + ExactSizeIterator + Clone + 'a,
{
    fn clone_box(&self) -> Box<dyn IterTrait<'a, T> + 'a> {
        Box::new(self.clone())
    }
}

pub struct Iter<'a, T: 'a> {
    inner: Box<dyn IterTrait<'a, T> + 'a>,
}

impl<'a, T> Clone for Iter<'a, T> {
    fn clone(&self) -> Self {
        Iter {
            inner: self.inner.clone_box(),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        self.inner.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(), Some(self.len()))
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<&'a T> {
        self.inner.next_back()
    }
}

impl<'a, T> ExactSizeIterator for Iter<'a, T> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

// IterMut

trait IterMutTrait<'a, T: 'a>: DoubleEndedIterator<Item = &'a mut T> + ExactSizeIterator<Item = &'a mut T> {}

struct PrivateIterMut<'a, T: 'a, P: 'a> {
    inner: slice::IterMut<'a, (T, P)>,
    last: std::option::IntoIter<&'a mut T>,
}

impl<'a, T, P> Iterator for PrivateIterMut<'a, T, P> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<&'a mut T> {
        self.inner.next().map(|(t, _)| t).or_else(|| self.last.next())
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(), Some(self.len()))
    }
}

impl<'a, T, P> DoubleEndedIterator for PrivateIterMut<'a, T, P> {
    fn next_back(&mut self) -> Option<&'a mut T> {
        self.last.next().or_else(|| self.inner.next_back().map(|(t, _)| t))
    }
}

impl<'a, T, P> ExactSizeIterator for PrivateIterMut<'a, T, P> {
    fn len(&self) -> usize {
        self.inner.len() + self.last.len()
    }
}

impl<'a, T: 'a, I> IterMutTrait<'a, T> for I where
    I: DoubleEndedIterator<Item = &'a mut T> + ExactSizeIterator<Item = &'a mut T> + 'a
{
}

pub struct IterMut<'a, T: 'a> {
    inner: Box<dyn IterMutTrait<'a, T> + 'a>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<&'a mut T> {
        self.inner.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(), Some(self.len()))
    }
}

impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    fn next_back(&mut self) -> Option<&'a mut T> {
        self.inner.next_back()
    }
}

impl<'a, T> ExactSizeIterator for IterMut<'a, T> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

// Pairs

pub struct Pairs<'a, T: 'a, P: 'a> {
    inner: slice::Iter<'a, (T, P)>,
    last: std::option::IntoIter<&'a T>,
}

impl<'a, T, P> Iterator for Pairs<'a, T, P> {
    type Item = Pair<&'a T, &'a P>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .next()
            .map(|(t, p)| Pair::Punctuated(t, p))
            .or_else(|| self.last.next().map(Pair::End))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(), Some(self.len()))
    }
}

impl<'a, T, P> DoubleEndedIterator for Pairs<'a, T, P> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.last
            .next()
            .map(Pair::End)
            .or_else(|| self.inner.next_back().map(|(t, p)| Pair::Punctuated(t, p)))
    }
}

impl<'a, T, P> ExactSizeIterator for Pairs<'a, T, P> {
    fn len(&self) -> usize {
        self.inner.len() + self.last.len()
    }
}

impl<'a, T, P> Clone for Pairs<'a, T, P> {
    fn clone(&self) -> Self {
        Pairs {
            inner: self.inner.clone(),
            last: self.last.clone(),
        }
    }
}

// PairsMut

pub struct PairsMut<'a, T: 'a, P: 'a> {
    inner: slice::IterMut<'a, (T, P)>,
    last: std::option::IntoIter<&'a mut T>,
}

impl<'a, T, P> Iterator for PairsMut<'a, T, P> {
    type Item = Pair<&'a mut T, &'a mut P>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .next()
            .map(|(t, p)| Pair::Punctuated(t, p))
            .or_else(|| self.last.next().map(Pair::End))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(), Some(self.len()))
    }
}

impl<'a, T, P> DoubleEndedIterator for PairsMut<'a, T, P> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.last
            .next()
            .map(Pair::End)
            .or_else(|| self.inner.next_back().map(|(t, p)| Pair::Punctuated(t, p)))
    }
}

impl<'a, T, P> ExactSizeIterator for PairsMut<'a, T, P> {
    fn len(&self) -> usize {
        self.inner.len() + self.last.len()
    }
}

// IntoPairs

pub struct IntoPairs<T, P> {
    inner: vec::IntoIter<(T, P)>,
    last: std::option::IntoIter<T>,
}

impl<T, P> Iterator for IntoPairs<T, P> {
    type Item = Pair<T, P>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .next()
            .map(|(t, p)| Pair::Punctuated(t, p))
            .or_else(|| self.last.next().map(Pair::End))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(), Some(self.len()))
    }
}

impl<T, P> DoubleEndedIterator for IntoPairs<T, P> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.last
            .next()
            .map(Pair::End)
            .or_else(|| self.inner.next_back().map(|(t, p)| Pair::Punctuated(t, p)))
    }
}

impl<T, P> ExactSizeIterator for IntoPairs<T, P> {
    fn len(&self) -> usize {
        self.inner.len() + self.last.len()
    }
}

impl<T: Clone, P: Clone> Clone for IntoPairs<T, P> {
    fn clone(&self) -> Self {
        IntoPairs {
            inner: self.inner.clone(),
            last: self.last.clone(),
        }
    }
}

#[cfg(feature = "serde")]
impl<T: serde::Serialize, P> serde::Serialize for Punctuated<T, P> {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeSeq;
        let mut seq = s.serialize_seq(Some(self.len()))?;
        for item in self.iter() {
            seq.serialize_element(item)?;
        }
        seq.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Span;
    use crate::token::Ident;
    use crate::token::punct::Comma;

    fn parse_stream(src: &str) -> crate::TokenStream {
        src.parse().unwrap()
    }

    #[test]
    fn parse_terminated_basic() {
        let ts = parse_stream("a , b , c");
        let mut ps = ts.parse();
        let p: Punctuated<Ident, Comma> = Punctuated::parse_terminated(&mut ps).unwrap();
        assert_eq!(p.len(), 3);
        assert!(!p.trailing_punct());
    }

    #[test]
    fn parse_terminated_trailing() {
        let ts = parse_stream("a , b , c ,");
        let mut ps = ts.parse();
        let p: Punctuated<Ident, Comma> = Punctuated::parse_terminated(&mut ps).unwrap();
        assert_eq!(p.len(), 3);
        assert!(p.trailing_punct());
    }

    #[test]
    fn parse_terminated_empty() {
        let ts = parse_stream("");
        let mut ps = ts.parse();
        let p: Punctuated<Ident, Comma> = Punctuated::parse_terminated(&mut ps).unwrap();
        assert!(p.is_empty());
    }

    #[test]
    fn parse_separated_nonempty_stops_early() {
        // "a , b c" — stops after "b" when next token is ident not punct
        let ts = parse_stream("a , b c");
        let mut ps = ts.parse();
        let p: Punctuated<Ident, Comma> = Punctuated::parse_separated_nonempty(&mut ps).unwrap();
        assert_eq!(p.len(), 2);
        // "c" should remain in stream
        assert!(!ps.is_empty());
    }

    #[test]
    fn to_tokens_roundtrip() {
        use crate::token::ToTokenStream;

        let ts = parse_stream("a , b , c");
        let mut ps = ts.parse();
        let p: Punctuated<Ident, Comma> = Punctuated::parse_terminated(&mut ps).unwrap();
        let out = p.to_token_stream();
        // TokenStream::to_string compacts whitespace; verify token count and structure
        assert_eq!(out.len(), 5); // a , b , c
        assert_eq!(p.len(), 3);
    }

    #[test]
    fn len_is_empty_trailing_punct() {
        let mut p: Punctuated<Ident, Comma> = Punctuated::new();
        assert!(p.is_empty());
        assert_eq!(p.len(), 0);
        p.push_value(Ident::new("a", Span::default()));
        assert_eq!(p.len(), 1);
        assert!(!p.trailing_punct());
        p.push_punct(Comma::default());
        assert_eq!(p.len(), 1);
        assert!(p.trailing_punct());
        p.push_value(Ident::new("b", Span::default()));
        assert_eq!(p.len(), 2);
        assert!(!p.trailing_punct());
    }

    #[test]
    fn iter_yields_values_in_order() {
        let ts = parse_stream("a , b , c");
        let mut ps = ts.parse();
        let p: Punctuated<Ident, Comma> = Punctuated::parse_terminated(&mut ps).unwrap();
        let names: Vec<_> = p.iter().map(|id| id.name().as_ref().to_owned()).collect();
        assert_eq!(names, vec!["a", "b", "c"]);
    }

    #[test]
    fn pairs_yields_correct_variants() {
        let ts = parse_stream("a , b , c");
        let mut ps = ts.parse();
        let p: Punctuated<Ident, Comma> = Punctuated::parse_terminated(&mut ps).unwrap();
        let pairs: Vec<_> = p.pairs().collect();
        assert_eq!(pairs.len(), 3);
        assert!(matches!(pairs[0], Pair::Punctuated(_, _)));
        assert!(matches!(pairs[1], Pair::Punctuated(_, _)));
        assert!(matches!(pairs[2], Pair::End(_)));
    }

    #[test]
    fn pop_and_pop_punct() {
        let mut p: Punctuated<Ident, Comma> = Punctuated::new();
        p.push_value(Ident::new("a", Span::default()));
        p.push_punct(Comma::default());
        p.push_value(Ident::new("b", Span::default()));

        // pop trailing value
        let last = p.pop().unwrap();
        assert!(matches!(last, Pair::End(_)));
        assert_eq!(p.len(), 1);
        assert!(p.trailing_punct());

        // pop trailing punct
        let punct = p.pop_punct().unwrap();
        assert_eq!(punct.as_str(), ",");
        assert!(!p.trailing_punct());
        assert_eq!(p.len(), 1);
    }

    #[test]
    fn index() {
        let ts = parse_stream("a , b , c");
        let mut ps = ts.parse();
        let p: Punctuated<Ident, Comma> = Punctuated::parse_terminated(&mut ps).unwrap();
        assert_eq!(p[0].name().as_ref(), "a");
        assert_eq!(p[1].name().as_ref(), "b");
        assert_eq!(p[2].name().as_ref(), "c");
    }
}
