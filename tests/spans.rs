use moxy::ast::*;
use moxy::token::{Spanner, ToTokenStream};

// Collects every node whose span differs from the span of its emitted tokens
#[derive(Default)]
struct SpanCheck {
    mismatches: Vec<String>,
}

impl SpanCheck {
    fn check(&mut self, kind: &str, node: &(impl Spanner + ToTokenStream)) {
        let tokens = node.to_token_stream();

        if tokens.is_empty() {
            return;
        }

        let (actual, expected) = (node.span().byte_range(), tokens.span().byte_range());

        if actual != expected {
            self.mismatches
                .push(format!("{kind} `{tokens}`: {actual:?}, tokens at {expected:?}"));
        }
    }
}

impl<'ast> Visit<'ast> for SpanCheck {
    fn visit_expr(&mut self, node: &'ast Expr) {
        self.check("expression", node);
        walk_expr(self, node);
    }

    fn visit_pattern(&mut self, node: &'ast Pattern) {
        self.check("pattern", node);
        walk_pattern(self, node);
    }

    fn visit_type(&mut self, node: &'ast Type) {
        self.check("type", node);
        walk_type(self, node);
    }

    fn visit_item(&mut self, node: &'ast Item) {
        self.check("item", node);
        walk_item(self, node);
    }

    fn visit_stmt(&mut self, node: &'ast Stmt) {
        self.check("statement", node);
        walk_stmt(self, node);
    }

    fn visit_impl_item(&mut self, node: &'ast ImplItem) {
        self.check("impl item", node);
        walk_impl_item(self, node);
    }

    fn visit_trait_item(&mut self, node: &'ast TraitItem) {
        self.check("trait item", node);
        walk_trait_item(self, node);
    }

    fn visit_variant(&mut self, node: &'ast Variant) {
        self.check("variant", node);
        walk_variant(self, node);
    }

    fn visit_field_def(&mut self, node: &'ast Field) {
        self.check("field", node);
        walk_field_def(self, node);
    }

    fn visit_generic_param(&mut self, node: &'ast GenericParam) {
        self.check("generic parameter", node);
        walk_generic_param(self, node);
    }
}

#[test]
fn every_node_spans_exactly_its_tokens() {
    let source = r#"
        use std::{fmt, io::Write as _};
        struct Unit;
        struct Tuple(pub u8, std::vec::Vec<u8>);
        pub(crate) struct Named<'a, T: Clone = u8, const N: usize = 3> where T: Copy { a: &'a T, b: [u8; N] }
        enum E<T> { A, B(T), C { x: u8 } = 1 }
        union U { a: u8, b: u16 }
        type Alias<T> = Vec<T>;
        type Callback = Box<dyn Fn::(u8) -> u8 + Send>;
        const C: u8 = 1 + 2;
        static S: &str = "s";
        trait Tr: Clone { const K: u8; type T; fn f(&self) -> u8 { 1 } fn g(); }
        impl<T> Tr for Named<'_, T> { const K: u8 = 1; type T = u8; fn f(&self) -> u8 { self.a.len() as u8 } }
        mod m { fn inner() {} }
        extern "C" { fn ext(x: i32) -> i32; }
        macro_rules! mac { () => {} }
        fn f(a: u8, (b, _): (u8, u8)) -> Option<u8> {
            let x = zz + (a * b) - -a;
            let Some(y) = f2(x) else { return None };
            let s = S { a: 1, ..Default::default() };
            let c = |v: u8| -> u8 { v + 1 };
            #[allow(unused)] let r = &mut x[0..2];
            if let 1..=9 | 20 = x && y > 0 { y.method::<u8>(1)?.field } else { loop { break 'l; } };
            match x { 0 => (), n @ 1..=9 => {} _ if a => continue, _ => {} }
            for i in 0.. { while i < 3 { i += 1; } }
            async { a.await };
            unsafe { *p };
            m!(a);
            vec![1].len();
            x as u64
        }
    "#;

    let file: File = moxy::parse!(source).unwrap();
    let mut check = SpanCheck::default();
    for item in &file.items {
        check.visit_item(item);
    }
    assert!(check.mismatches.is_empty(), "{}", check.mismatches.join("\n"));
}

#[test]
fn a_group_closes_at_its_closing_delimiter() {
    let expression: Expr = moxy::parse!("zz + (a * b)").unwrap();
    let paren = expression.as_binary().unwrap().right.as_paren().unwrap();
    let (open, close) = (paren.content.open().byte_range(), paren.content.close().byte_range());
    assert_eq!(close.start - open.start, "(a * b".len());
    assert_eq!(close.len(), 1);
}
