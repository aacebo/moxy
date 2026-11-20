use moxy_token::{Group, MAX_NESTED_NODES, TokenStream};

use super::*;

// Walks a tree no deeper than `MAX_NESTED_NODES` nested nodes, and records
// whether it found one deeper
#[derive(Default)]
struct DepthCheck {
    nodes: usize,
    exceeded: bool,
}

impl DepthCheck {
    fn nest(&mut self, walk: impl FnOnce(&mut Self)) {
        self.nest_by(1, walk);
    }

    // Enters a node that counts as `nodes` nested nodes
    fn nest_by(&mut self, nodes: usize, walk: impl FnOnce(&mut Self)) {
        if self.exceeded || self.nodes + nodes > MAX_NESTED_NODES {
            self.exceeded = true;
            return;
        }

        self.nodes += nodes;
        walk(self);
        self.nodes -= nodes;
    }

    // Unparsed tokens, which a traversal walks as deep as their groups nest,
    // take a level of the parser for each group, so two nodes here
    fn tokens(&mut self, tokens: &TokenStream) {
        self.exceeded = self.exceeded || tokens.nests_deeper_than((MAX_NESTED_NODES - self.nodes) / 2);
    }

    fn group(&mut self, group: &Group) {
        self.tokens(&group.tokens);
    }
}

impl<'ast> Visit<'ast> for DepthCheck {
    fn visit_expr(&mut self, node: &'ast Expr) {
        self.nest(|check| match node {
            Expr::Verbatim(tokens) => check.tokens(tokens),
            _ => walk_expr(check, node),
        });
    }

    fn visit_type(&mut self, node: &'ast Type) {
        self.nest(|check| walk_type(check, node));
    }

    fn visit_pattern(&mut self, node: &'ast Pattern) {
        self.nest(|check| walk_pattern(check, node));
    }

    fn visit_item(&mut self, node: &'ast Item) {
        self.nest(|check| walk_item(check, node));
    }

    fn visit_stmt_block(&mut self, node: &'ast StmtBlock) {
        self.nest(|check| walk_stmt_block(check, node));
    }

    // A use group nests use trees, as in `a::{b::{c}}`, and an associated
    // type bound nests paths, as in `A<B: A<B: C>>`, with none of the nodes
    // above between their levels
    fn visit_use_group(&mut self, node: &'ast UseGroup) {
        self.nest(|check| walk_use_group(check, node));
    }

    // An associated argument nests generic arguments through its own, as in
    // `A<B<C = u8> = u8>`, and its bounds or value, which the parser weighs
    // more, as in `A<B: A<B: C>>`
    fn visit_constraint_argument(&mut self, node: &'ast ConstraintArgument) {
        self.nest_by(ConstraintArgument::NESTED_NODES, |check| {
            check.visit_path_arguments(&node.args);
            check.nest_by(ConstraintArgument::BOUNDS_NESTED_NODES, |check| {
                node.bounds.iter().for_each(|bound| check.visit_type_bound(bound))
            });
        });
    }

    fn visit_assoc_type_argument(&mut self, node: &'ast AssocTypeArgument) {
        self.nest_by(AssocTypeArgument::NESTED_NODES, |check| {
            check.visit_path_arguments(&node.args);
            check.nest_by(AssocTypeArgument::TYPE_NESTED_NODES, |check| check.visit_type(&node.ty));
        });
    }

    fn visit_assoc_const_argument(&mut self, node: &'ast AssocConstArgument) {
        self.nest_by(AssocConstArgument::NESTED_NODES, |check| {
            check.visit_path_arguments(&node.args);
            check.nest_by(AssocConstArgument::EXPR_NESTED_NODES, |check| check.visit_expr(&node.expr));
        });
    }

    // The parser holds a use path in one chain, but a tree built in code can
    // nest a chain in a chain
    fn visit_use_path_chain(&mut self, node: &'ast UsePathChain) {
        self.nest(|check| walk_use_path_chain(check, node));
    }

    fn visit_macro_call(&mut self, node: &'ast MacroCall) {
        self.group(&node.body);
        walk_macro_call(self, node);
    }

    fn visit_item_macro_rules(&mut self, node: &'ast ItemMacroRules) {
        self.group(&node.body);
        walk_item_macro_rules(self, node);
    }

    fn visit_item_decl_macro(&mut self, node: &'ast ItemDeclMacro) {
        if let Some(params) = &node.params {
            self.group(params);
        }

        self.group(&node.body);
        walk_item_decl_macro(self, node);
    }

    fn visit_meta_content(&mut self, node: &'ast MetaContent) {
        match node {
            MetaContent::Unit => {}
            MetaContent::List(group) => self.group(group),
            MetaContent::Expr { expr, .. } => self.tokens(expr),
        }
    }
}

macro_rules! depth_checks {
    ($($ty:ty => $visit:ident;)*) => {$(
        impl $ty {
            /// Whether the tree nests past [`MAX_DEPTH`](moxy_token::MAX_DEPTH), as a tree built in
            /// code can: whether the expressions, types, patterns, items,
            /// blocks, use trees, associated arguments, and token groups that
            /// nest in one another count past [`MAX_NESTED_NODES`], a limit
            /// the parser's trees are tested to stay within. A tree within
            /// it runs every traversal on the stack `MAX_DEPTH` is sized
            /// for, as tested for the shapes listed in [`MAX_NESTED_NODES`].
            /// The check stops descending once it exceeds the limit, so it
            /// runs on trees nested to any depth.
            pub fn nests_past_max_depth(&self) -> bool {
                let mut check = DepthCheck::default();
                check.$visit(self);
                check.exceeded
            }
        }
    )*};
}

depth_checks! {
    Item => visit_item;
    Expr => visit_expr;
    Type => visit_type;
    Pattern => visit_pattern;
    Stmt => visit_stmt;
    StmtBlock => visit_stmt_block;
}

impl File {
    /// Whether the file nests past [`MAX_DEPTH`](moxy_token::MAX_DEPTH), as
    /// [`Item::nests_past_max_depth`] counts it.
    pub fn nests_past_max_depth(&self) -> bool {
        let mut check = DepthCheck::default();
        check.visit_attributes(&self.attrs);
        self.items.iter().for_each(|item| check.visit_item(item));
        check.exceeded
    }
}
