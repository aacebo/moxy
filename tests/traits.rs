use moxy::ast::{Item, TraitItem};
use moxy::token::ToTokenStream;

#[test]
fn traits_preserve_supertraits_and_every_member_family() {
    let item: Item = moxy::parse!(
        "pub trait Service<T>:Send{const LIMIT:usize=1;type Output:Clone;fn call(&self,value:T)->Self::Output;invoke!();}"
    )
    .unwrap();
    let item_trait = item.as_trait().unwrap();
    assert_eq!(item_trait.ident.text(), "Service");
    assert_eq!(item_trait.generics.params.len(), 1);
    debug_assert_eq!(item_trait.items.inner.len(), 4, "{item_trait:#?}");
    assert_eq!(
        moxy::fmt!(&item).unwrap(),
        "pub trait Service<T>: Send {\n\tconst LIMIT: usize = 1;\n\ttype Output: Clone;\n\tfn call(&self, value: T) -> Self::Output;\n\tinvoke!();\n}"
    );
}

#[test]
fn trait_method_signatures_complete_the_syntax_pipeline() {
    let item: Item = moxy::parse!("pub trait Service<T>: Send { fn call(&self, value: T) -> T; }").unwrap();
    let item_trait = item.as_trait().unwrap();
    assert_eq!(item_trait.ident.text(), "Service");
    assert_eq!(item_trait.generics.params.len(), 1);
    assert_eq!(item_trait.supertraits.len(), 1);
    assert_eq!(item_trait.items.inner.len(), 1);
    assert!(item_trait.items.inner[0].is_fn());
    assert_eq!(
        moxy::fmt!(&item).unwrap(),
        "pub trait Service<T>: Send {\n\tfn call(&self, value: T) -> T;\n}"
    );
}

#[test]
fn trait_aliases_render_bounds_with_canonical_spacing() {
    let item: Item = moxy::parse!("pub trait Portable=Clone+Send+'static;").unwrap();
    let alias = item.as_trait_alias().unwrap();
    assert_eq!(alias.ident.text(), "Portable");
    assert_eq!(alias.bounds.len(), 3);
    assert_eq!(moxy::fmt!(&item).unwrap(), "pub trait Portable = Clone + Send + 'static;");
}

#[test]
fn trait_where_clauses_follow_the_supertraits() {
    for (source, tokens, formatted) in [
        (
            "trait A<T> where T: Clone {}",
            "trait A < T > where T : Clone {}",
            "trait A<T>\nwhere\nT: Clone {}",
        ),
        (
            "trait A: Send where Self: Sized { fn a(&self); }",
            "trait A : Send where Self : Sized {fn a (& self) ;}",
            "trait A: Send\nwhere\nSelf: Sized {\n\tfn a(&self);\n}",
        ),
    ] {
        let item: Item = moxy::parse!(source).unwrap();
        assert!(item.as_trait().unwrap().generics.where_clause.is_some(), "`{source}`");
        assert_eq!(item.to_token_stream().to_string(), tokens);
        assert_eq!(moxy::fmt!(&item).unwrap(), formatted);
    }
}

#[test]
fn associated_types_keep_a_where_clause_after_their_bounds() {
    let item: Item = moxy::parse!("trait A { type Gat<'a>: Iterator<Item = &'a u8> where Self: 'a; }").unwrap();
    let TraitItem::Type(ty) = &item.as_trait().unwrap().items.inner[0] else {
        panic!("expected an associated type");
    };
    assert!(ty.generics.where_clause.is_some());
    assert_eq!(
        item.to_token_stream().to_string(),
        "trait A {type Gat < 'a > : Iterator < Item = & 'a u8 > where Self : 'a ;}"
    );
}

#[test]
fn associated_type_where_clauses_are_formatted() {
    let item: Item = moxy::parse!("trait A { type Gat<'a>: Iterator<Item = &'a u8> where Self: 'a; }").unwrap();
    assert_eq!(
        moxy::fmt!(&item).unwrap(),
        "trait A {\n\ttype Gat<'a>: Iterator<Item = &'a u8>\n\twhere\n\tSelf: 'a;\n}"
    );
}

#[test]
fn brace_delimited_macro_items_need_no_semicolon() {
    let item: Item = moxy::parse!("trait A { fn f(&self) {} declare! { x } fn g(&self); }").unwrap();
    let items = &item.as_trait().unwrap().items.inner;
    assert_eq!(items.len(), 3);
    assert!(items[1].is_macro());
    assert_eq!(
        item.to_token_stream().to_string(),
        "trait A {fn f (& self) {} declare ! {x} fn g (& self) ;}"
    );
    assert_eq!(
        moxy::fmt!(&item).unwrap(),
        "trait A {\n\tfn f(&self) {}\n\tdeclare!{x}\n\tfn g(&self);\n}"
    );
}

#[test]
fn parenthesized_macro_items_need_a_semicolon() {
    let error = moxy::parse!("trait A { m!() }" as Item).err().unwrap();
    assert_eq!(error.message(), "expected `;` after a macro item in parentheses or brackets");
    let error = moxy::parse!("impl A { m![] }" as Item).err().unwrap();
    assert_eq!(error.message(), "expected `;` after a macro item in parentheses or brackets");

    for source in ["trait A { m!(); n! {} }", "impl A { m![]; n! {} }"] {
        let item: Item = moxy::parse!(source).unwrap_or_else(|error| panic!("`{source}`: {error}"));
        assert_eq!(item.to_token_stream().to_string().matches('!').count(), 2);
    }
}

#[test]
fn trait_items_keep_a_visibility_as_rustc_parses_them() {
    let item: Item = moxy::parse!("trait A { pub fn f(); pub(crate) const X: u8; pub type T; }").unwrap();
    assert_eq!(
        item.to_token_stream().to_string(),
        "trait A {pub fn f () ; pub (crate) const X : u8 ; pub type T ;}"
    );
    assert_eq!(
        moxy::fmt!(&item).unwrap(),
        "trait A {\n\tpub fn f();\n\tpub(crate) const X: u8;\n\tpub type T;\n}"
    );
}
