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
#[derive(Debug, Clone, PartialEq, Eq)]
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
            Self::Use(v) => v.span(),
            Self::ExternCrate(v) => v.span(),
            Self::Mod(v) => v.span(),
            Self::Fn(v) => v.span(),
            Self::Struct(v) => v.span(),
            Self::Enum(v) => v.span(),
            Self::Union(v) => v.span(),
            Self::Trait(v) => v.span(),
            Self::TraitAlias(v) => v.span(),
            Self::Impl(v) => v.span(),
            Self::TypeAlias(v) => v.span(),
            Self::Const(v) => v.span(),
            Self::Static(v) => v.span(),
            Self::Macro(v) => v.span(),
            Self::Macro2(v) => v.span(),
            Self::ForeignMod(v) => v.span(),
        }
    }
}

impl From<ItemUse> for Item {
    fn from(value: ItemUse) -> Self {
        Self::Use(value)
    }
}

impl From<ItemExternCrate> for Item {
    fn from(value: ItemExternCrate) -> Self {
        Self::ExternCrate(value)
    }
}

impl From<ItemMod> for Item {
    fn from(value: ItemMod) -> Self {
        Self::Mod(value)
    }
}

impl From<ItemFn> for Item {
    fn from(value: ItemFn) -> Self {
        Self::Fn(value)
    }
}

impl From<ItemStruct> for Item {
    fn from(value: ItemStruct) -> Self {
        Self::Struct(value)
    }
}

impl From<ItemEnum> for Item {
    fn from(value: ItemEnum) -> Self {
        Self::Enum(value)
    }
}

impl From<ItemUnion> for Item {
    fn from(value: ItemUnion) -> Self {
        Self::Union(value)
    }
}

impl From<ItemTrait> for Item {
    fn from(value: ItemTrait) -> Self {
        Self::Trait(value)
    }
}

impl From<ItemTraitAlias> for Item {
    fn from(value: ItemTraitAlias) -> Self {
        Self::TraitAlias(value)
    }
}

impl From<ItemImpl> for Item {
    fn from(value: ItemImpl) -> Self {
        Self::Impl(value)
    }
}

impl From<ItemTypeAlias> for Item {
    fn from(value: ItemTypeAlias) -> Self {
        Self::TypeAlias(value)
    }
}

impl From<ItemConst> for Item {
    fn from(value: ItemConst) -> Self {
        Self::Const(value)
    }
}

impl From<ItemStatic> for Item {
    fn from(value: ItemStatic) -> Self {
        Self::Static(value)
    }
}

impl From<ItemMacro> for Item {
    fn from(value: ItemMacro) -> Self {
        Self::Macro(value)
    }
}

impl From<ItemMacroRules> for Item {
    fn from(value: ItemMacroRules) -> Self {
        Self::Macro2(value)
    }
}

impl From<ItemForeignMod> for Item {
    fn from(value: ItemForeignMod) -> Self {
        Self::ForeignMod(value)
    }
}

impl Parse for Item {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let at = stream.span();

        if let Some(item) = stream.parse_if::<ItemMacroRules>() {
            return Ok(Self::Macro2(item));
        }
        if let Some(item) = stream.parse_if::<ItemUse>() {
            return Ok(Self::Use(item));
        }
        if let Some(item) = stream.parse_if::<ItemExternCrate>() {
            return Ok(Self::ExternCrate(item));
        }
        if let Some(item) = stream.parse_if::<ItemForeignMod>() {
            return Ok(Self::ForeignMod(item));
        }
        if let Some(item) = stream.parse_if::<ItemMod>() {
            return Ok(Self::Mod(item));
        }
        if let Some(item) = stream.parse_if::<ItemStruct>() {
            return Ok(Self::Struct(item));
        }
        if let Some(item) = stream.parse_if::<ItemEnum>() {
            return Ok(Self::Enum(item));
        }
        if let Some(item) = stream.parse_if::<ItemUnion>() {
            return Ok(Self::Union(item));
        }
        if let Some(item) = stream.parse_if::<ItemTraitAlias>() {
            return Ok(Self::TraitAlias(item));
        }
        if let Some(item) = stream.parse_if::<ItemTrait>() {
            return Ok(Self::Trait(item));
        }
        if let Some(item) = stream.parse_if::<ItemImpl>() {
            return Ok(Self::Impl(item));
        }
        if let Some(item) = stream.parse_if::<ItemTypeAlias>() {
            return Ok(Self::TypeAlias(item));
        }
        if let Some(item) = stream.parse_if::<ItemConst>() {
            return Ok(Self::Const(item));
        }
        if let Some(item) = stream.parse_if::<ItemStatic>() {
            return Ok(Self::Static(item));
        }
        if let Some(item) = stream.parse_if::<ItemFn>() {
            return Ok(Self::Fn(item));
        }
        if let Some(item) = stream.parse_if::<ItemMacro>() {
            return Ok(Self::Macro(item));
        }

        Err(LexError::new(at).message("expected item").into())
    }
}

impl ToTokens for Item {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            Self::Use(v) => v.to_tokens(t),
            Self::ExternCrate(v) => v.to_tokens(t),
            Self::Mod(v) => v.to_tokens(t),
            Self::Fn(v) => v.to_tokens(t),
            Self::Struct(v) => v.to_tokens(t),
            Self::Enum(v) => v.to_tokens(t),
            Self::Union(v) => v.to_tokens(t),
            Self::Trait(v) => v.to_tokens(t),
            Self::TraitAlias(v) => v.to_tokens(t),
            Self::Impl(v) => v.to_tokens(t),
            Self::TypeAlias(v) => v.to_tokens(t),
            Self::Const(v) => v.to_tokens(t),
            Self::Static(v) => v.to_tokens(t),
            Self::Macro(v) => v.to_tokens(t),
            Self::Macro2(v) => v.to_tokens(t),
            Self::ForeignMod(v) => v.to_tokens(t),
        }
    }
}
