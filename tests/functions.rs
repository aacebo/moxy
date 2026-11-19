use moxy::ast::Item;

#[test]
fn function_signatures_and_bodies_complete_the_syntax_pipeline() {
    let item: Item = moxy::parse!("pub fn add(a:u32,b:u32)->u32{a+b}").unwrap();
    let function = item.as_fn().unwrap();
    assert_eq!(function.sig.ident.text(), "add");
    assert_eq!(function.sig.params.inner.inputs.len(), 2);
    assert!(function.sig.output.is_type());
    assert_eq!(function.body.as_ref().unwrap().stmts.len(), 1);
    assert_eq!(moxy::fmt!(&item).unwrap(), "pub fn add(a: u32, b: u32) -> u32 {\n\ta + b\n}");
}

#[test]
fn async_generic_functions_preserve_bounds_references_and_return_types() {
    let item: Item = moxy::parse!("pub async fn fetch<T:Clone>(url:&str)->Option<T>{None}").unwrap();
    let function = item.as_fn().unwrap();
    assert!(function.sig.asyncness.is_some());
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
    assert!(function.sig.safety.is_some());
    assert!(function.sig.abi.is_some());
    assert!(function.sig.params.inner.variadic.is_some());
    assert_eq!(
        moxy::fmt!(&item).unwrap(),
        "pub unsafe extern \"C\" fn log(format: *const u8, ...) -> i32 {\n\t0\n}"
    );
}

#[test]
fn parameters_after_a_path_generic_argument_are_kept() {
    let item: Item = moxy::parse!("fn f(x: Vec<a::B>, y: u8) -> u8 { y }").unwrap();
    let function = item.as_fn().unwrap();
    assert_eq!(function.sig.params.inner.inputs.len(), 2);
    assert_eq!(moxy::fmt!(&item).unwrap(), "fn f(x: Vec<a::B>, y: u8) -> u8 {\n\ty\n}");
}

#[test]
fn typed_self_receivers_keep_their_type() {
    for (source, expected) in [
        ("fn f(self: Box<Self>, x: u8) {}", "fn f(self: Box<Self>, x: u8) {}"),
        ("fn f(mut self: Box<Self>) {}", "fn f(mut self: Box<Self>) {}"),
        ("fn f(self: &Self) {}", "fn f(self: &Self) {}"),
        ("fn f(self: std::rc::Rc<Self>) {}", "fn f(self: std::rc::Rc<Self>) {}"),
    ] {
        let item: Item = moxy::parse!(source).unwrap_or_else(|error| panic!("`{source}`: {error}"));
        let function = item.as_fn().unwrap();
        let moxy::ast::FnParam::Receiver(receiver) = function.sig.params.inner.inputs.first().unwrap() else {
            panic!("`{source}`: the first parameter is not a receiver");
        };
        assert!(receiver.ty.is_some(), "`{source}`");
        assert_eq!(moxy::fmt!(&item).unwrap(), expected);
    }
}
