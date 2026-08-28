use moxy::ast::Item;

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
