use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::{LexError, Parse, Span, Spanner, ToTokens, TokenStream};

mod item_const;
mod item_enum;
mod item_extern_crate;
mod item_fn;
mod item_foreign_mod;
mod item_impl;
mod item_macro;
mod item_macro_rules;
mod item_mod;
mod item_static;
mod item_struct;
mod item_trait;
mod item_trait_alias;
mod item_type_alias;
mod item_union;
mod item_use;

pub use item_const::*;
pub use item_enum::*;
pub use item_extern_crate::*;
pub use item_fn::*;
pub use item_foreign_mod::*;
pub use item_impl::*;
pub use item_macro::*;
pub use item_macro_rules::*;
pub use item_mod::*;
pub use item_static::*;
pub use item_struct::*;
pub use item_trait::*;
pub use item_trait_alias::*;
pub use item_type_alias::*;
pub use item_union::*;
pub use item_use::*;

/// A top-level item (fn, struct, enum, trait, impl, use, ...).
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Item {
    Use(ItemUse),
    ExternCrate(ItemExternCrate),
    Mod(ItemMod),
    Fn(ItemFn),
    Struct(ItemStruct),
    Enum(ItemEnum),
    Union(ItemUnion),
    Trait(ItemTrait),
    TraitAlias(ItemTraitAlias),
    Impl(ItemImpl),
    TypeAlias(ItemTypeAlias),
    Const(ItemConst),
    Static(ItemStatic),
    Macro(ItemMacro),
    Macro2(ItemMacroRules),
    ForeignMod(ItemForeignMod),
}

impl Item {
    pub fn is_use(&self) -> bool {
        matches!(self, Self::Use(_))
    }

    pub fn is_extern_crate(&self) -> bool {
        matches!(self, Self::ExternCrate(_))
    }

    pub fn is_mod(&self) -> bool {
        matches!(self, Self::Mod(_))
    }

    pub fn is_fn(&self) -> bool {
        matches!(self, Self::Fn(_))
    }

    pub fn is_struct(&self) -> bool {
        matches!(self, Self::Struct(_))
    }

    pub fn is_enum(&self) -> bool {
        matches!(self, Self::Enum(_))
    }

    pub fn is_union(&self) -> bool {
        matches!(self, Self::Union(_))
    }

    pub fn is_trait(&self) -> bool {
        matches!(self, Self::Trait(_))
    }

    pub fn is_trait_alias(&self) -> bool {
        matches!(self, Self::TraitAlias(_))
    }

    pub fn is_impl(&self) -> bool {
        matches!(self, Self::Impl(_))
    }

    pub fn is_type_alias(&self) -> bool {
        matches!(self, Self::TypeAlias(_))
    }

    pub fn is_const(&self) -> bool {
        matches!(self, Self::Const(_))
    }

    pub fn is_static(&self) -> bool {
        matches!(self, Self::Static(_))
    }

    pub fn is_macro(&self) -> bool {
        matches!(self, Self::Macro(_))
    }

    pub fn is_macro2(&self) -> bool {
        matches!(self, Self::Macro2(_))
    }

    pub fn is_foreign_mod(&self) -> bool {
        matches!(self, Self::ForeignMod(_))
    }

    pub fn as_use(&self) -> Option<&ItemUse> {
        if let Self::Use(v) = self { Some(v) } else { None }
    }

    pub fn as_extern_crate(&self) -> Option<&ItemExternCrate> {
        if let Self::ExternCrate(v) = self { Some(v) } else { None }
    }

    pub fn as_mod(&self) -> Option<&ItemMod> {
        if let Self::Mod(v) = self { Some(v) } else { None }
    }

    pub fn as_fn(&self) -> Option<&ItemFn> {
        if let Self::Fn(v) = self { Some(v) } else { None }
    }

    pub fn as_struct(&self) -> Option<&ItemStruct> {
        if let Self::Struct(v) = self { Some(v) } else { None }
    }

    pub fn as_enum(&self) -> Option<&ItemEnum> {
        if let Self::Enum(v) = self { Some(v) } else { None }
    }

    pub fn as_union(&self) -> Option<&ItemUnion> {
        if let Self::Union(v) = self { Some(v) } else { None }
    }

    pub fn as_trait(&self) -> Option<&ItemTrait> {
        if let Self::Trait(v) = self { Some(v) } else { None }
    }

    pub fn as_trait_alias(&self) -> Option<&ItemTraitAlias> {
        if let Self::TraitAlias(v) = self { Some(v) } else { None }
    }

    pub fn as_impl(&self) -> Option<&ItemImpl> {
        if let Self::Impl(v) = self { Some(v) } else { None }
    }

    pub fn as_type_alias(&self) -> Option<&ItemTypeAlias> {
        if let Self::TypeAlias(v) = self { Some(v) } else { None }
    }

    pub fn as_const(&self) -> Option<&ItemConst> {
        if let Self::Const(v) = self { Some(v) } else { None }
    }

    pub fn as_static(&self) -> Option<&ItemStatic> {
        if let Self::Static(v) = self { Some(v) } else { None }
    }

    pub fn as_macro(&self) -> Option<&ItemMacro> {
        if let Self::Macro(v) = self { Some(v) } else { None }
    }

    pub fn as_macro2(&self) -> Option<&ItemMacroRules> {
        if let Self::Macro2(v) = self { Some(v) } else { None }
    }

    pub fn as_foreign_mod(&self) -> Option<&ItemForeignMod> {
        if let Self::ForeignMod(v) = self { Some(v) } else { None }
    }
}

impl Spanner for Item {
    fn span(&self) -> Span {
        match self {
            Item::Use(v) => v.span(),
            Item::ExternCrate(v) => v.span(),
            Item::Mod(v) => v.span(),
            Item::Fn(v) => v.span(),
            Item::Struct(v) => v.span(),
            Item::Enum(v) => v.span(),
            Item::Union(v) => v.span(),
            Item::Trait(v) => v.span(),
            Item::TraitAlias(v) => v.span(),
            Item::Impl(v) => v.span(),
            Item::TypeAlias(v) => v.span(),
            Item::Const(v) => v.span(),
            Item::Static(v) => v.span(),
            Item::Macro(v) => v.span(),
            Item::Macro2(v) => v.span(),
            Item::ForeignMod(v) => v.span(),
        }
    }
}

impl From<ItemUse> for Item {
    fn from(value: ItemUse) -> Self {
        Item::Use(value)
    }
}

impl From<ItemExternCrate> for Item {
    fn from(value: ItemExternCrate) -> Self {
        Item::ExternCrate(value)
    }
}

impl From<ItemMod> for Item {
    fn from(value: ItemMod) -> Self {
        Item::Mod(value)
    }
}

impl From<ItemFn> for Item {
    fn from(value: ItemFn) -> Self {
        Item::Fn(value)
    }
}

impl From<ItemStruct> for Item {
    fn from(value: ItemStruct) -> Self {
        Item::Struct(value)
    }
}

impl From<ItemEnum> for Item {
    fn from(value: ItemEnum) -> Self {
        Item::Enum(value)
    }
}

impl From<ItemUnion> for Item {
    fn from(value: ItemUnion) -> Self {
        Item::Union(value)
    }
}

impl From<ItemTrait> for Item {
    fn from(value: ItemTrait) -> Self {
        Item::Trait(value)
    }
}

impl From<ItemTraitAlias> for Item {
    fn from(value: ItemTraitAlias) -> Self {
        Item::TraitAlias(value)
    }
}

impl From<ItemImpl> for Item {
    fn from(value: ItemImpl) -> Self {
        Item::Impl(value)
    }
}

impl From<ItemTypeAlias> for Item {
    fn from(value: ItemTypeAlias) -> Self {
        Item::TypeAlias(value)
    }
}

impl From<ItemConst> for Item {
    fn from(value: ItemConst) -> Self {
        Item::Const(value)
    }
}

impl From<ItemStatic> for Item {
    fn from(value: ItemStatic) -> Self {
        Item::Static(value)
    }
}

impl From<ItemMacro> for Item {
    fn from(value: ItemMacro) -> Self {
        Item::Macro(value)
    }
}

impl From<ItemMacroRules> for Item {
    fn from(value: ItemMacroRules) -> Self {
        Item::Macro2(value)
    }
}

impl From<ItemForeignMod> for Item {
    fn from(value: ItemForeignMod) -> Self {
        Item::ForeignMod(value)
    }
}

impl Parse for Item {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let at = stream.span();

        if stream.peek::<ItemMacroRules>() {
            return Ok(Item::Macro2(stream.parse()?));
        }
        if stream.peek::<ItemUse>() {
            return Ok(Item::Use(stream.parse()?));
        }
        if stream.peek::<ItemExternCrate>() {
            return Ok(Item::ExternCrate(stream.parse()?));
        }
        if stream.peek::<ItemForeignMod>() {
            return Ok(Item::ForeignMod(stream.parse()?));
        }
        if stream.peek::<ItemMod>() {
            return Ok(Item::Mod(stream.parse()?));
        }
        if stream.peek::<ItemStruct>() {
            return Ok(Item::Struct(stream.parse()?));
        }
        if stream.peek::<ItemEnum>() {
            return Ok(Item::Enum(stream.parse()?));
        }
        if stream.peek::<ItemUnion>() {
            return Ok(Item::Union(stream.parse()?));
        }
        if stream.peek::<ItemTraitAlias>() {
            return Ok(Item::TraitAlias(stream.parse()?));
        }
        if stream.peek::<ItemTrait>() {
            return Ok(Item::Trait(stream.parse()?));
        }
        if stream.peek::<ItemImpl>() {
            return Ok(Item::Impl(stream.parse()?));
        }
        if stream.peek::<ItemTypeAlias>() {
            return Ok(Item::TypeAlias(stream.parse()?));
        }
        if stream.peek::<ItemConst>() {
            return Ok(Item::Const(stream.parse()?));
        }
        if stream.peek::<ItemStatic>() {
            return Ok(Item::Static(stream.parse()?));
        }
        if stream.peek::<ItemFn>() {
            return Ok(Item::Fn(stream.parse()?));
        }
        if stream.peek::<ItemMacro>() {
            return Ok(Item::Macro(stream.parse()?));
        }

        Err(LexError::new(at).message("expected item").into())
    }
}

impl ToTokens for Item {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            Item::Use(v) => v.to_tokens(t),
            Item::ExternCrate(v) => v.to_tokens(t),
            Item::Mod(v) => v.to_tokens(t),
            Item::Fn(v) => v.to_tokens(t),
            Item::Struct(v) => v.to_tokens(t),
            Item::Enum(v) => v.to_tokens(t),
            Item::Union(v) => v.to_tokens(t),
            Item::Trait(v) => v.to_tokens(t),
            Item::TraitAlias(v) => v.to_tokens(t),
            Item::Impl(v) => v.to_tokens(t),
            Item::TypeAlias(v) => v.to_tokens(t),
            Item::Const(v) => v.to_tokens(t),
            Item::Static(v) => v.to_tokens(t),
            Item::Macro(v) => v.to_tokens(t),
            Item::Macro2(v) => v.to_tokens(t),
            Item::ForeignMod(v) => v.to_tokens(t),
        }
    }
}

#[cfg(test)]
mod tests {
    use moxy_token::ToTokenStream;

    use super::*;
    use crate::Crate;

    fn render<T: ToTokenStream>(v: &T) -> String {
        v.to_token_stream().to_string()
    }

    #[test]
    fn item_fn() {
        let i = moxy_token::parse!("fn f<T: A + 'a>(x: T) -> U where T: B { x }" as Item).unwrap();
        assert!(matches!(i, Item::Fn(_)));
    }

    #[test]
    fn item_struct() {
        assert!(matches!(
            moxy_token::parse!("pub struct S<T> { a: T }" as Item).unwrap(),
            Item::Struct(_)
        ));
        assert!(matches!(
            moxy_token::parse!("struct P(u8, u16);" as Item).unwrap(),
            Item::Struct(_)
        ));
        assert!(matches!(moxy_token::parse!("struct U;" as Item).unwrap(), Item::Struct(_)));
    }

    #[test]
    fn item_enum() {
        let i = moxy_token::parse!("enum E { A, B(u8), C { x: i32 } }" as Item).unwrap();
        match i {
            Item::Enum(e) => assert_eq!(e.variants.inner.len(), 3),
            _ => panic!("expected enum"),
        }
    }

    #[test]
    fn item_impl() {
        assert!(matches!(
            moxy_token::parse!("impl<T> Trait for S<T> { fn m(&self) {} }" as Item).unwrap(),
            Item::Impl(_)
        ));
        assert!(matches!(moxy_token::parse!("impl S { }" as Item).unwrap(), Item::Impl(_)));
    }

    #[test]
    fn item_trait() {
        let i = moxy_token::parse!("trait T: Clone { fn m(&self); type Out; }" as Item).unwrap();
        match i {
            Item::Trait(t) => assert_eq!(t.items.inner.len(), 2),
            _ => panic!("expected trait"),
        }
    }

    #[test]
    fn item_use() {
        assert!(matches!(
            moxy_token::parse!("use a::{b, c as d, e::*};" as Item).unwrap(),
            Item::Use(_)
        ));
    }

    #[test]
    fn item_const_static_type() {
        assert!(matches!(
            moxy_token::parse!("const X: u8 = 1;" as Item).unwrap(),
            Item::Const(_)
        ));
        assert!(matches!(
            moxy_token::parse!("static Y: u8 = 1;" as Item).unwrap(),
            Item::Static(_)
        ));
        assert!(matches!(
            moxy_token::parse!("type Z = u8;" as Item).unwrap(),
            Item::TypeAlias(_)
        ));
    }

    #[test]
    fn item_with_attr() {
        let i = moxy_token::parse!("#[derive(Clone)] pub fn g() {}" as Item).unwrap();
        match i {
            Item::Fn(f) => assert_eq!(f.attrs.len(), 1),
            _ => panic!("expected fn"),
        }
    }

    #[test]
    fn item_mod_and_macro() {
        assert!(matches!(
            moxy_token::parse!("mod m { fn a() {} }" as Item).unwrap(),
            Item::Mod(_)
        ));
        assert!(matches!(moxy_token::parse!("mod m;" as Item).unwrap(), Item::Mod(_)));
        assert!(matches!(
            moxy_token::parse!("macro_rules! m { () => {} }" as Item).unwrap(),
            Item::Macro2(_)
        ));
    }

    #[test]
    fn unsafe_auto_trait() {
        match moxy_token::parse!("unsafe trait T {}" as Item).unwrap() {
            Item::Trait(t) => assert!(matches!(t.unsafety, crate::Unsafety::Unsafe(_))),
            _ => panic!("expected trait"),
        }
        match moxy_token::parse!("auto trait T {}" as Item).unwrap() {
            Item::Trait(t) => assert!(t.auto_keyword.is_some()),
            _ => panic!("expected trait"),
        }
        assert!(matches!(
            moxy_token::parse!("unsafe auto trait T {}" as Item).unwrap(),
            Item::Trait(_)
        ));
    }

    #[test]
    fn negative_impl() {
        match moxy_token::parse!("impl !Send for S {}" as Item).unwrap() {
            Item::Impl(i) => {
                let tr = i.trait_ref.unwrap();
                assert!(matches!(tr.polarity, crate::BoundPolarity::Negative(_)));
            }
            _ => panic!("expected impl"),
        }
    }

    #[test]
    fn variadic_fn() {
        let sig = moxy_token::parse!("fn printf(fmt: u8, ...)" as crate::Signature).unwrap();
        assert!(sig.params.inner.variadic.is_some());
    }

    #[test]
    fn crate_roundtrip() {
        let c = moxy_token::parse!("fn a() {} struct S { x: u8 }" as Crate).unwrap();
        assert_eq!(c.items.len(), 2);
        let r = render(&c);
        let c2: Crate = moxy_token::parse!(r).unwrap();
        assert_eq!(render(&c2), r);
    }
}
