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
    fn visit_path(&mut self, node: &'ast Path) {
        self.check("path", node);
        walk_path(self, node);
    }

    fn visit_path_segment(&mut self, node: &'ast PathSegment) {
        self.check("path segment", node);
        walk_path_segment(self, node);
    }

    fn visit_qself(&mut self, node: &'ast QSelf) {
        self.check("qualified self", node);
        walk_qself(self, node);
    }

    fn visit_generics(&mut self, node: &'ast Generics) {
        self.check("generics", node);
        walk_generics(self, node);
    }

    fn visit_where_clause(&mut self, node: &'ast WhereClause) {
        self.check("where clause", node);
        walk_where_clause(self, node);
    }

    fn visit_where_predicate(&mut self, node: &'ast WherePredicate) {
        self.check("where predicate", node);
        walk_where_predicate(self, node);
    }

    fn visit_signature(&mut self, node: &'ast Signature) {
        self.check("signature", node);

        if let Some(abi) = &node.abi {
            self.check("abi", abi);
        }

        walk_signature(self, node);
    }

    fn visit_use_tree(&mut self, node: &'ast UseTree) {
        self.check("use tree", node);
        walk_use_tree(self, node);
    }

    fn visit_use_path(&mut self, node: &'ast UsePath) {
        self.check("use path", node);
        walk_use_path(self, node);
    }

    fn visit_trait_bound(&mut self, node: &'ast TraitBound) {
        self.check("trait bound", node);
        walk_trait_bound(self, node);
    }

    fn visit_type_bound(&mut self, node: &'ast TypeBound) {
        self.check("type bound", node);
        walk_type_bound(self, node);
    }

    fn visit_generic_argument(&mut self, node: &'ast GenericArgument) {
        self.check("generic argument", node);
        walk_generic_argument(self, node);
    }

    fn visit_fn_param(&mut self, node: &'ast FnParam) {
        self.check("parameter", node);
        walk_fn_param(self, node);
    }

    fn visit_field_value(&mut self, node: &'ast FieldValue) {
        self.check("field value", node);
        walk_field_value(self, node);
    }

    fn visit_match_arm(&mut self, node: &'ast MatchArm) {
        self.check("match arm", node);
        walk_match_arm(self, node);
    }

    fn visit_attribute(&mut self, node: &'ast Attribute) {
        self.check("attribute", node);
        walk_attribute(self, node);
    }

    fn visit_receiver(&mut self, node: &'ast Receiver) {
        self.check("receiver", node);
        walk_receiver(self, node);
    }

    fn visit_foreign_item(&mut self, node: &'ast ForeignItem) {
        self.check("foreign item", node);
        walk_foreign_item(self, node);
    }

    fn visit_closure_param(&mut self, node: &'ast ClosureParam) {
        self.check("closure parameter", node);
        walk_closure_param(self, node);
    }

    fn visit_return_type(&mut self, node: &'ast ReturnType) {
        self.check("return type", node);
        walk_return_type(self, node);
    }

    fn visit_visibility(&mut self, node: &'ast Visibility) {
        self.check("visibility", node);
        walk_visibility(self, node);
    }

    fn visit_lifetime(&mut self, node: &'ast Lifetime) {
        self.check("lifetime", node);
        walk_lifetime(self, node);
    }

    fn visit_stmt_block(&mut self, node: &'ast StmtBlock) {
        self.check("block", node);
        walk_stmt_block(self, node);
    }

    fn visit_bare_fn_arg(&mut self, node: &'ast BareFnArg) {
        self.check("function pointer parameter", node);
        walk_bare_fn_arg(self, node);
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

// Every node of every differential corpus source that parses
#[test]
fn every_corpus_node_spans_exactly_its_tokens() {
    let mut check = SpanCheck::default();

    for line in include_str!("differential/corpus.txt").lines() {
        let Some((kind, source)) = line.split_once(": ").filter(|_| !line.starts_with('#')) else {
            continue;
        };
        let source = source.replace("\\n", "\n");
        let before = check.mismatches.len();

        match kind {
            "e" => moxy::parse!(source as Expr).map(|node| check.visit_expr(&node)),
            "b" => moxy::parse!(source as StmtBlock).map(|node| check.visit_stmt_block(&node)),
            "p" => moxy::parse!(source as Pattern).map(|node| check.visit_pattern(&node)),
            "t" => moxy::parse!(source as Type).map(|node| check.visit_type(&node)),
            "i" => moxy::parse!(source as Item).map(|node| check.visit_item(&node)),
            "f" => moxy::parse!(source as File).map(|node| node.items.iter().for_each(|item| check.visit_item(item))),
            _ => panic!("unknown kind `{kind}`"),
        }
        .ok();

        for mismatch in &mut check.mismatches[before..] {
            mismatch.push_str(&format!(" in {line}"));
        }
    }

    assert!(check.mismatches.is_empty(), "{}", check.mismatches.join("\n"));
}

#[test]
fn nested_function_sugar_spans_in_linear_time() {
    // A path, a list of one value, and parenthesized arguments each took the
    // span of their last part twice, which doubled the work per level
    let source = format!("{}T", "Fn(A) -> ".repeat(64));
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::Builder::new()
        .stack_size(64 << 20)
        .spawn(move || {
            let ty: Type = moxy::parse!(source).unwrap();
            sender.send(ty.span().byte_range().len()).unwrap();
        })
        .unwrap();
    let len = receiver
        .recv_timeout(std::time::Duration::from_secs(10))
        .expect("span took longer than 10 seconds");
    assert_eq!(len, 64 * "Fn(A) -> ".len() + 1);
}
