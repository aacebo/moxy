use moxy_ast::{
    Abi, Asyncness, BoundLifetimes, Constness, Crate, Expr, Item, Movability, Pattern, Stmt, Type, Unsafety, Variadic,
};
use moxy_fmt::fmt;

#[test]
fn qualified_expressions_closures_fields_labels_and_verbatim_have_exact_output() {
    for (source, expected) in [
        ("_", "_"),
        ("<T>::value", "<T>::value"),
        ("Point { x: 1, y }", "Point {\n\tx: 1,\n\ty,\n}"),
        ("match value {}", "match value {\n}"),
    ] {
        let expression: Expr = moxy_token::parse!(source).unwrap();
        assert_eq!(fmt!(&expression).unwrap(), expected, "wrong exact output for {source}");
    }

    let verbatim = Expr::Verbatim("custom tokens".parse().unwrap());
    assert_eq!(fmt!(&verbatim).unwrap(), "custom tokens");

    let closure: Expr = moxy_token::parse!("move |x: &'a T| -> U { x }").unwrap();
    let closure = closure.as_primary().unwrap().as_closure().unwrap().clone();
    let mut closure = closure;
    closure.lifetimes = Some(moxy_token::parse!("for<'a>" as BoundLifetimes).unwrap());
    closure.constness = Constness::Const(Default::default());
    closure.movability = Movability::Static(Default::default());
    closure.asyncness = Asyncness::Async(Default::default());
    assert_eq!(
        fmt!(&closure).unwrap(),
        "for<'a> const static async move |x: &'a T| -> U {\n\tx\n}"
    );
}

#[test]
fn advanced_type_and_statement_branches_have_exact_output() {
    for (source, expected) in [
        ("<T>::Assoc", "<T>::Assoc"),
        ("&'a mut T", "&'a mut T"),
        ("*mut T", "*mut T"),
        ("*const T", "*const T"),
    ] {
        let ty: Type = moxy_token::parse!(source).unwrap();
        assert_eq!(fmt!(&ty).unwrap(), expected, "wrong exact output for {source}");
    }

    let mut bare: Type = moxy_token::parse!("fn(name: &'a T) -> U").unwrap();
    let Type::BareFn(bare) = &mut bare else { unreachable!() };
    bare.lifetimes = Some(moxy_token::parse!("for<'a>" as BoundLifetimes).unwrap());
    bare.unsafety = Unsafety::Unsafe(Default::default());
    bare.abi = Some(moxy_token::parse!("extern \"C\"" as Abi).unwrap());
    bare.params.inner.variadic = Some(Variadic {
        attrs: Default::default(),
        name: None,
        dots: Default::default(),
    });
    assert_eq!(fmt!(bare).unwrap(), "for<'a> unsafe extern \"C\" fn(name: &'a T, ...) -> U");
    bare.params.inner.inputs.clear();
    assert_eq!(fmt!(bare).unwrap(), "for<'a> unsafe extern \"C\" fn(...) -> U");

    for (source, expected) in [
        ("let value: Option<T>;", "let value: Option<T>;"),
        (
            "let value = compute() else { return; };",
            "let value = compute() else {\n\treturn;\n};",
        ),
        ("{ work(); value }", "{\n\twork();\n\tvalue\n}"),
        ("const VALUE: usize = 1;", "const VALUE: usize = 1;"),
        ("value", "value"),
        ("value;", "value;"),
        ("call!()", "call!()"),
        ("call!();", "call!();"),
    ] {
        let statement: Stmt = moxy_token::parse!(source).unwrap();
        assert_eq!(fmt!(&statement).unwrap(), expected, "wrong exact output for {source}");
    }
}

#[test]
fn pattern_and_path_optional_branches_have_exact_output() {
    for (source, expected) in [
        ("_", "_"),
        ("..", ".."),
        ("ref mut value @ Some(_)", "ref mut value @ Some(_)"),
        ("Point { x: renamed, y, .. }", "Point {\n\tx: renamed,\n\ty,\n\t..\n}"),
        ("&mut value", "&mut value"),
        ("1..=10", "1..=10"),
        ("box value", "box value"),
        ("const { 1 }", "const {\n\t1\n}"),
    ] {
        let pattern: Pattern = moxy_token::parse!(source).unwrap();
        assert_eq!(fmt!(&pattern).unwrap(), expected, "wrong exact output for {source}");
    }
    let pattern = Pattern::Macro(moxy_token::parse!("macro_call!(tokens)" as moxy_ast::MacroCall).unwrap());
    assert_eq!(fmt!(&pattern).unwrap(), "macro_call!(tokens)");

    for (source, expected) in [
        ("::std::path::Path", "::std::path::Path"),
        ("Fn(A, B) -> C", "Fn(A, B) -> C"),
        (
            "Trait<Item<T> = Vec<T>, N<T> = 4, Item<T>: Clone>",
            "Trait<Item<T> = Vec<T>, N<T> = 4, Item<T>: Clone>",
        ),
    ] {
        let ty: Type = moxy_token::parse!(source).unwrap();
        assert_eq!(fmt!(&ty).unwrap(), expected, "wrong exact path output for {source}");
    }
}

#[test]
fn item_modifiers_members_and_field_shapes_have_exact_output() {
    for (source, expected) in [
        ("extern crate core as rust_core;", "extern crate core as rust_core;"),
        ("pub mod declared;", "pub mod declared;"),
        (
            "pub struct Named<T> where T: Clone { pub value: T, hidden: usize }",
            "pub struct Named<T>\nwhere\nT: Clone {\n\tpub value: T,\n\thidden: usize,\n}",
        ),
        ("pub struct Tuple(pub String, usize);", "pub struct Tuple(pub String, usize);"),
        ("pub struct Unit;", "pub struct Unit;"),
        (
            "enum Choice { Unit, Tuple(u8, u16), Named { value: usize }, Explicit = 4 }",
            "enum Choice {\n\tUnit,\n\tTuple(u8, u16),\n\tNamed {\n\t\tvalue: usize,\n\t},\n\tExplicit = 4,\n}",
        ),
        ("unsafe auto trait Marker {}", "unsafe auto trait Marker {}"),
        ("trait Alias = Clone + Send;", "trait Alias = Clone + Send;"),
        ("unsafe impl !Send for Record {}", "unsafe impl !Send for Record {}"),
        (
            "extern \"C\" { fn call(value: usize) -> usize; static mut VALUE: usize; type Output; invoke!(); }",
            "extern \"C\" {\n\tfn call(value: usize) -> usize;\n\t\n\tstatic mut VALUE: usize;\n\t\n\ttype Output;\n\t\n\tinvoke!();\n}",
        ),
    ] {
        let item: Item = moxy_token::parse!(source).unwrap();
        assert_eq!(fmt!(&item).unwrap(), expected, "wrong exact output for {source}");
    }

    let krate: Crate =
        moxy_token::parse!("pub struct Unit; impl Unit { const VALUE: usize = 1; type Output = usize; fn run() {} call!(); }")
            .unwrap();
    assert_eq!(
        fmt!(&krate).unwrap(),
        "pub struct Unit;\n\nimpl Unit {\n\tconst VALUE: usize = 1;\n\t\n\ttype Output = usize;\n\t\n\tfn run() {}\n\t\n\tcall!();\n}"
    );
}

#[test]
#[ignore]
fn formatter_regressions_require_valid_exact_output_for_labels_qself_and_inline_modules() {
    let mut failures = Vec::new();
    for (source, expected) in [
        (
            "'label: while ready { continue 'label; }",
            "'label: while ready {\n\tcontinue 'label;\n}",
        ),
        (
            "'label: for item in items { break 'label item; }",
            "'label: for item in items {\n\tbreak 'label item;\n}",
        ),
        ("'label: loop { break 'label; }", "'label: loop {\n\tbreak 'label;\n}"),
        ("<T as Trait>::Assoc::value", "<T as Trait>::Assoc::value"),
    ] {
        let expression: Expr = moxy_token::parse!(source).unwrap();
        let actual = fmt!(&expression).unwrap();
        if actual != expected {
            failures.push((source, actual, expected));
        }
    }

    let source = "<T as Trait>::Assoc";
    let ty: Type = moxy_token::parse!(source).unwrap();
    let actual = fmt!(&ty).unwrap();
    if actual != source {
        failures.push((source, actual, source));
    }

    let source = "pub mod inline { pub use root::{self, item as renamed, *}; }";
    let expected = "pub mod inline {\n\tpub use root::{self, item as renamed, *};\n}";
    let item: Item = moxy_token::parse!(source).unwrap();
    let actual = fmt!(&item).unwrap();
    if actual != expected {
        failures.push((source, actual, expected));
    }

    let source = "macro_rules! generated { ($tokens:tt) => { $tokens }; }";
    let item: Item = moxy_token::parse!(source).unwrap();
    let actual = fmt!(&item).unwrap();
    if actual != source {
        failures.push((source, actual, source));
    }

    let source = "#![allow(dead_code)] pub struct Unit;";
    let expected = "#![allow(dead_code)]\n\npub struct Unit;";
    let krate: Crate = moxy_token::parse!(source).unwrap();
    let actual = fmt!(&krate).unwrap();
    if actual != expected {
        failures.push((source, actual, expected));
    }

    assert!(failures.is_empty(), "formatter changed valid Rust syntax: {failures:#?}");
}
