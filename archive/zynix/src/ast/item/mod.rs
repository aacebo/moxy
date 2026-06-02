use crate::parse::{ParseError, ParseStream};
use crate::token::{LexError, ToTokens};
use crate::{Parse, TokenStream};

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

#[doc = "A top-level item (fn, struct, enum, trait, impl, use, ...)."]
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

        if stream.peek::<ItemMacroRules>().is_some() {
            return Ok(Item::Macro2(stream.parse()?));
        }
        if stream.peek::<ItemUse>().is_some() {
            return Ok(Item::Use(stream.parse()?));
        }
        if stream.peek::<ItemExternCrate>().is_some() {
            return Ok(Item::ExternCrate(stream.parse()?));
        }
        if stream.peek::<ItemForeignMod>().is_some() {
            return Ok(Item::ForeignMod(stream.parse()?));
        }
        if stream.peek::<ItemMod>().is_some() {
            return Ok(Item::Mod(stream.parse()?));
        }
        if stream.peek::<ItemStruct>().is_some() {
            return Ok(Item::Struct(stream.parse()?));
        }
        if stream.peek::<ItemEnum>().is_some() {
            return Ok(Item::Enum(stream.parse()?));
        }
        if stream.peek::<ItemUnion>().is_some() {
            return Ok(Item::Union(stream.parse()?));
        }
        if stream.peek::<ItemTraitAlias>().is_some() {
            return Ok(Item::TraitAlias(stream.parse()?));
        }
        if stream.peek::<ItemTrait>().is_some() {
            return Ok(Item::Trait(stream.parse()?));
        }
        if stream.peek::<ItemImpl>().is_some() {
            return Ok(Item::Impl(stream.parse()?));
        }
        if stream.peek::<ItemTypeAlias>().is_some() {
            return Ok(Item::TypeAlias(stream.parse()?));
        }
        if stream.peek::<ItemConst>().is_some() {
            return Ok(Item::Const(stream.parse()?));
        }
        if stream.peek::<ItemStatic>().is_some() {
            return Ok(Item::Static(stream.parse()?));
        }
        if stream.peek::<ItemFn>().is_some() {
            return Ok(Item::Fn(stream.parse()?));
        }
        if stream.peek::<ItemMacro>().is_some() {
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
    use std::str::FromStr;

    use super::*;
    use crate::ast::Crate;
    use crate::token::ToTokenStream;

    fn parse<T: Parse>(src: &str) -> T {
        let ts = TokenStream::from_str(src).unwrap();
        ts.parse().parse::<T>().unwrap()
    }

    fn render<T: ToTokenStream>(v: &T) -> String {
        v.to_token_stream().to_string()
    }

    #[test]
    fn item_fn() {
        let i: Item = parse("fn f<T: A + 'a>(x: T) -> U where T: B { x }");
        assert!(matches!(i, Item::Fn(_)));
    }

    #[test]
    fn item_struct() {
        assert!(matches!(parse::<Item>("pub struct S<T> { a: T }"), Item::Struct(_)));
        assert!(matches!(parse::<Item>("struct P(u8, u16);"), Item::Struct(_)));
        assert!(matches!(parse::<Item>("struct U;"), Item::Struct(_)));
    }

    #[test]
    fn item_enum() {
        let i: Item = parse("enum E { A, B(u8), C { x: i32 } }");
        match i {
            Item::Enum(e) => assert_eq!(e.variants.len(), 3),
            _ => panic!("expected enum"),
        }
    }

    #[test]
    fn item_impl() {
        assert!(matches!(
            parse::<Item>("impl<T> Trait for S<T> { fn m(&self) {} }"),
            Item::Impl(_)
        ));
        assert!(matches!(parse::<Item>("impl S { }"), Item::Impl(_)));
    }

    #[test]
    fn item_trait() {
        let i: Item = parse("trait T: Clone { fn m(&self); type Out; }");
        match i {
            Item::Trait(t) => assert_eq!(t.items.len(), 2),
            _ => panic!("expected trait"),
        }
    }

    #[test]
    fn item_use() {
        assert!(matches!(parse::<Item>("use a::{b, c as d, e::*};"), Item::Use(_)));
    }

    #[test]
    fn item_const_static_type() {
        assert!(matches!(parse::<Item>("const X: u8 = 1;"), Item::Const(_)));
        assert!(matches!(parse::<Item>("static Y: u8 = 1;"), Item::Static(_)));
        assert!(matches!(parse::<Item>("type Z = u8;"), Item::TypeAlias(_)));
    }

    #[test]
    fn item_with_attr() {
        let i: Item = parse("#[derive(Clone)] pub fn g() {}");
        match i {
            Item::Fn(f) => assert_eq!(f.attrs.len(), 1),
            _ => panic!("expected fn"),
        }
    }

    #[test]
    fn item_mod_and_macro() {
        assert!(matches!(parse::<Item>("mod m { fn a() {} }"), Item::Mod(_)));
        assert!(matches!(parse::<Item>("mod m;"), Item::Mod(_)));
        assert!(matches!(parse::<Item>("macro_rules! m { () => {} }"), Item::Macro2(_)));
    }

    #[test]
    fn unsafe_auto_trait() {
        match parse::<Item>("unsafe trait T {}") {
            Item::Trait(t) => assert!(matches!(t.unsafety, crate::ast::Unsafety::Unsafe)),
            _ => panic!("expected trait"),
        }
        match parse::<Item>("auto trait T {}") {
            Item::Trait(t) => assert!(t.auto),
            _ => panic!("expected trait"),
        }
        assert!(matches!(parse::<Item>("unsafe auto trait T {}"), Item::Trait(_)));
    }

    #[test]
    fn negative_impl() {
        match parse::<Item>("impl !Send for S {}") {
            Item::Impl(i) => {
                let tr = i.trait_ref.unwrap();
                assert!(matches!(tr.polarity, crate::ast::BoundPolarity::Negative));
            }
            _ => panic!("expected impl"),
        }
    }

    #[test]
    fn variadic_fn() {
        // extern "C" fn(u8, ...) as a bare-fn type round-trips the variadic.
        let sig: crate::ast::Signature = {
            let ts = TokenStream::from_str("fn printf(fmt: u8, ...)").unwrap();
            ts.parse().parse().unwrap()
        };
        assert!(sig.variadic.is_some());
    }

    #[test]
    fn crate_roundtrip() {
        let c: Crate = parse("fn a() {} struct S { x: u8 }");
        assert_eq!(c.items.len(), 2);
        // re-render and re-parse stability
        let r = render(&c);
        let c2: Crate = parse(&r);
        assert_eq!(render(&c2), r);
    }
}
