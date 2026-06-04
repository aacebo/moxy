use moxy_ast::fields::{FieldDef, Fields, FieldsNamed, FieldsUnnamed};
use moxy_ast::item::*;
use moxy_ast::member::foreign_item::*;
use moxy_ast::member::impl_item::*;
use moxy_ast::member::trait_item::*;
use moxy_ast::member::{ForeignItem, ImplItem, TraitItem};
use moxy_ast::sig::{FnParam, Receiver, Signature, Variadic};
use moxy_ast::use_tree::*;
use moxy_ast::{Item, UseTree, Variant};

use crate::{Fmt, FmtError, Formatter};

// ── Signature ─────────────────────────────────────────────────────────────────

impl Fmt for Signature {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.constness.fmt(f)?;

        if matches!(self.constness, moxy_ast::Constness::Const(_)) {
            f.text(" ")?;
        }

        self.asyncness.fmt(f)?;

        if matches!(self.asyncness, moxy_ast::Asyncness::Async(_)) {
            f.text(" ")?;
        }

        self.unsafety.fmt(f)?;

        if matches!(self.unsafety, moxy_ast::Unsafety::Unsafe(_)) {
            f.text(" ")?;
        }

        if let Some(abi) = &self.abi {
            abi.fmt(f)?;
            f.text(" ")?;
        }

        f.text("fn ")?;
        self.ident.fmt(f)?;
        self.generics.fmt(f)?;
        f.text("(")?;
        f.group(|f| {
            for pair in self.paren.inner.inputs.pairs() {
                match pair {
                    moxy_ast::Pair::Punctuated(param, _) => {
                        param.fmt(f)?;
                        f.text(",")?;
                        f.text(" ")?;
                    }
                    moxy_ast::Pair::End(param) => {
                        param.fmt(f)?;
                    }
                }
            }

            if let Some(variadic) = &self.paren.inner.variadic {
                if !self.paren.inner.inputs.is_empty() {
                    f.text(",")?;
                    f.text(" ")?;
                }

                variadic.fmt(f)?;
            }

            Ok(())
        })?;
        f.text(")")?;
        self.output.fmt(f)
    }
}

impl Fmt for FnParam {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Receiver(v) => v.fmt(f),
            Self::Typed(v) => v.fmt(f),
        }
    }
}

impl Fmt for Receiver {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        if self.reference.is_some() {
            f.text("&")?;

            if let Some(lt) = &self.lifetime {
                lt.fmt(f)?;
                f.text(" ")?;
            }

            self.mutability.fmt(f)?;

            if matches!(self.mutability, moxy_ast::Mutability::Mutable(_)) {
                f.text(" ")?;
            }
        } else {
            self.mutability.fmt(f)?;

            if matches!(self.mutability, moxy_ast::Mutability::Mutable(_)) {
                f.text(" ")?;
            }
        }

        f.text("self")
    }
}

impl Fmt for Variadic {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        if let Some(name) = &self.name {
            name.fmt(f)?;
            f.text(": ")?;
        }

        f.text("...")
    }
}

// ── Fields ────────────────────────────────────────────────────────────────────

impl Fmt for Fields {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Named(v) => v.fmt(f),
            Self::Unnamed(v) => v.fmt(f),
            Self::Unit => Ok(()),
        }
    }
}

impl Fmt for FieldsNamed {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text(" {")?;
        f.indent(|f| {
            for pair in self.brace.inner.pairs() {
                f.hard_break()?;

                match pair {
                    moxy_ast::Pair::Punctuated(field, _) => {
                        field.fmt(f)?;
                        f.text(",")?;
                    }
                    moxy_ast::Pair::End(field) => {
                        field.fmt(f)?;
                        f.text(",")?;
                    }
                }
            }

            Ok(())
        })?;

        if !self.brace.inner.is_empty() {
            f.hard_break()?;
        }

        f.text("}")
    }
}

impl Fmt for FieldsUnnamed {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("(")?;
        self.paren.inner.fmt(f)?;
        f.text(")")
    }
}

impl Fmt for FieldDef {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.vis.fmt(f)?;

        if !matches!(self.vis, moxy_ast::Visibility::Inherited) {
            f.text(" ")?;
        }

        if let Some(ident) = &self.ident {
            ident.fmt(f)?;
            f.text(": ")?;
        }

        self.ty.fmt(f)
    }
}

// ── UseTree ───────────────────────────────────────────────────────────────────

impl Fmt for UseTree {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Path(v) => v.fmt(f),
            Self::Name(v) => v.fmt(f),
            Self::Rename(v) => v.fmt(f),
            Self::Glob(_) => f.text("*"),
            Self::Group(v) => v.fmt(f),
        }
    }
}

impl Fmt for UsePath {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.ident.fmt(f)?;
        f.text("::")?;
        self.tree.fmt(f)
    }
}

impl Fmt for UseName {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.ident.fmt(f)
    }
}

impl Fmt for UseRename {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.ident.fmt(f)?;
        f.text(" as ")?;
        self.rename.fmt(f)
    }
}

impl Fmt for UseGroup {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("{")?;
        self.brace.inner.fmt(f)?;
        f.text("}")
    }
}

// ── Item ──────────────────────────────────────────────────────────────────────

impl Fmt for Item {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Use(v) => v.fmt(f),
            Self::ExternCrate(v) => v.fmt(f),
            Self::Mod(v) => v.fmt(f),
            Self::Fn(v) => v.fmt(f),
            Self::Struct(v) => v.fmt(f),
            Self::Enum(v) => v.fmt(f),
            Self::Union(v) => v.fmt(f),
            Self::Trait(v) => v.fmt(f),
            Self::TraitAlias(v) => v.fmt(f),
            Self::Impl(v) => v.fmt(f),
            Self::TypeAlias(v) => v.fmt(f),
            Self::Const(v) => v.fmt(f),
            Self::Static(v) => v.fmt(f),
            Self::Macro(v) => v.fmt(f),
            Self::Macro2(v) => v.fmt(f),
            Self::ForeignMod(v) => v.fmt(f),
        }
    }
}

impl Fmt for ItemUse {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.vis.fmt(f)?;

        if !matches!(self.vis, moxy_ast::Visibility::Inherited) {
            f.text(" ")?;
        }

        f.text("use ")?;
        self.tree.fmt(f)?;
        f.text(";")
    }
}

impl Fmt for ItemExternCrate {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.vis.fmt(f)?;

        if !matches!(self.vis, moxy_ast::Visibility::Inherited) {
            f.text(" ")?;
        }

        f.text("extern crate ")?;
        self.ident.fmt(f)?;

        if let Some(rename) = &self.rename {
            f.text(" as ")?;
            rename.fmt(f)?;
        }

        f.text(";")
    }
}

impl Fmt for ItemMod {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.vis.fmt(f)?;

        if !matches!(self.vis, moxy_ast::Visibility::Inherited) {
            f.text(" ")?;
        }

        self.unsafety.fmt(f)?;

        if matches!(self.unsafety, moxy_ast::Unsafety::Unsafe(_)) {
            f.text(" ")?;
        }

        f.text("mod ")?;
        self.ident.fmt(f)?;

        if let Some(content) = &self.content {
            f.text(" {")?;
            f.indent(|f| {
                for (i, item) in content.inner.iter().enumerate() {
                    f.hard_break()?;
                    if i > 0 {
                        f.hard_break()?;
                    }
                    item.fmt(f)?;
                }

                Ok(())
            })?;

            if !content.inner.is_empty() {
                f.hard_break()?;
            }

            f.text("}")
        } else {
            f.text(";")
        }
    }
}

impl Fmt for ItemFn {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.vis.fmt(f)?;

        if !matches!(self.vis, moxy_ast::Visibility::Inherited) {
            f.text(" ")?;
        }

        self.defaultness.fmt(f)?;

        if matches!(self.defaultness, moxy_ast::Defaultness::Default(_)) {
            f.text(" ")?;
        }

        self.sig.fmt(f)?;
        f.text(" ")?;
        self.body.fmt(f)
    }
}

impl Fmt for ItemStruct {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.vis.fmt(f)?;

        if !matches!(self.vis, moxy_ast::Visibility::Inherited) {
            f.text(" ")?;
        }

        f.text("struct ")?;
        self.ident.fmt(f)?;
        self.generics.fmt(f)?;
        self.fields.fmt(f)?;

        if matches!(self.fields, moxy_ast::Fields::Unnamed(_) | moxy_ast::Fields::Unit) {
            f.text(";")?;
        }

        Ok(())
    }
}

impl Fmt for ItemEnum {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.vis.fmt(f)?;

        if !matches!(self.vis, moxy_ast::Visibility::Inherited) {
            f.text(" ")?;
        }

        f.text("enum ")?;
        self.ident.fmt(f)?;
        self.generics.fmt(f)?;
        f.text(" {")?;
        f.indent(|f| {
            for pair in self.brace.inner.pairs() {
                f.hard_break()?;

                match pair {
                    moxy_ast::Pair::Punctuated(v, _) => {
                        v.fmt(f)?;
                        f.text(",")?;
                    }
                    moxy_ast::Pair::End(v) => {
                        v.fmt(f)?;
                        f.text(",")?;
                    }
                }
            }

            Ok(())
        })?;

        if !self.brace.inner.is_empty() {
            f.hard_break()?;
        }

        f.text("}")
    }
}

impl Fmt for Variant {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.ident.fmt(f)?;
        self.fields.fmt(f)?;

        if let Some(discriminant) = &self.discriminant {
            f.text(" = ")?;
            discriminant.fmt(f)?;
        }

        Ok(())
    }
}

impl Fmt for ItemUnion {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.vis.fmt(f)?;

        if !matches!(self.vis, moxy_ast::Visibility::Inherited) {
            f.text(" ")?;
        }

        f.text("union ")?;
        self.ident.fmt(f)?;
        self.generics.fmt(f)?;
        self.fields.fmt(f)
    }
}

impl Fmt for ItemTrait {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.vis.fmt(f)?;

        if !matches!(self.vis, moxy_ast::Visibility::Inherited) {
            f.text(" ")?;
        }

        self.unsafety.fmt(f)?;

        if matches!(self.unsafety, moxy_ast::Unsafety::Unsafe(_)) {
            f.text(" ")?;
        }

        if self.auto_keyword.is_some() {
            f.text("auto ")?;
        }

        f.text("trait ")?;
        self.ident.fmt(f)?;
        self.generics.fmt(f)?;

        if !self.supertraits.is_empty() {
            f.text(": ")?;
            self.supertraits.fmt(f)?;
        }

        f.text(" {")?;
        f.indent(|f| {
            for item in &self.brace.inner {
                f.hard_break()?;
                item.fmt(f)?;
            }

            Ok(())
        })?;

        if !self.brace.inner.is_empty() {
            f.hard_break()?;
        }

        f.text("}")
    }
}

impl Fmt for ItemTraitAlias {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.vis.fmt(f)?;

        if !matches!(self.vis, moxy_ast::Visibility::Inherited) {
            f.text(" ")?;
        }

        f.text("trait ")?;
        self.ident.fmt(f)?;
        self.generics.fmt(f)?;
        f.text(" = ")?;
        self.bounds.fmt(f)?;
        f.text(";")
    }
}

impl Fmt for ItemImpl {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.defaultness.fmt(f)?;

        if matches!(self.defaultness, moxy_ast::Defaultness::Default(_)) {
            f.text(" ")?;
        }

        self.unsafety.fmt(f)?;

        if matches!(self.unsafety, moxy_ast::Unsafety::Unsafe(_)) {
            f.text(" ")?;
        }

        f.text("impl")?;
        self.generics.fmt(f)?;
        f.text(" ")?;

        if let Some(trait_ref) = &self.trait_ref {
            trait_ref.fmt(f)?;
            f.text(" for ")?;
        }

        self.self_ty.fmt(f)?;

        f.text(" {")?;
        f.indent(|f| {
            for (i, item) in self.brace.inner.iter().enumerate() {
                f.hard_break()?;
                if i > 0 {
                    f.hard_break()?;
                }
                item.fmt(f)?;
            }

            Ok(())
        })?;

        if !self.brace.inner.is_empty() {
            f.hard_break()?;
        }

        f.text("}")
    }
}

impl Fmt for ItemTypeAlias {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.vis.fmt(f)?;

        if !matches!(self.vis, moxy_ast::Visibility::Inherited) {
            f.text(" ")?;
        }

        f.text("type ")?;
        self.ident.fmt(f)?;
        self.generics.fmt(f)?;
        f.text(" = ")?;
        self.ty.fmt(f)?;
        f.text(";")
    }
}

impl Fmt for ItemConst {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.vis.fmt(f)?;

        if !matches!(self.vis, moxy_ast::Visibility::Inherited) {
            f.text(" ")?;
        }

        f.text("const ")?;
        self.ident.fmt(f)?;
        self.generics.fmt(f)?;
        f.text(": ")?;
        self.ty.fmt(f)?;
        f.text(" = ")?;
        self.expr.fmt(f)?;
        f.text(";")
    }
}

impl Fmt for ItemStatic {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.vis.fmt(f)?;

        if !matches!(self.vis, moxy_ast::Visibility::Inherited) {
            f.text(" ")?;
        }

        f.text("static ")?;
        self.mutability.fmt(f)?;

        if matches!(self.mutability, moxy_ast::Mutability::Mutable(_)) {
            f.text(" ")?;
        }

        self.ident.fmt(f)?;
        f.text(": ")?;
        self.ty.fmt(f)?;
        f.text(" = ")?;
        self.expr.fmt(f)?;
        f.text(";")
    }
}

impl Fmt for ItemMacro {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        if let Some(ident) = &self.ident {
            ident.fmt(f)?;
            f.text(" ")?;
        }

        self.mac.fmt(f)?;

        if self.semi {
            f.text(";")?;
        }

        Ok(())
    }
}

impl Fmt for ItemMacroRules {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("macro_rules! ")?;
        self.ident.fmt(f)?;
        f.text(" {")?;
        f.text(self.body.stream())?;
        f.text("}")
    }
}

impl Fmt for ItemForeignMod {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.unsafety.fmt(f)?;

        if matches!(self.unsafety, moxy_ast::Unsafety::Unsafe(_)) {
            f.text(" ")?;
        }

        self.abi.fmt(f)?;
        f.text(" {")?;
        f.indent(|f| {
            for (i, item) in self.brace.inner.iter().enumerate() {
                f.hard_break()?;
                if i > 0 {
                    f.hard_break()?;
                }
                item.fmt(f)?;
            }

            Ok(())
        })?;

        if !self.brace.inner.is_empty() {
            f.hard_break()?;
        }

        f.text("}")
    }
}

// ── ImplItem ──────────────────────────────────────────────────────────────────

impl Fmt for ImplItem {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Fn(v) => v.fmt(f),
            Self::Const(v) => v.fmt(f),
            Self::Type(v) => v.fmt(f),
            Self::Macro(v) => v.fmt(f),
        }
    }
}

impl Fmt for ImplItemFn {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.vis.fmt(f)?;

        if !matches!(self.vis, moxy_ast::Visibility::Inherited) {
            f.text(" ")?;
        }

        self.defaultness.fmt(f)?;

        if matches!(self.defaultness, moxy_ast::Defaultness::Default(_)) {
            f.text(" ")?;
        }

        self.sig.fmt(f)?;
        f.text(" ")?;
        self.body.fmt(f)
    }
}

impl Fmt for ImplItemConst {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.vis.fmt(f)?;

        if !matches!(self.vis, moxy_ast::Visibility::Inherited) {
            f.text(" ")?;
        }

        self.defaultness.fmt(f)?;

        if matches!(self.defaultness, moxy_ast::Defaultness::Default(_)) {
            f.text(" ")?;
        }

        f.text("const ")?;
        self.ident.fmt(f)?;
        self.generics.fmt(f)?;
        f.text(": ")?;
        self.ty.fmt(f)?;
        f.text(" = ")?;
        self.expr.fmt(f)?;
        f.text(";")
    }
}

impl Fmt for ImplItemType {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.vis.fmt(f)?;

        if !matches!(self.vis, moxy_ast::Visibility::Inherited) {
            f.text(" ")?;
        }

        self.defaultness.fmt(f)?;

        if matches!(self.defaultness, moxy_ast::Defaultness::Default(_)) {
            f.text(" ")?;
        }

        f.text("type ")?;
        self.ident.fmt(f)?;
        self.generics.fmt(f)?;
        f.text(" = ")?;
        self.ty.fmt(f)?;
        f.text(";")
    }
}

impl Fmt for ImplItemMacro {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.mac.fmt(f)?;

        if self.semi.is_some() {
            f.text(";")?;
        }

        Ok(())
    }
}

// ── TraitItem ─────────────────────────────────────────────────────────────────

impl Fmt for TraitItem {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Fn(v) => v.fmt(f),
            Self::Const(v) => v.fmt(f),
            Self::Type(v) => v.fmt(f),
            Self::Macro(v) => v.fmt(f),
        }
    }
}

impl Fmt for TraitItemFn {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.sig.fmt(f)?;

        if let Some(body) = &self.default_body {
            f.text(" ")?;
            body.fmt(f)?;
        } else {
            f.text(";")?;
        }

        Ok(())
    }
}

impl Fmt for TraitItemConst {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("const ")?;
        self.ident.fmt(f)?;
        self.generics.fmt(f)?;
        f.text(": ")?;
        self.ty.fmt(f)?;

        if let Some((_, default)) = &self.default {
            f.text(" = ")?;
            default.fmt(f)?;
        }

        f.text(";")
    }
}

impl Fmt for TraitItemType {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("type ")?;
        self.ident.fmt(f)?;
        self.generics.fmt(f)?;

        if !self.bounds.is_empty() {
            f.text(": ")?;
            self.bounds.fmt(f)?;
        }

        if let Some((_, default)) = &self.default {
            f.text(" = ")?;
            default.fmt(f)?;
        }

        f.text(";")
    }
}

impl Fmt for TraitItemMacro {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.mac.fmt(f)?;

        if self.semi.is_some() {
            f.text(";")?;
        }

        Ok(())
    }
}

// ── ForeignItem ───────────────────────────────────────────────────────────────

impl Fmt for ForeignItem {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Fn(v) => v.fmt(f),
            Self::Static(v) => v.fmt(f),
            Self::Type(v) => v.fmt(f),
            Self::Macro(v) => v.fmt(f),
        }
    }
}

impl Fmt for ForeignItemFn {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.vis.fmt(f)?;

        if !matches!(self.vis, moxy_ast::Visibility::Inherited) {
            f.text(" ")?;
        }

        self.sig.fmt(f)?;
        f.text(";")
    }
}

impl Fmt for ForeignItemStatic {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.vis.fmt(f)?;

        if !matches!(self.vis, moxy_ast::Visibility::Inherited) {
            f.text(" ")?;
        }

        f.text("static ")?;
        self.mutability.fmt(f)?;

        if matches!(self.mutability, moxy_ast::Mutability::Mutable(_)) {
            f.text(" ")?;
        }

        self.ident.fmt(f)?;
        f.text(": ")?;
        self.ty.fmt(f)?;
        f.text(";")
    }
}

impl Fmt for ForeignItemType {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.vis.fmt(f)?;

        if !matches!(self.vis, moxy_ast::Visibility::Inherited) {
            f.text(" ")?;
        }

        f.text("type ")?;
        self.ident.fmt(f)?;
        self.generics.fmt(f)?;
        f.text(";")
    }
}

impl Fmt for ForeignItemMacro {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.mac.fmt(f)?;

        if self.semi.is_some() {
            f.text(";")?;
        }

        Ok(())
    }
}
