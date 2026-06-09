use moxy_token::keyword::{Dyn, Impl};
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::{And, Comma, Star};
use moxy_token::{Delim, Parse, Span, Spanner, ToTokens, TokenStream};

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
mod typed_param;

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
pub use typed_param::*;

/// A Rust type expression. Covers all positions where a type can appear in source code.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Type {
    Never(moxy_token::punct::Not),
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

    pub fn as_never(&self) -> Option<&moxy_token::punct::Not> {
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
            Type::Never(not) => not.span(),
            Type::Infer(id) => id.span(),
            Type::Path(v) => v.span(),
            Type::Tuple(v) => v.span(),
            Type::Array(v) => v.span(),
            Type::Slice(v) => v.span(),
            Type::Reference(v) => v.span(),
            Type::Pointer(v) => v.span(),
            Type::BareFn(v) => v.span(),
            Type::ImplTrait(v) => v.span(),
            Type::TraitObject(v) => v.span(),
            Type::Paren(v) => v.span(),
            Type::Group(v) => v.span(),
            Type::Macro(v) => v.span(),
        }
    }
}

impl From<TypePath> for Type {
    fn from(value: TypePath) -> Self {
        Type::Path(value)
    }
}

impl From<TypeReference> for Type {
    fn from(value: TypeReference) -> Self {
        Type::Reference(value)
    }
}

impl From<TypePointer> for Type {
    fn from(value: TypePointer) -> Self {
        Type::Pointer(value)
    }
}

impl From<TypeTuple> for Type {
    fn from(value: TypeTuple) -> Self {
        Type::Tuple(value)
    }
}

impl From<TypeParen> for Type {
    fn from(value: TypeParen) -> Self {
        Type::Paren(value)
    }
}

impl From<TypeSlice> for Type {
    fn from(value: TypeSlice) -> Self {
        Type::Slice(value)
    }
}

impl From<TypeImplTrait> for Type {
    fn from(value: TypeImplTrait) -> Self {
        Type::ImplTrait(value)
    }
}

impl From<TypeTraitObject> for Type {
    fn from(value: TypeTraitObject) -> Self {
        Type::TraitObject(value)
    }
}

impl From<TypeBareFn> for Type {
    fn from(value: TypeBareFn) -> Self {
        Type::BareFn(value)
    }
}

impl Parse for Type {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        // `&` reference.
        if stream.peek::<And>() {
            return Ok(Type::Reference(stream.parse()?));
        }

        // `*` raw pointer.
        if stream.peek::<Star>() {
            return Ok(Type::Pointer(stream.parse()?));
        }

        // Never `!`.
        if stream.peek::<moxy_token::punct::Not>() {
            let not = stream.parse::<moxy_token::punct::Not>()?;
            return Ok(Type::Never(not));
        }

        // Infer `_`.
        if matches!(stream.curr(), Some(tt) if tt.text() == Some("_")) {
            let span = stream.span();
            stream.advance();
            return Ok(Type::Infer(moxy_token::Ident::new("_").with_span(span)));
        }

        // `[T]` slice or `[T; N]` array — decided by a `;` inside the brackets.
        // Both share the same `[` token so we disambiguate inline after peeking
        // inside the group rather than calling `TypeArray::parse` or
        // `TypeSlice::parse` individually (which would each consume the group).
        if matches!(stream.curr(), Some(tt) if tt.delim() == Some(Delim::Bracket)) {
            let (bracket_span, group_tokens) = stream.parse_group_spanned(Delim::Bracket)?;
            let mut inner = group_tokens.parse();
            let elem = Box::new(inner.parse::<Type>()?);

            if inner.peek::<moxy_token::punct::Semi>() {
                let semi = inner.parse::<moxy_token::punct::Semi>()?;
                let len = inner.parse::<crate::Expr>()?;
                return Ok(Type::Array(TypeArray {
                    content: Delimited::bracket(bracket_span, type_array::ArrayInner { elem, semi, len }),
                }));
            }

            {
                let elem = Delimited::bracket(bracket_span, elem);
                return Ok(Type::Slice(TypeSlice { elem }));
            }
        }

        // `impl Trait`.
        if stream.peek::<Impl>() {
            return Ok(Type::ImplTrait(stream.parse()?));
        }

        // `dyn Trait`.
        if stream.peek::<Dyn>() {
            return Ok(Type::TraitObject(stream.parse()?));
        }

        // Bare fn pointer: `fn(...)`, `extern "C" fn(...)`, `unsafe fn(...)`.
        if stream.peek::<moxy_token::keyword::Fn>()
            || stream.peek::<moxy_token::keyword::Extern>()
            || stream.peek::<moxy_token::keyword::Unsafe>()
        {
            return Ok(Type::BareFn(stream.parse()?));
        }

        // `(...)` — one element with no trailing comma is a parenthesized type;
        // anything else (empty, multiple, or trailing comma) is a tuple.
        // Both variants share the same `(` token so we disambiguate inline.
        if matches!(stream.curr(), Some(tt) if tt.delim() == Some(Delim::Paren)) {
            let (paren_span, group_tokens) = stream.parse_group_spanned(Delim::Paren)?;
            let mut inner = group_tokens.parse();
            let elems: Punctuated<Type, Comma> = Punctuated::parse_terminated(&mut inner)?;

            return if elems.len() == 1 && !elems.is_trailing() {
                let content = Delimited::paren(paren_span, Box::new(elems.into_iter().next().unwrap()));
                Ok(Type::Paren(TypeParen { content }))
            } else {
                let elems_del = Delimited::paren(paren_span, elems);
                Ok(Type::Tuple(TypeTuple { elems: elems_del }))
            };
        }

        // Macro type `m!(...)` — a path followed by `!`.
        if let Some(mac) = stream.parse_if::<TypeMacro>() {
            return Ok(Type::Macro(mac));
        }

        // Otherwise a path type: `T`, `std::vec::Vec`, or a qualified
        // `<T as Trait>::Item` (which begins with `<`).
        Ok(Type::Path(stream.parse()?))
    }
}

impl ToTokens for Type {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Type::Path(value) => value.to_tokens(tokens),
            Type::Reference(value) => value.to_tokens(tokens),
            Type::Pointer(value) => value.to_tokens(tokens),
            Type::Tuple(value) => value.to_tokens(tokens),
            Type::Paren(value) => value.to_tokens(tokens),
            Type::Slice(value) => value.to_tokens(tokens),
            Type::ImplTrait(value) => value.to_tokens(tokens),
            Type::TraitObject(value) => value.to_tokens(tokens),
            Type::BareFn(value) => value.to_tokens(tokens),
            Type::Array(value) => value.to_tokens(tokens),
            Type::Macro(value) => value.to_tokens(tokens),
            Type::Never(not) => not.to_tokens(tokens),
            Type::Infer(id) => id.to_tokens(tokens),
            // `Group` is only produced via the proc-macro bridge, never `from_str`.
            Type::Group(_) => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use moxy_token::ToTokenStream;

    use super::*;

    fn roundtrip(src: &str) -> String {
        let t: Type = moxy_token::parse!(src).unwrap();
        t.to_token_stream().to_string()
    }

    fn parse_err(src: &str) -> bool {
        moxy_token::parse!(src).map(|_: Type| ()).is_err()
    }

    #[test]
    fn never_infer_array_macro() {
        assert!(matches!(moxy_token::parse!("!" as Type).unwrap(), Type::Never(_)));
        assert!(matches!(moxy_token::parse!("_" as Type).unwrap(), Type::Infer(_)));
        assert!(matches!(moxy_token::parse!("[u8; 4]" as Type).unwrap(), Type::Array(_)));
        assert!(matches!(moxy_token::parse!("[u8]" as Type).unwrap(), Type::Slice(_)));
        assert!(matches!(moxy_token::parse!("m!(x)" as Type).unwrap(), Type::Macro(_)));
        assert_eq!(roundtrip("[u8 ; 4]"), "[u8 ; 4]");
    }

    #[test]
    fn fn_trait_bounds() {
        assert!(matches!(moxy_token::parse!("Fn(u8) -> bool" as Type).unwrap(), Type::Path(_)));
        assert!(matches!(
            moxy_token::parse!("Box<dyn Fn(u8) -> bool>" as Type).unwrap(),
            Type::Path(_)
        ));
        assert!(matches!(
            moxy_token::parse!("dyn FnMut()" as Type).unwrap(),
            Type::TraitObject(_)
        ));
    }

    #[test]
    fn reference() {
        assert!(matches!(moxy_token::parse!("&'a T" as Type).unwrap(), Type::Reference { .. }));
        assert!(matches!(
            moxy_token::parse!("&mut T" as Type).unwrap(),
            Type::Reference { .. }
        ));
        assert!(matches!(moxy_token::parse!("&T" as Type).unwrap(), Type::Reference { .. }));
    }

    #[test]
    fn pointer() {
        assert!(matches!(
            moxy_token::parse!("*const T" as Type).unwrap(),
            Type::Pointer { .. }
        ));
        assert!(matches!(moxy_token::parse!("*mut T" as Type).unwrap(), Type::Pointer { .. }));
        assert!(parse_err("*T"));
    }

    #[test]
    fn slice() {
        assert!(matches!(moxy_token::parse!("[T]" as Type).unwrap(), Type::Slice { .. }));
    }

    #[test]
    fn paren_vs_tuple() {
        assert!(matches!(moxy_token::parse!("(T)" as Type).unwrap(), Type::Paren { .. }));
        assert!(matches!(moxy_token::parse!("(A, B)" as Type).unwrap(), Type::Tuple { .. }));
        assert!(matches!(moxy_token::parse!("(T,)" as Type).unwrap(), Type::Tuple { .. }));
        assert!(matches!(moxy_token::parse!("()" as Type).unwrap(), Type::Tuple { .. }));
    }

    #[test]
    fn roundtrips() {
        for (src, rendered) in [
            ("&'a T", "& 'a T"),
            ("&mut T", "& mut T"),
            ("*const T", "* const T"),
            ("*mut T", "* mut T"),
            ("[T]", "[T]"),
            ("(T)", "(T)"),
            ("(A, B)", "(A , B)"),
        ] {
            assert_eq!(roundtrip(src), rendered, "roundtrip mismatch for {src}");
        }
    }

    #[test]
    fn path() {
        assert!(matches!(moxy_token::parse!("T" as Type).unwrap(), Type::Path { .. }));
        assert!(matches!(
            moxy_token::parse!("std::vec::Vec" as Type).unwrap(),
            Type::Path { .. }
        ));
        assert_eq!(roundtrip("std :: vec :: Vec"), "std :: vec :: Vec");
    }

    #[test]
    fn qualified_path() {
        assert!(matches!(
            moxy_token::parse!("<T as Trait>::Item" as Type).unwrap(),
            Type::Path { .. }
        ));
        assert_eq!(roundtrip("<T as Trait>::Item"), "< T as Trait > :: Item");
        assert_eq!(roundtrip("<T>::Item"), "< T > :: Item");
    }

    #[test]
    fn nested() {
        assert!(matches!(moxy_token::parse!("&[T]" as Type).unwrap(), Type::Reference { .. }));
        assert_eq!(roundtrip("&[T]"), "& [T]");
        assert_eq!(roundtrip("(A, B)"), "(A , B)");
    }

    #[test]
    fn from_variant() {
        use crate::Delimited;
        let elem = Delimited::bracket(
            moxy_token::span::DelimSpan::default(),
            Box::new(moxy_token::parse!("T" as Type).unwrap()),
        );
        let s = TypeSlice { elem };
        assert!(matches!(Type::from(s), Type::Slice { .. }));
    }
}
