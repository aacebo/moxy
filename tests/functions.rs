use moxy::ast::Item;

#[test]
fn function_signatures_and_bodies_complete_the_syntax_pipeline() {
    let item: Item = moxy::parse!("pub fn add(a:u32,b:u32)->u32{a+b}").unwrap();
    let function = item.as_fn().unwrap();
    assert_eq!(function.sig.ident.text(), "add");
    assert_eq!(function.sig.params.inner.inputs.len(), 2);
    assert!(function.sig.output.is_type());
    assert_eq!(function.body.stmts.len(), 1);
    assert_eq!(moxy::fmt!(&item).unwrap(), "pub fn add(a: u32, b: u32) -> u32 {\n\ta + b\n}");
}

#[test]
fn async_generic_functions_preserve_bounds_references_and_return_types() {
    let item: Item = moxy::parse!("pub async fn fetch<T:Clone>(url:&str)->Option<T>{None}").unwrap();
    let function = item.as_fn().unwrap();
    assert!(matches!(function.sig.asyncness, moxy::ast::Asyncness::Async(_)));
    assert_eq!(function.sig.generics.params.len(), 1);
    assert_eq!(function.sig.params.inner.inputs.len(), 1);
    assert_eq!(
        moxy::fmt!(&item).unwrap(),
        "pub async fn fetch<T: Clone>(url: &str) -> Option<T> {\n\tNone\n}"
    );
}

#[test]
fn unsafe_extern_variadic_functions_render_exactly() {
    let item: Item = moxy::parse!("pub unsafe extern \"C\" fn log(format:*const u8,...)->i32{0}").unwrap();
    let function = item.as_fn().unwrap();
    assert!(matches!(function.sig.unsafety, moxy::ast::Unsafety::Unsafe(_)));
    assert!(function.sig.abi.is_some());
    assert!(function.sig.params.inner.variadic.is_some());
    assert_eq!(
        moxy::fmt!(&item).unwrap(),
        "pub unsafe extern \"C\" fn log(format: *const u8, ...) -> i32 {\n\t0\n}"
    );
}
