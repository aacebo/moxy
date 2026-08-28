use moxy_ast::fields::{Field, Fields, FieldsNamed, FieldsUnnamed};
use moxy_ast::item::*;
use moxy_ast::member::foreign_item::*;
use moxy_ast::member::impl_item::*;
use moxy_ast::member::trait_item::*;
use moxy_ast::member::{ForeignItem, ImplItem, TraitItem};
use moxy_ast::sig::{FnParam, Receiver, Signature, Variadic};
use moxy_ast::use_tree::*;
use moxy_ast::{Item, UseTree, Variant};

use crate::{FmtError, Format, Formatter};

// ── Signature ─────────────────────────────────────────────────────────────────

impl Format for Signature {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.constness.format(f)?;

        if matches!(self.constness, moxy_ast::Constness::Const(_)) {
            f.text(" ")?;
        }

        self.asyncness.format(f)?;

        if matches!(self.asyncness, moxy_ast::Asyncness::Async(_)) {
            f.text(" ")?;
        }

        self.unsafety.format(f)?;

        if matches!(self.unsafety, moxy_ast::Unsafety::Unsafe(_)) {
            f.text(" ")?;
        }

        if let Some(abi) = &self.abi {
            abi.format(f)?;
            f.text(" ")?;
        }

        f.text("fn ")?;
        self.ident.format(f)?;
        self.generics.format(f)?;
        f.text("(")?;
        f.group(|f| {
            for pair in self.params.inner.inputs.pairs() {
                match pair {
                    moxy_ast::Pair::Punctuated(param, _) => {
                        param.format(f)?;
                        f.text(",")?;
                        f.text(" ")?;
                    }
                    moxy_ast::Pair::End(param) => {
                        param.format(f)?;
                    }
                }
            }

            if let Some(variadic) = &self.params.inner.variadic {
                variadic.format(f)?;
            }

            Ok(())
        })?;

        f.text(")")?;
        self.output.format(f)?;

        if let Some(where_clause) = &self.generics.where_clause {
            where_clause.format(f)?;
        }

        Ok(())
    }
}

impl Format for FnParam {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Receiver(v) => v.format(f),
            Self::Typed(v) => v.format(f),
        }
    }
}

impl Format for Receiver {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        if self.reference.is_some() {
            f.text("&")?;

            if let Some(lt) = &self.lifetime {
                lt.format(f)?;
                f.text(" ")?;
            }

            self.mutability.format(f)?;

            if matches!(self.mutability, moxy_ast::Mutability::Mutable(_)) {
                f.text(" ")?;
            }
        } else {
            self.mutability.format(f)?;

            if matches!(self.mutability, moxy_ast::Mutability::Mutable(_)) {
                f.text(" ")?;
            }
        }

        f.text("self")
    }
}

impl Format for Variadic {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        if let Some(name) = &self.name {
            name.format(f)?;
            f.text(": ")?;
        }

        f.text("...")
    }
}

// ── Fields ────────────────────────────────────────────────────────────────────

impl Format for Fields {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Named(v) => v.format(f),
            Self::Unnamed(v) => v.format(f),
            Self::Unit => Ok(()),
        }
    }
}

impl Format for FieldsNamed {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text(" {")?;
        f.indent(|f| {
            for pair in self.fields.inner.pairs() {
                f.hard_break()?;

                match pair {
                    moxy_ast::Pair::Punctuated(field, _) => {
                        field.format(f)?;
                        f.text(",")?;
                    }
                    moxy_ast::Pair::End(field) => {
                        field.format(f)?;
                        f.text(",")?;
                    }
                }
            }

            Ok(())
        })?;

        if !self.fields.inner.is_empty() {
            f.hard_break()?;
        }

        f.text("}")
    }
}

impl Format for FieldsUnnamed {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("(")?;
        self.fields.inner.format(f)?;
        f.text(")")
    }
}

impl Format for Field {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.vis.format(f)?;

        if !matches!(self.vis, moxy_ast::Visibility::Inherited) {
            f.text(" ")?;
        }

        if let Some(ident) = &self.ident {
            ident.format(f)?;
            f.text(": ")?;
        }

        self.ty.format(f)
    }
}

// ── UseTree ───────────────────────────────────────────────────────────────────

impl Format for UseTree {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Path(v) => v.format(f),
            Self::Name(v) => v.format(f),
            Self::Rename(v) => v.format(f),
            Self::Glob(_) => f.text("*"),
            Self::Group(v) => v.format(f),
        }
    }
}

impl Format for UsePath {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.ident.format(f)?;
        f.text("::")?;
        self.tree.format(f)
    }
}

impl Format for UseName {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.ident.format(f)
    }
}

impl Format for UseRename {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.ident.format(f)?;
        f.text(" as ")?;
        self.rename.format(f)
    }
}

impl Format for UseGroup {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("{")?;
        self.items.inner.format(f)?;
        f.text("}")
    }
}

// ── Item ──────────────────────────────────────────────────────────────────────

impl Format for Item {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Use(v) => v.format(f),
            Self::ExternCrate(v) => v.format(f),
            Self::Mod(v) => v.format(f),
            Self::Fn(v) => v.format(f),
            Self::Struct(v) => v.format(f),
            Self::Enum(v) => v.format(f),
            Self::Union(v) => v.format(f),
            Self::Trait(v) => v.format(f),
            Self::TraitAlias(v) => v.format(f),
            Self::Impl(v) => v.format(f),
            Self::TypeAlias(v) => v.format(f),
            Self::Const(v) => v.format(f),
            Self::Static(v) => v.format(f),
            Self::Macro(v) => v.format(f),
            Self::Macro2(v) => v.format(f),
            Self::ForeignMod(v) => v.format(f),
        }
    }
}

impl Format for ItemUse {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.vis.format(f)?;

        if !matches!(self.vis, moxy_ast::Visibility::Inherited) {
            f.text(" ")?;
        }

        f.text("use ")?;
        self.tree.format(f)?;
        f.text(";")
    }
}

impl Format for ItemExternCrate {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.vis.format(f)?;

        if !matches!(self.vis, moxy_ast::Visibility::Inherited) {
            f.text(" ")?;
        }

        f.text("extern crate ")?;
        self.ident.format(f)?;

        if let Some(rename) = &self.rename {
            f.text(" as ")?;
            rename.format(f)?;
        }

        f.text(";")
    }
}

impl Format for ItemMod {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.vis.format(f)?;

        if !matches!(self.vis, moxy_ast::Visibility::Inherited) {
            f.text(" ")?;
        }

        self.unsafety.format(f)?;

        if matches!(self.unsafety, moxy_ast::Unsafety::Unsafe(_)) {
            f.text(" ")?;
        }

        f.text("mod ")?;
        self.ident.format(f)?;

        if let Some(content) = &self.content {
            f.text(" {")?;
            f.indent(|f| {
                for (i, item) in content.inner.iter().enumerate() {
                    f.hard_break()?;
                    if i > 0 {
                        f.hard_break()?;
                    }
                    item.format(f)?;
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

impl Format for ItemFn {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.vis.format(f)?;

        if !matches!(self.vis, moxy_ast::Visibility::Inherited) {
            f.text(" ")?;
        }

        self.sig.format(f)?;
        f.text(" ")?;
        self.body.format(f)
    }
}

impl Format for ItemStruct {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.vis.format(f)?;

        if !matches!(self.vis, moxy_ast::Visibility::Inherited) {
            f.text(" ")?;
        }

        f.text("struct ")?;
        self.ident.format(f)?;
        self.generics.format(f)?;

        if let Some(where_clause) = &self.generics.where_clause {
            where_clause.format(f)?;
        }

        self.fields.format(f)?;

        if matches!(self.fields, moxy_ast::Fields::Unnamed(_) | moxy_ast::Fields::Unit) {
            f.text(";")?;
        }

        Ok(())
    }
}

impl Format for ItemEnum {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.vis.format(f)?;

        if !matches!(self.vis, moxy_ast::Visibility::Inherited) {
            f.text(" ")?;
        }

        f.text("enum ")?;
        self.ident.format(f)?;
        self.generics.format(f)?;
        f.text(" {")?;
        f.indent(|f| {
            for pair in self.variants.inner.pairs() {
                f.hard_break()?;

                match pair {
                    moxy_ast::Pair::Punctuated(v, _) => {
                        v.format(f)?;
                        f.text(",")?;
                    }
                    moxy_ast::Pair::End(v) => {
                        v.format(f)?;
                        f.text(",")?;
                    }
                }
            }

            Ok(())
        })?;

        if !self.variants.inner.is_empty() {
            f.hard_break()?;
        }

        f.text("}")
    }
}

impl Format for Variant {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.ident.format(f)?;
        self.fields.format(f)?;

        if let Some(discriminant) = &self.discriminant {
            f.text(" = ")?;
            discriminant.format(f)?;
        }

        Ok(())
    }
}

impl Format for ItemUnion {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.vis.format(f)?;

        if !matches!(self.vis, moxy_ast::Visibility::Inherited) {
            f.text(" ")?;
        }

        f.text("union ")?;
        self.ident.format(f)?;
        self.generics.format(f)?;
        self.fields.format(f)
    }
}

impl Format for ItemTrait {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.vis.format(f)?;

        if !matches!(self.vis, moxy_ast::Visibility::Inherited) {
            f.text(" ")?;
        }

        self.unsafety.format(f)?;

        if matches!(self.unsafety, moxy_ast::Unsafety::Unsafe(_)) {
            f.text(" ")?;
        }

        if self.auto_keyword.is_some() {
            f.text("auto ")?;
        }

        f.text("trait ")?;
        self.ident.format(f)?;
        self.generics.format(f)?;

        if !self.supertraits.is_empty() {
            f.text(": ")?;
            self.supertraits.format(f)?;
        }

        f.text(" {")?;
        f.indent(|f| {
            for item in &self.items.inner {
                f.hard_break()?;
                item.format(f)?;
            }

            Ok(())
        })?;

        if !self.items.inner.is_empty() {
            f.hard_break()?;
        }

        f.text("}")
    }
}

impl Format for ItemTraitAlias {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.vis.format(f)?;

        if !matches!(self.vis, moxy_ast::Visibility::Inherited) {
            f.text(" ")?;
        }

        f.text("trait ")?;
        self.ident.format(f)?;
        self.generics.format(f)?;
        f.text(" = ")?;
        self.bounds.format(f)?;
        f.text(";")
    }
}

impl Format for ItemImpl {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.defaultness.format(f)?;

        if matches!(self.defaultness, moxy_ast::Defaultness::Default(_)) {
            f.text(" ")?;
        }

        self.unsafety.format(f)?;

        if matches!(self.unsafety, moxy_ast::Unsafety::Unsafe(_)) {
            f.text(" ")?;
        }

        f.text("impl")?;
        self.generics.format(f)?;
        f.text(" ")?;

        if let Some(trait_ref) = &self.trait_ref {
            trait_ref.format(f)?;
            f.text(" for ")?;
        }

        self.self_ty.format(f)?;

        f.text(" {")?;
        f.indent(|f| {
            for (i, item) in self.items.inner.iter().enumerate() {
                f.hard_break()?;
                if i > 0 {
                    f.hard_break()?;
                }
                item.format(f)?;
            }

            Ok(())
        })?;

        if !self.items.inner.is_empty() {
            f.hard_break()?;
        }

        f.text("}")
    }
}

impl Format for ItemTypeAlias {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.vis.format(f)?;

        if !matches!(self.vis, moxy_ast::Visibility::Inherited) {
            f.text(" ")?;
        }

        f.text("type ")?;
        self.ident.format(f)?;
        self.generics.format(f)?;
        f.text(" = ")?;
        self.ty.format(f)?;
        f.text(";")
    }
}

impl Format for ItemConst {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.vis.format(f)?;

        if !matches!(self.vis, moxy_ast::Visibility::Inherited) {
            f.text(" ")?;
        }

        f.text("const ")?;
        self.ident.format(f)?;
        self.generics.format(f)?;
        f.text(": ")?;
        self.ty.format(f)?;
        f.text(" = ")?;
        self.expr.format(f)?;
        f.text(";")
    }
}

impl Format for ItemStatic {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.vis.format(f)?;

        if !matches!(self.vis, moxy_ast::Visibility::Inherited) {
            f.text(" ")?;
        }

        f.text("static ")?;
        self.mutability.format(f)?;

        if matches!(self.mutability, moxy_ast::Mutability::Mutable(_)) {
            f.text(" ")?;
        }

        self.ident.format(f)?;
        f.text(": ")?;
        self.ty.format(f)?;
        f.text(" = ")?;
        self.expr.format(f)?;
        f.text(";")
    }
}

impl Format for ItemMacro {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.call.format(f)?;

        if self.semi_punct.is_some() {
            f.text(";")?;
        }

        Ok(())
    }
}

impl Format for ItemMacroRules {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("macro_rules! ")?;
        self.ident.format(f)?;
        f.text(" { ")?;
        f.text(self.body.stream())?;
        f.text(" }")
    }
}

impl Format for ItemForeignMod {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.unsafety.format(f)?;

        if matches!(self.unsafety, moxy_ast::Unsafety::Unsafe(_)) {
            f.text(" ")?;
        }

        self.abi.format(f)?;
        f.text(" {")?;
        f.indent(|f| {
            for (i, item) in self.items.inner.iter().enumerate() {
                f.hard_break()?;
                if i > 0 {
                    f.hard_break()?;
                }
                item.format(f)?;
            }

            Ok(())
        })?;

        if !self.items.inner.is_empty() {
            f.hard_break()?;
        }

        f.text("}")
    }
}

// ── ImplItem ──────────────────────────────────────────────────────────────────

impl Format for ImplItem {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Fn(v) => v.format(f),
            Self::Const(v) => v.format(f),
            Self::Type(v) => v.format(f),
            Self::Macro(v) => v.format(f),
        }
    }
}

impl Format for ImplItemFn {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.vis.format(f)?;

        if !matches!(self.vis, moxy_ast::Visibility::Inherited) {
            f.text(" ")?;
        }

        self.defaultness.format(f)?;

        if matches!(self.defaultness, moxy_ast::Defaultness::Default(_)) {
            f.text(" ")?;
        }

        self.sig.format(f)?;
        f.text(" ")?;
        self.body.format(f)
    }
}

impl Format for ImplItemConst {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.vis.format(f)?;

        if !matches!(self.vis, moxy_ast::Visibility::Inherited) {
            f.text(" ")?;
        }

        self.defaultness.format(f)?;

        if matches!(self.defaultness, moxy_ast::Defaultness::Default(_)) {
            f.text(" ")?;
        }

        f.text("const ")?;
        self.ident.format(f)?;
        self.generics.format(f)?;
        f.text(": ")?;
        self.ty.format(f)?;
        f.text(" = ")?;
        self.expr.format(f)?;
        f.text(";")
    }
}

impl Format for ImplItemType {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.vis.format(f)?;

        if !matches!(self.vis, moxy_ast::Visibility::Inherited) {
            f.text(" ")?;
        }

        self.defaultness.format(f)?;

        if matches!(self.defaultness, moxy_ast::Defaultness::Default(_)) {
            f.text(" ")?;
        }

        f.text("type ")?;
        self.ident.format(f)?;
        self.generics.format(f)?;
        f.text(" = ")?;
        self.ty.format(f)?;
        f.text(";")
    }
}

impl Format for ImplItemMacro {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.mac.format(f)?;

        if self.semi.is_some() {
            f.text(";")?;
        }

        Ok(())
    }
}

// ── TraitItem ─────────────────────────────────────────────────────────────────

impl Format for TraitItem {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Fn(v) => v.format(f),
            Self::Const(v) => v.format(f),
            Self::Type(v) => v.format(f),
            Self::Macro(v) => v.format(f),
        }
    }
}

impl Format for TraitItemFn {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.sig.format(f)?;

        if let Some(body) = &self.body {
            f.text(" ")?;
            body.format(f)?;
        } else {
            f.text(";")?;
        }

        Ok(())
    }
}

impl Format for TraitItemConst {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("const ")?;
        self.ident.format(f)?;
        self.generics.format(f)?;
        f.text(": ")?;
        self.ty.format(f)?;

        if let Some((_, default)) = &self.default {
            f.text(" = ")?;
            default.format(f)?;
        }

        f.text(";")
    }
}

impl Format for TraitItemType {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("type ")?;
        self.ident.format(f)?;
        self.generics.format(f)?;

        if !self.bounds.is_empty() {
            f.text(": ")?;
            self.bounds.format(f)?;
        }

        if let Some((_, default)) = &self.default {
            f.text(" = ")?;
            default.format(f)?;
        }

        f.text(";")
    }
}

impl Format for TraitItemMacro {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.mac.format(f)?;
        f.text(";")?;
        Ok(())
    }
}

// ── ForeignItem ───────────────────────────────────────────────────────────────

impl Format for ForeignItem {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Fn(v) => v.format(f),
            Self::Static(v) => v.format(f),
            Self::Type(v) => v.format(f),
            Self::Macro(v) => v.format(f),
        }
    }
}

impl Format for ForeignItemFn {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.vis.format(f)?;

        if !matches!(self.vis, moxy_ast::Visibility::Inherited) {
            f.text(" ")?;
        }

        self.sig.format(f)?;
        f.text(";")
    }
}

impl Format for ForeignItemStatic {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.vis.format(f)?;

        if !matches!(self.vis, moxy_ast::Visibility::Inherited) {
            f.text(" ")?;
        }

        f.text("static ")?;
        self.mutability.format(f)?;

        if matches!(self.mutability, moxy_ast::Mutability::Mutable(_)) {
            f.text(" ")?;
        }

        self.ident.format(f)?;
        f.text(": ")?;
        self.ty.format(f)?;
        f.text(";")
    }
}

impl Format for ForeignItemType {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.vis.format(f)?;

        if !matches!(self.vis, moxy_ast::Visibility::Inherited) {
            f.text(" ")?;
        }

        f.text("type ")?;
        self.ident.format(f)?;
        self.generics.format(f)?;
        f.text(";")
    }
}

impl Format for ForeignItemMacro {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.mac.format(f)?;

        if self.semi.is_some() {
            f.text(";")?;
        }

        Ok(())
    }
}
