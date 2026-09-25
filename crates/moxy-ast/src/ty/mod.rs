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

use moxy_token::{Delim, Group, Ident, Keyword, Punct, Span, Spanner, ToTokens, TokenStream, TokenTree};

use crate::*;

/// A Rust type expression. Covers all positions where a type can appear in source code.
#[derive(Clone)]
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

impl Type {
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

impl Parse for Type {
    fn peek(cursor: Cursor<'_>) -> bool {
        match cursor.curr() {
            Some(TokenTree::Ident(_)) => true,
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
            Some(TokenTree::Punct(p)) => matches!(
                p,
                Punct::And(_) | Punct::Star(_) | Punct::Not(_) | Punct::Colon(_) | Punct::Lt(_)
            ),
            _ => false,
        }
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        if !Self::peek(parser.cursor()) {
            return parser.error("expected type").into();
        }

        // `&` reference.
        if <Token![&]>::peek(parser.cursor()) {
            return Ok(Self::Reference(<_ as Parse>::parse(parser)?));
        }

        // `*` raw pointer.
        if <Token![*]>::peek(parser.cursor()) {
            return Ok(Self::Pointer(<_ as Parse>::parse(parser)?));
        }

        // Never `!`.
        if <Token![!]>::peek(parser.cursor()) {
            return Ok(Self::Never(<_ as Parse>::parse(parser)?));
        }

        // Infer `_`.
        if matches!(parser.curr(), Some(tt) if tt.text() == Some("_")) {
            let span = parser.span();
            parser.advance();
            return Ok(Self::Infer(moxy_token::Ident::new("_").with_span(span)));
        }

        // `[T]` slice or `[T; N]` array — decided by a `;` inside the brackets.
        // Both share the same `[` token so we disambiguate inline after peeking
        // inside the group rather than calling `TypeArray::parse` or
        // `TypeSlice::parse` individually (which would each consume the group).
        if matches!(parser.curr(), Some(tt) if tt.delim() == Some(Delim::Bracket)) {
            let (bracket_span, inner) = parser.parse_group_spanned(Delim::Bracket)?;
            let elem = Box::new(<_ as Parse>::parse(&inner)?);

            if <Token![;]>::peek(inner.cursor()) {
                let semi = <_ as Parse>::parse(&inner)?;
                let len = <_ as Parse>::parse(&inner)?;

                if !inner.is_empty() {
                    return inner.error("unexpected trailing input").into();
                }

                return Ok(Self::Array(TypeArray {
                    content: Delimited::bracket(bracket_span, type_array::ArrayInner { elem, semi, len }),
                }));
            }

            if !inner.is_empty() {
                return inner.error("unexpected trailing input").into();
            }

            let elem = Delimited::bracket(bracket_span, elem);
            return Ok(Self::Slice(TypeSlice { elem }));
        }

        // `impl Trait`.
        if <Token![impl]>::peek(parser.cursor()) {
            return Ok(Self::ImplTrait(<_ as Parse>::parse(parser)?));
        }

        // `dyn Trait` and the legacy bare multi-bound trait-object form.
        if TypeTraitObject::peek(parser.cursor()) {
            return Ok(Self::TraitObject(<_ as Parse>::parse(parser)?));
        }

        // Bare fn pointer: `fn(...)`, `extern "C" fn(...)`, `unsafe fn(...)`.
        if <Token![fn]>::peek(parser.cursor())
            || <Token![extern]>::peek(parser.cursor())
            || <Token![unsafe]>::peek(parser.cursor())
            || BoundLifetimes::peek(parser.cursor())
        {
            return Ok(Self::BareFn(<_ as Parse>::parse(parser)?));
        }

        // `(...)` — one element with no trailing comma is a parenthesized type;
        // anything else (empty, multiple, or trailing comma) is a tuple.
        // Both variants share the same `(` token so we disambiguate inline.
        if matches!(parser.curr(), Some(tt) if tt.delim() == Some(Delim::Paren)) {
            let (paren_span, inner) = parser.parse_group_spanned(Delim::Paren)?;
            let elems: Punctuated<Self, Token![,]> = Punctuated::parse_terminated(&inner)?;

            return if elems.len() == 1 && !elems.is_trailing() {
                let content = Delimited::paren(paren_span, Box::new(elems.into_iter().next().unwrap()));
                Ok(Self::Paren(TypeParen { content }))
            } else {
                let elems_del = Delimited::paren(paren_span, elems);
                Ok(Self::Tuple(TypeTuple { elems: elems_del }))
            };
        }

        if matches!(parser.curr(), Some(tt) if tt.delim() == Some(Delim::None)) {
            let (span, inner) = parser.parse_group_spanned(Delim::None)?;
            return Ok(Self::Group(TypeGroup {
                span: span.span(),
                elem: Box::new(<_ as Parse>::parse(&inner)?),
            }));
        }

        // Otherwise a path type: `T`, `std::vec::Vec`, or a qualified
        // `<T as Trait>::Item` (which begins with `<`).
        if <Token![<]>::peek(parser.cursor()) {
            return Ok(Self::Path(<_ as Parse>::parse(parser)?));
        }

        let path = <_ as Parse>::parse(parser)?;

        if <Token![!]>::peek(parser.cursor()) {
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

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
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

        if TypeTraitObject::peek(cursor) {
            return TypeTraitObject::skip(cursor);
        }

        if <Token![fn]>::peek(cursor)
            || <Token![extern]>::peek(cursor)
            || <Token![unsafe]>::peek(cursor)
            || BoundLifetimes::peek(cursor)
        {
            return TypeBareFn::skip(cursor);
        }

        if cursor.is_delimited(Delim::Paren) {
            if TypeParen::peek(cursor) {
                return TypeParen::skip(cursor);
            }

            return TypeTuple::skip(cursor);
        }

        if cursor.is_delimited(Delim::None) {
            return TypeGroup::skip(cursor);
        }

        if <Token![<]>::peek(cursor) {
            return TypePath::skip(cursor);
        }

        let mut cursor = Path::skip(cursor)?;

        if <Token![!]>::peek(cursor) {
            cursor = <Token![!]>::skip(cursor)?;
            cursor = Group::skip(cursor)?;
        }

        Some(cursor)
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
            // `Group` is only produced via the proc-macro bridge, never `from_str`.
            Self::Group(_) => {}
        }
    }
}
