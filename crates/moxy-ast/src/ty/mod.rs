use crate::{Parse, ParseError, Parser};
use moxy_token::{Delim, Span, Spanner, ToTokens, TokenStream};

use crate::{Delimited, Punctuated};

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

/// A Rust type expression. Covers all positions where a type can appear in source code.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Type {
    Never(Token![!]),
    Infer(moxy_token::Ident),
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

    pub fn as_infer(&self) -> Option<&moxy_token::Ident> {
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
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        // `&` reference.
        if parser.peek::<Token![&]>() {
            return Ok(Self::Reference(parser.parse()?));
        }

        // `*` raw pointer.
        if parser.peek::<Token![*]>() {
            return Ok(Self::Pointer(parser.parse()?));
        }

        // Never `!`.
        if parser.peek::<Token![!]>() {
            let not = parser.parse::<Token![!]>()?;
            return Ok(Self::Never(not));
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
            let (bracket_span, group_tokens) = parser.parse_group_spanned(Delim::Bracket)?;
            let inner = Parser::from_tokens(&group_tokens);
            let elem = Box::new(inner.parse::<Self>()?);

            if inner.peek::<Token![;]>() {
                let semi = inner.parse::<Token![;]>()?;
                let len = inner.parse::<crate::Expr>()?;
                return Ok(Self::Array(TypeArray {
                    content: Delimited::bracket(bracket_span, type_array::ArrayInner { elem, semi, len }),
                }));
            }

            {
                let elem = Delimited::bracket(bracket_span, elem);
                return Ok(Self::Slice(TypeSlice { elem }));
            }
        }

        // `impl Trait`.
        if parser.peek::<Token![impl]>() {
            return Ok(Self::ImplTrait(parser.parse()?));
        }

        // `dyn Trait`.
        if parser.peek::<Token![dyn]>() {
            return Ok(Self::TraitObject(parser.parse()?));
        }

        // Bare fn pointer: `fn(...)`, `extern "C" fn(...)`, `unsafe fn(...)`.
        if parser.peek::<Token![fn]>() || parser.peek::<Token![extern]>() || parser.peek::<Token![unsafe]>() {
            return Ok(Self::BareFn(parser.parse()?));
        }

        // `(...)` — one element with no trailing comma is a parenthesized type;
        // anything else (empty, multiple, or trailing comma) is a tuple.
        // Both variants share the same `(` token so we disambiguate inline.
        if matches!(parser.curr(), Some(tt) if tt.delim() == Some(Delim::Paren)) {
            let (paren_span, group_tokens) = parser.parse_group_spanned(Delim::Paren)?;
            let inner = Parser::from_tokens(&group_tokens);
            let elems: Punctuated<Self, Token![,]> = Punctuated::parse_terminated(&inner)?;

            return if elems.len() == 1 && !elems.is_trailing() {
                let content = Delimited::paren(paren_span, Box::new(elems.into_iter().next().unwrap()));
                Ok(Self::Paren(TypeParen { content }))
            } else {
                let elems_del = Delimited::paren(paren_span, elems);
                Ok(Self::Tuple(TypeTuple { elems: elems_del }))
            };
        }

        // Macro type `m!(...)` — a path followed by `!`.
        if let Some(mac) = parser.parse_if::<TypeMacro>() {
            return Ok(Self::Macro(mac));
        }

        // Otherwise a path type: `T`, `std::vec::Vec`, or a qualified
        // `<T as Trait>::Item` (which begins with `<`).
        Ok(Self::Path(parser.parse()?))
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
