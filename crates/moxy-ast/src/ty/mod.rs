mod q_self;
mod type_array;
mod type_bare_fn;
mod type_group;
mod type_impl_trait;
mod type_macro;
mod type_paren;
mod type_path;
mod type_pointer;
mod type_reference;
mod type_slice;
mod type_trait_object;
mod type_tuple;

pub use q_self::*;
pub use type_array::*;
pub use type_bare_fn::*;
pub use type_group::*;
pub use type_impl_trait::*;
pub use type_macro::*;
pub use type_paren::*;
pub use type_path::*;
pub use type_pointer::*;
pub use type_reference::*;
pub use type_slice::*;
pub use type_trait_object::*;
pub use type_tuple::*;

use moxy_token::{Delim, Group, Ident, Keyword, Punct, Span, Spanner, ToTokenStream, ToTokens, TokenStream, TokenTree};

use crate::*;

/// A Rust type expression. Covers all positions where a type can appear in source code.
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[non_exhaustive]
pub enum Type {
    Never(Token![!]),
    Infer(Ident),
    Path(TypePath),
    Tuple(TypeTuple),
    Array(TypeArray),
    Slice(TypeSlice),
    Reference(TypeReference),
    Pointer(TypePointer),
    BareFn(TypeBareFn),
    ImplTrait(TypeImplTrait),
    TraitObject(TypeTraitObject),
    Paren(TypeParen),
    Group(TypeGroup),
    Macro(TypeMacro),
}

clone_in_small_frames!(Type {
    Never,
    Infer,
    Path,
    Tuple,
    Array,
    Slice,
    Reference,
    Pointer,
    BareFn,
    ImplTrait,
    TraitObject,
    Paren,
    Group,
    Macro
});

impl Type {
    pub(crate) fn to_tokens_without_plus(&self, tokens: &mut TokenStream) {
        let mut ty = self;
        while let Self::Group(group) = ty {
            ty = &group.elem;
        }

        let bounds = match ty {
            Self::TraitObject(object) => Some(&object.bounds),
            Self::ImplTrait(opaque) => Some(&opaque.bounds),
            _ => None,
        };

        // Rust treats a reconstructed invisible delimiter as transparent, so
        // `&$ty` with `dyn Trait + Send` needs visible parentheses on emission.
        if bounds.is_some_and(|bounds| bounds.len() > 1 || bounds.is_trailing()) {
            tokens.extend_one(Group::new(Delim::Paren, self.to_token_stream()).into_token_tree());
        } else {
            self.to_tokens(tokens);
        }
    }

    pub fn is_never(&self) -> bool {
        matches!(self, Self::Never(_))
    }

    pub fn is_infer(&self) -> bool {
        matches!(self, Self::Infer(_))
    }

    pub fn is_path(&self) -> bool {
        matches!(self, Self::Path(_))
    }

    pub fn is_tuple(&self) -> bool {
        matches!(self, Self::Tuple(_))
    }

    pub fn is_array(&self) -> bool {
        matches!(self, Self::Array(_))
    }

    pub fn is_slice(&self) -> bool {
        matches!(self, Self::Slice(_))
    }

    pub fn is_reference(&self) -> bool {
        matches!(self, Self::Reference(_))
    }

    pub fn is_pointer(&self) -> bool {
        matches!(self, Self::Pointer(_))
    }

    pub fn is_bare_fn(&self) -> bool {
        matches!(self, Self::BareFn(_))
    }

    pub fn is_impl_trait(&self) -> bool {
        matches!(self, Self::ImplTrait(_))
    }

    pub fn is_trait_object(&self) -> bool {
        matches!(self, Self::TraitObject(_))
    }

    pub fn is_paren(&self) -> bool {
        matches!(self, Self::Paren(_))
    }

    pub fn is_group(&self) -> bool {
        matches!(self, Self::Group(_))
    }

    pub fn is_macro(&self) -> bool {
        matches!(self, Self::Macro(_))
    }

    pub fn as_never(&self) -> Option<&Token![!]> {
        if let Self::Never(v) = self { Some(v) } else { None }
    }

    pub fn as_infer(&self) -> Option<&Ident> {
        if let Self::Infer(v) = self { Some(v) } else { None }
    }

    pub fn as_path(&self) -> Option<&TypePath> {
        if let Self::Path(v) = self { Some(v) } else { None }
    }

    pub fn as_tuple(&self) -> Option<&TypeTuple> {
        if let Self::Tuple(v) = self { Some(v) } else { None }
    }

    pub fn as_array(&self) -> Option<&TypeArray> {
        if let Self::Array(v) = self { Some(v) } else { None }
    }

    pub fn as_slice(&self) -> Option<&TypeSlice> {
        if let Self::Slice(v) = self { Some(v) } else { None }
    }

    pub fn as_reference(&self) -> Option<&TypeReference> {
        if let Self::Reference(v) = self { Some(v) } else { None }
    }

    pub fn as_pointer(&self) -> Option<&TypePointer> {
        if let Self::Pointer(v) = self { Some(v) } else { None }
    }

    pub fn as_bare_fn(&self) -> Option<&TypeBareFn> {
        if let Self::BareFn(v) = self { Some(v) } else { None }
    }

    pub fn as_impl_trait(&self) -> Option<&TypeImplTrait> {
        if let Self::ImplTrait(v) = self { Some(v) } else { None }
    }

    pub fn as_trait_object(&self) -> Option<&TypeTraitObject> {
        if let Self::TraitObject(v) = self { Some(v) } else { None }
    }

    pub fn as_paren(&self) -> Option<&TypeParen> {
        if let Self::Paren(v) = self { Some(v) } else { None }
    }

    pub fn as_group(&self) -> Option<&TypeGroup> {
        if let Self::Group(v) = self { Some(v) } else { None }
    }

    pub fn as_macro(&self) -> Option<&TypeMacro> {
        if let Self::Macro(v) = self { Some(v) } else { None }
    }
}

impl Spanner for Type {
    fn span(&self) -> Span {
        match self {
            Self::Never(not) => not.span(),
            Self::Infer(id) => id.span(),
            Self::Path(v) => v.span(),
            Self::Tuple(v) => v.span(),
            Self::Array(v) => v.span(),
            Self::Slice(v) => v.span(),
            Self::Reference(v) => v.span(),
            Self::Pointer(v) => v.span(),
            Self::BareFn(v) => v.span(),
            Self::ImplTrait(v) => v.span(),
            Self::TraitObject(v) => v.span(),
            Self::Paren(v) => v.span(),
            Self::Group(v) => v.span(),
            Self::Macro(v) => v.span(),
        }
    }
}

impl From<TypePath> for Type {
    fn from(value: TypePath) -> Self {
        Self::Path(value)
    }
}

impl From<TypeReference> for Type {
    fn from(value: TypeReference) -> Self {
        Self::Reference(value)
    }
}

impl From<TypePointer> for Type {
    fn from(value: TypePointer) -> Self {
        Self::Pointer(value)
    }
}

impl From<TypeTuple> for Type {
    fn from(value: TypeTuple) -> Self {
        Self::Tuple(value)
    }
}

impl From<TypeParen> for Type {
    fn from(value: TypeParen) -> Self {
        Self::Paren(value)
    }
}

impl From<TypeSlice> for Type {
    fn from(value: TypeSlice) -> Self {
        Self::Slice(value)
    }
}

impl From<TypeImplTrait> for Type {
    fn from(value: TypeImplTrait) -> Self {
        Self::ImplTrait(value)
    }
}

impl From<TypeTraitObject> for Type {
    fn from(value: TypeTraitObject) -> Self {
        Self::TraitObject(value)
    }
}

impl From<TypeBareFn> for Type {
    fn from(value: TypeBareFn) -> Self {
        Self::BareFn(value)
    }
}

impl Type {
    /// Parses a type that `+` does not continue, as after `as` in an
    /// expression, where `a as usize + 1` adds to the cast, and after `&` or
    /// `*`, where Rust rejects `&A + B` and `&dyn A + B` as ambiguous.
    pub(crate) fn parse_without_plus(parser: &Parser) -> Result<Self, ParseError> {
        let _depth = moxy_token::__private::Depth::enter().ok_or_else(|| ParseError::too_deep(parser.span()))?;

        if Self::peek_plain_path(parser.cursor()) {
            return Self::parse_path_or_macro(parser, false);
        }

        // `impl` and `dyn` take one bound here, as in `&dyn A` and `Fn() -> impl A`
        if <Token![impl]>::peek(parser.cursor()) {
            // The second level of an `impl` type, as in `TypeImplTrait`
            let _weight = moxy_token::__private::Depth::enter().ok_or_else(|| ParseError::too_deep(parser.span()))?;
            let impl_keyword = <_ as Parse>::parse(parser)?;
            let mut bounds = List::new();

            if TypeBound::peek(parser.cursor()) {
                bounds.push_value(<_ as Parse>::parse(parser)?);
            }

            return Ok(Self::ImplTrait(TypeImplTrait { impl_keyword, bounds }));
        }

        if <Token![dyn]>::peek(parser.cursor()) {
            let dyn_token = <_ as Parse>::parse(parser)?;
            let mut bounds = List::new();

            if TypeBound::peek(parser.cursor()) {
                bounds.push_value(<_ as Parse>::parse(parser)?);
            }

            return Ok(Self::TraitObject(TypeTraitObject { dyn_token, bounds }));
        }

        // Parentheses here hold a type, never the first bound of a trait object
        if parser.is_delimited(Delim::Paren) {
            return Self::parse_paren_or_tuple(parser);
        }

        <Self as Parse>::parse(parser)
    }

    pub(crate) fn skip_without_plus(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        let _depth = moxy_token::__private::Depth::enter()?;

        if Self::peek_plain_path(cursor) {
            return Self::skip_path_or_macro(cursor, false);
        }

        if <Token![impl]>::peek(cursor) || <Token![dyn]>::peek(cursor) {
            let _weight = if <Token![impl]>::peek(cursor) {
                Some(moxy_token::__private::Depth::enter()?)
            } else {
                None
            };
            let cursor = cursor.offset(1);
            return if TypeBound::peek(cursor) {
                TypeBound::skip(cursor)
            } else {
                Some(cursor)
            };
        }

        if cursor.is_delimited(Delim::Paren) {
            let inner = List::<Type, Token![,]>::skip_while_present(cursor.descend(Delim::Paren)?)?;
            return inner.is_empty().then(|| cursor.offset(1));
        }

        <Self as Parse>::skip(cursor)
    }

    // A path that is not qualified with `<..>` and starts no other form
    fn peek_plain_path(cursor: Cursor<'_>) -> bool {
        !<Token![dyn]>::peek(cursor)
            && !<Token![impl]>::peek(cursor)
            && !<Token![fn]>::peek(cursor)
            && !<Token![unsafe]>::peek(cursor)
            && !<Token![extern]>::peek(cursor)
            && Path::peek(cursor)
    }

    // A trait object that does not start with a path: `dyn A`, or a bound with
    // a binder as in `for<'a> A<'a> + B`. One that starts with a path is told
    // apart after the path, by the `+` that follows it, so the path is read
    // once.
    fn peek_trait_object(cursor: Cursor<'_>) -> bool {
        if <Token![dyn]>::peek(cursor) {
            return true;
        }

        // A bare trait object may start with a lifetime bound, as in `'a + A`
        if Self::peek_lifetime_bound(cursor) {
            return true;
        }

        <Token![for]>::peek(cursor)
            && BoundLifetimes::skip(cursor).is_some_and(|after| {
                !<Token![fn]>::peek(after) && !<Token![unsafe]>::peek(after) && !<Token![extern]>::peek(after)
            })
    }

    // A bare trait object whose first bound is in parentheses, as in
    // `(?Sized) + Copy`, which rustc's parser accepts where `+` may follow.
    // Parentheses that a `+` follows can hold only a bound there, so the
    // group is read once, as the bound
    pub(crate) fn peek_paren_bound_object(cursor: Cursor<'_>) -> bool {
        cursor.is_delimited(Delim::Paren) && <Token![+]>::peek(cursor.offset(1))
    }

    pub(crate) fn peek_lifetime_bound(cursor: Cursor<'_>) -> bool {
        Lifetime::skip(cursor).is_some_and(|after| <Token![+]>::peek(after))
    }

    fn parse_never(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self::Never(<_ as Parse>::parse(parser)?))
    }

    // A bare trait object whose first bound is `path`, at the `+` after it
    pub(crate) fn parse_bare_trait_object(parser: &Parser, path: Path) -> Result<Self, ParseError> {
        let mut bounds = List::new();
        bounds.push_value(TypeBound::Trait(TraitBound {
            paren: None,
            polarity: None,
            lifetimes: None,
            modifier: None,
            path,
        }));

        while <Token![+]>::peek(parser.cursor()) {
            bounds.push_punct(<_ as Parse>::parse(parser)?);

            if !TypeBound::peek(parser.cursor()) {
                break;
            }

            bounds.push_value(<_ as Parse>::parse(parser)?);
        }

        Ok(Self::TraitObject(TypeTraitObject { dyn_token: None, bounds }))
    }

    fn skip_path_or_macro(cursor: Cursor<'_>, plus: bool) -> Option<Cursor<'_>> {
        if <Token![<]>::peek(cursor) {
            return TypePath::skip(cursor);
        }

        let mut cursor = Path::skip(cursor)?;

        if plus && <Token![+]>::peek(cursor) {
            return TypeBound::skip_bounds_after(cursor);
        }

        if <Token![!]>::peek(cursor) && !<Token![!=]>::peek(cursor) {
            cursor = <Token![!]>::skip(cursor)?;
            cursor = Group::skip(cursor)?;
        }

        Some(cursor)
    }

    fn parse_infer(parser: &Parser) -> Result<Self, ParseError> {
        let span = parser.span();
        parser.advance();
        Ok(Self::Infer(moxy_token::Ident::new("_").with_span(span)))
    }

    // `[T]` slice or `[T; N]` array, told apart by a `;` inside the brackets
    // An array type takes another level of stack for its length, as in
    // `[u8; { let x: [u8; ..]; 1 }]`
    fn parse_slice_or_array(parser: &Parser) -> Result<Self, ParseError> {
        let _weight = moxy_token::__private::Depth::enter().ok_or_else(|| ParseError::too_deep(parser.span()))?;
        let (bracket_span, inner) = parser.parse_group_spanned(Delim::Bracket)?;
        let elem = Box::new(<_ as Parse>::parse(&inner)?);

        if <Token![;]>::peek(inner.cursor()) {
            let semi = <_ as Parse>::parse(&inner)?;
            let len = <_ as Parse>::parse(&inner)?;

            if !inner.is_empty() {
                return inner.unexpected().into();
            }

            return Ok(Self::Array(TypeArray {
                content: Delimited::bracket(bracket_span, type_array::ArrayInner { elem, semi, len }),
            }));
        }

        if !inner.is_empty() {
            return inner.unexpected().into();
        }

        let elem = Delimited::bracket(bracket_span, elem);
        Ok(Self::Slice(TypeSlice { elem }))
    }

    // One element with no trailing comma is a parenthesized type; anything
    // else, empty, several, or with a trailing comma, is a tuple
    fn parse_paren_or_tuple(parser: &Parser) -> Result<Self, ParseError> {
        let (paren_span, inner) = parser.parse_group_spanned(Delim::Paren)?;
        let elems: List<Self, Token![,]> = List::parse_all(&inner)?;

        if elems.len() == 1 && !elems.is_trailing() {
            let content = Delimited::paren(paren_span, Box::new(elems.into_iter().next().unwrap()));
            Ok(Self::Paren(TypeParen { content }))
        } else {
            let elems_del = Delimited::paren(paren_span, elems);
            Ok(Self::Tuple(TypeTuple { elems: elems_del }))
        }
    }

    fn parse_group(parser: &Parser) -> Result<Self, ParseError> {
        let (span, inner) = parser.parse_group_spanned(Delim::None)?;

        Ok(Self::Group(TypeGroup {
            span: span.span(),
            elem: Box::new(<_ as Parse>::parse(&inner)?),
        }))
    }

    // A path type, `T`, `std::vec::Vec`, or a qualified `<T as Trait>::Item`,
    // or a type macro
    // With `plus`, a `+` after the path makes a bare trait object of it, as in
    // `Box<A + Send>`
    fn parse_path_or_macro(parser: &Parser, plus: bool) -> Result<Self, ParseError> {
        if <Token![<]>::peek(parser.cursor()) {
            return Ok(Self::Path(<_ as Parse>::parse(parser)?));
        }

        let path: Path = <_ as Parse>::parse(parser)?;

        if plus && <Token![+]>::peek(parser.cursor()) {
            return Self::parse_bare_trait_object(parser, path);
        }

        // `!=` after a type ends a cast (`a as u8 != b`), not a type macro
        if <Token![!]>::peek(parser.cursor()) && !<Token![!=]>::peek(parser.cursor()) {
            return Ok(Self::Macro(TypeMacro {
                mac: MacroCall {
                    path,
                    bang: <_ as Parse>::parse(parser)?,
                    body: <_ as Parse>::parse(parser)?,
                },
            }));
        }

        Ok(Self::Path(TypePath { qself: None, path }))
    }
}

impl Parse for Type {
    fn peek(cursor: Cursor<'_>) -> bool {
        // A weak keyword such as `raw` or `default` can name a type or begin its path
        if Ident::peek(cursor) {
            return true;
        }

        match cursor.curr() {
            Some(TokenTree::Group(g)) => matches!(g.delim, Delim::Paren | Delim::Bracket | Delim::None),
            Some(TokenTree::Keyword(k)) => matches!(
                k,
                Keyword::Impl(_)
                    | Keyword::Dyn(_)
                    | Keyword::Fn(_)
                    | Keyword::Extern(_)
                    | Keyword::For(_)
                    | Keyword::Unsafe(_)
                    | Keyword::SelfType(_)
                    | Keyword::SelfValue(_)
                    | Keyword::Super(_)
                    | Keyword::Crate(_)
            ),
            Some(TokenTree::Punct(Punct::Quote(_))) => Self::peek_lifetime_bound(cursor),
            Some(TokenTree::Punct(p)) => matches!(
                p,
                Punct::And(_) | Punct::Star(_) | Punct::Not(_) | Punct::Colon(_) | Punct::Lt(_)
            ),
            _ => false,
        }
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        if !Self::peek(parser.cursor()) {
            return parser.expected("type").into();
        }

        let _depth = moxy_token::__private::Depth::enter().ok_or_else(|| ParseError::too_deep(parser.span()))?;

        // One call site for every form keeps one return slot in this frame
        let parse: fn(&Parser) -> Result<Self, ParseError> = if <Token![&]>::peek(parser.cursor()) {
            parse_into::<TypeReference, Self>
        } else if <Token![*]>::peek(parser.cursor()) {
            parse_into::<TypePointer, Self>
        } else if <Token![!]>::peek(parser.cursor()) {
            Self::parse_never
        } else if matches!(parser.curr(), Some(tt) if tt.text() == Some("_")) {
            Self::parse_infer
        } else if parser.is_delimited(Delim::Bracket) {
            Self::parse_slice_or_array
        } else if <Token![impl]>::peek(parser.cursor()) {
            parse_into::<TypeImplTrait, Self>
        } else if Self::peek_trait_object(parser.cursor()) || Self::peek_paren_bound_object(parser.cursor()) {
            // `dyn Trait` and the legacy bare multi-bound trait-object form
            parse_into::<TypeTraitObject, Self>
        } else if <Token![fn]>::peek(parser.cursor())
            || <Token![extern]>::peek(parser.cursor())
            || <Token![unsafe]>::peek(parser.cursor())
            || BoundLifetimes::peek(parser.cursor())
        {
            parse_into::<TypeBareFn, Self>
        } else if parser.is_delimited(Delim::Paren) {
            Self::parse_paren_or_tuple
        } else if parser.is_delimited(Delim::None) {
            Self::parse_group
        } else {
            |parser| Self::parse_path_or_macro(parser, true)
        };

        parse(parser)
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        let _depth = moxy_token::__private::Depth::enter()?;

        // Only what starts a type for the parser starts one here
        if !Self::peek(cursor) {
            return None;
        }

        if <Token![&]>::peek(cursor) {
            return TypeReference::skip(cursor);
        }

        if <Token![*]>::peek(cursor) {
            return TypePointer::skip(cursor);
        }

        if <Token![!]>::peek(cursor) {
            return <Token![!]>::skip(cursor);
        }

        if matches!(cursor.curr(), Some(tt) if tt.text() == Some("_")) {
            return Some(cursor.offset(1));
        }

        if cursor.is_delimited(Delim::Bracket) {
            let _weight = moxy_token::__private::Depth::enter()?;
            let mut inner = cursor.descend(Delim::Bracket)?;
            inner = Type::skip(inner)?;

            if <Token![;]>::peek(inner) {
                inner = <Token![;]>::skip(inner)?;
                inner = Expr::skip(inner)?;
            }

            return inner.is_empty().then(|| cursor.offset(1));
        }

        if <Token![impl]>::peek(cursor) {
            return TypeImplTrait::skip(cursor);
        }

        if Self::peek_trait_object(cursor) {
            return TypeTraitObject::skip(cursor);
        }

        if <Token![fn]>::peek(cursor)
            || <Token![extern]>::peek(cursor)
            || <Token![unsafe]>::peek(cursor)
            || BoundLifetimes::peek(cursor)
        {
            return TypeBareFn::skip(cursor);
        }

        if Self::peek_paren_bound_object(cursor) {
            return TypeTraitObject::skip(cursor);
        }

        // A parenthesized type and a tuple skip alike, each element once, so
        // nested parentheses skip in linear time
        if cursor.is_delimited(Delim::Paren) {
            let inner = List::<Type, Token![,]>::skip_while_present(cursor.descend(Delim::Paren)?)?;
            return inner.is_empty().then(|| cursor.offset(1));
        }

        if cursor.is_delimited(Delim::None) {
            return TypeGroup::skip(cursor);
        }

        Self::skip_path_or_macro(cursor, true)
    }
}

impl ToTokens for Type {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Path(value) => value.to_tokens(tokens),
            Self::Reference(value) => value.to_tokens(tokens),
            Self::Pointer(value) => value.to_tokens(tokens),
            Self::Tuple(value) => value.to_tokens(tokens),
            Self::Paren(value) => value.to_tokens(tokens),
            Self::Slice(value) => value.to_tokens(tokens),
            Self::ImplTrait(value) => value.to_tokens(tokens),
            Self::TraitObject(value) => value.to_tokens(tokens),
            Self::BareFn(value) => value.to_tokens(tokens),
            Self::Array(value) => value.to_tokens(tokens),
            Self::Macro(value) => value.to_tokens(tokens),
            Self::Never(not) => not.to_tokens(tokens),
            Self::Infer(id) => id.to_tokens(tokens),
            Self::Group(value) => value.to_tokens(tokens),
        }
    }
}
