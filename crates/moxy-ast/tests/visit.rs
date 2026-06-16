use moxy_ast::expr::{ExprBinary, ExprCall};
use moxy_ast::item::ItemFn;
use moxy_ast::visit::{Visit, VisitMut, walk_expr_binary, walk_expr_call, walk_path_segment, walk_path_segment_mut};
use moxy_ast::{Expr, PathSegment};
use moxy_token::{Ident, ToTokenStream};

// ---- count visitor (immutable) -----------------------------------------

#[derive(Default)]
struct Counter {
    binary: usize,
    calls: usize,
}

impl<'ast> Visit<'ast> for Counter {
    fn visit_expr_binary(&mut self, node: &'ast ExprBinary) {
        self.binary += 1;
        walk_expr_binary(self, node);
    }

    fn visit_expr_call(&mut self, node: &'ast ExprCall) {
        self.calls += 1;
        walk_expr_call(self, node);
    }
}

#[test]
fn counts_nested_binary_exprs() {
    let e = moxy_token::parse!("a + b * c" as Expr).unwrap();
    let mut c = Counter::default();
    c.visit_expr(&e);
    assert_eq!(c.binary, 2, "expected `a + (b * c)` to have 2 binary nodes");
}

#[test]
fn counts_nested_calls() {
    let e = moxy_token::parse!("f(g(h(x)))" as Expr).unwrap();
    let mut c = Counter::default();
    c.visit_expr(&e);
    assert_eq!(c.calls, 3, "expected 3 nested call expressions");
}

// ---- collect visitor: reaches sig + body via path segments -------------

#[derive(Default)]
struct IdentCollector {
    names: Vec<String>,
}

impl<'ast> Visit<'ast> for IdentCollector {
    fn visit_path_segment(&mut self, node: &'ast PathSegment) {
        self.names.push(node.ident.to_string());
        walk_path_segment(self, node);
    }
}

#[test]
fn collects_idents_from_fn_sig_and_body() {
    let f = moxy_token::parse!("fn foo(x: Bar) -> Baz { qux(x) }" as ItemFn).unwrap();
    let mut c = IdentCollector::default();
    c.visit_item_fn(&f);
    // Bar (param ty), Baz (return ty), qux (call) are all path segments.
    for expected in ["Bar", "Baz", "qux"] {
        assert!(
            c.names.iter().any(|n| n == expected),
            "expected to reach `{expected}` segment; got {:?}",
            c.names
        );
    }
}

// ---- prune: not calling the walker stops descent -----------------------

#[derive(Default)]
struct Pruner {
    fns_seen: usize,
    binary_seen: usize,
}

impl<'ast> Visit<'ast> for Pruner {
    fn visit_item_fn(&mut self, _node: &'ast ItemFn) {
        self.fns_seen += 1;
        // Intentionally do NOT call walk_item_fn -> prune the subtree.
    }

    fn visit_expr_binary(&mut self, node: &'ast ExprBinary) {
        self.binary_seen += 1;
        walk_expr_binary(self, node);
    }
}

#[test]
fn pruning_stops_descent() {
    let f = moxy_token::parse!("fn foo() { a + b }" as ItemFn).unwrap();
    let mut p = Pruner::default();
    p.visit_item_fn(&f);
    assert_eq!(p.fns_seen, 1);
    assert_eq!(p.binary_seen, 0, "body should not be visited when fn is pruned");
}

// ---- rewrite (VisitMut): mutate children in place ----------------------

struct Renamer;

impl VisitMut for Renamer {
    fn visit_path_segment_mut(&mut self, node: &mut PathSegment) {
        if node.ident.to_string() == "a" {
            node.ident = Ident::new("z");
        }
        walk_path_segment_mut(self, node);
    }
}

#[test]
fn rewrite_renames_idents() {
    let mut e = moxy_token::parse!("a + a * b" as Expr).unwrap();
    Renamer.visit_expr_mut(&mut e);
    let rendered = e.to_token_stream().to_string();
    assert!(!rendered.contains('a'), "all `a` idents should be renamed: {rendered}");
    assert_eq!(rendered.matches('z').count(), 2, "two `a`s became `z`: {rendered}");
    assert!(rendered.contains('b'), "`b` should be untouched: {rendered}");
}
