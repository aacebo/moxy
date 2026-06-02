use crate::parse::{ParseError, ParseStream};
use crate::token::ToTokens;
use crate::{Parse, TokenStream};

pub mod binary;
pub mod block;
pub mod jump;
pub mod postfix;
pub mod primary;
pub mod unary;

pub use binary::*;
pub use block::*;
pub use jump::*;
pub use postfix::*;
pub use primary::*;
pub use unary::*;

#[doc = "A Rust expression. The primary recursive node covering all expression forms."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Expr {
    Unary(UnaryExpr),
    Binary(BinaryExpr),
    Postfix(PostfixExpr),
    Block(BlockExpr),
    Jump(JumpExpr),
    Primary(PrimaryExpr),
    Infer,
    Verbatim(TokenStream),
}

impl From<UnaryExpr> for Expr {
    fn from(v: UnaryExpr) -> Self {
        Expr::Unary(v)
    }
}

impl From<BinaryExpr> for Expr {
    fn from(v: BinaryExpr) -> Self {
        Expr::Binary(v)
    }
}

impl From<PostfixExpr> for Expr {
    fn from(v: PostfixExpr) -> Self {
        Expr::Postfix(v)
    }
}

impl From<BlockExpr> for Expr {
    fn from(v: BlockExpr) -> Self {
        Expr::Block(v)
    }
}

impl From<JumpExpr> for Expr {
    fn from(v: JumpExpr) -> Self {
        Expr::Jump(v)
    }
}

impl From<PrimaryExpr> for Expr {
    fn from(v: PrimaryExpr) -> Self {
        Expr::Primary(v)
    }
}

impl From<ExprReference> for Expr {
    fn from(value: ExprReference) -> Self {
        Expr::Unary(UnaryExpr::from(value))
    }
}

impl From<ExprUnary> for Expr {
    fn from(value: ExprUnary) -> Self {
        Expr::Unary(UnaryExpr::from(value))
    }
}

impl From<ExprCast> for Expr {
    fn from(value: ExprCast) -> Self {
        Expr::Unary(UnaryExpr::from(value))
    }
}

impl From<ExprTry> for Expr {
    fn from(value: ExprTry) -> Self {
        Expr::Unary(UnaryExpr::from(value))
    }
}

impl From<ExprBinary> for Expr {
    fn from(value: ExprBinary) -> Self {
        Expr::Binary(BinaryExpr::from(value))
    }
}

impl From<ExprAssign> for Expr {
    fn from(value: ExprAssign) -> Self {
        Expr::Binary(BinaryExpr::from(value))
    }
}

impl From<ExprAssignOp> for Expr {
    fn from(value: ExprAssignOp) -> Self {
        Expr::Binary(BinaryExpr::from(value))
    }
}

impl From<ExprRange> for Expr {
    fn from(value: ExprRange) -> Self {
        Expr::Binary(BinaryExpr::from(value))
    }
}

impl From<ExprType> for Expr {
    fn from(value: ExprType) -> Self {
        Expr::Binary(BinaryExpr::from(value))
    }
}

impl From<ExprCall> for Expr {
    fn from(value: ExprCall) -> Self {
        Expr::Postfix(PostfixExpr::from(value))
    }
}

impl From<ExprMethodCall> for Expr {
    fn from(value: ExprMethodCall) -> Self {
        Expr::Postfix(PostfixExpr::from(value))
    }
}

impl From<ExprField> for Expr {
    fn from(value: ExprField) -> Self {
        Expr::Postfix(PostfixExpr::from(value))
    }
}

impl From<ExprIndex> for Expr {
    fn from(value: ExprIndex) -> Self {
        Expr::Postfix(PostfixExpr::from(value))
    }
}

impl From<ExprAwait> for Expr {
    fn from(value: ExprAwait) -> Self {
        Expr::Postfix(PostfixExpr::from(value))
    }
}

impl From<ExprBrace> for Expr {
    fn from(value: ExprBrace) -> Self {
        Expr::Block(BlockExpr::from(value))
    }
}

impl From<ExprIf> for Expr {
    fn from(value: ExprIf) -> Self {
        Expr::Block(BlockExpr::from(value))
    }
}

impl From<ExprWhile> for Expr {
    fn from(value: ExprWhile) -> Self {
        Expr::Block(BlockExpr::from(value))
    }
}

impl From<ExprForLoop> for Expr {
    fn from(value: ExprForLoop) -> Self {
        Expr::Block(BlockExpr::from(value))
    }
}

impl From<ExprLoop> for Expr {
    fn from(value: ExprLoop) -> Self {
        Expr::Block(BlockExpr::from(value))
    }
}

impl From<ExprMatch> for Expr {
    fn from(value: ExprMatch) -> Self {
        Expr::Block(BlockExpr::from(value))
    }
}

impl From<ExprAsync> for Expr {
    fn from(value: ExprAsync) -> Self {
        Expr::Block(BlockExpr::from(value))
    }
}

impl From<ExprUnsafe> for Expr {
    fn from(value: ExprUnsafe) -> Self {
        Expr::Block(BlockExpr::from(value))
    }
}

impl From<ExprConst> for Expr {
    fn from(value: ExprConst) -> Self {
        Expr::Block(BlockExpr::from(value))
    }
}

impl From<ExprTryBlock> for Expr {
    fn from(value: ExprTryBlock) -> Self {
        Expr::Block(BlockExpr::from(value))
    }
}

impl From<ExprReturn> for Expr {
    fn from(value: ExprReturn) -> Self {
        Expr::Jump(JumpExpr::from(value))
    }
}

impl From<ExprBreak> for Expr {
    fn from(value: ExprBreak) -> Self {
        Expr::Jump(JumpExpr::from(value))
    }
}

impl From<ExprContinue> for Expr {
    fn from(value: ExprContinue) -> Self {
        Expr::Jump(JumpExpr::from(value))
    }
}

impl From<ExprYield> for Expr {
    fn from(value: ExprYield) -> Self {
        Expr::Jump(JumpExpr::from(value))
    }
}

impl From<ExprLit> for Expr {
    fn from(value: ExprLit) -> Self {
        Expr::Primary(PrimaryExpr::from(value))
    }
}

impl From<ExprPath> for Expr {
    fn from(value: ExprPath) -> Self {
        Expr::Primary(PrimaryExpr::from(value))
    }
}

impl From<ExprStruct> for Expr {
    fn from(value: ExprStruct) -> Self {
        Expr::Primary(PrimaryExpr::from(value))
    }
}

impl From<ExprClosure> for Expr {
    fn from(value: ExprClosure) -> Self {
        Expr::Primary(PrimaryExpr::from(value))
    }
}

impl From<ExprTuple> for Expr {
    fn from(value: ExprTuple) -> Self {
        Expr::Primary(PrimaryExpr::from(value))
    }
}

impl From<ExprArray> for Expr {
    fn from(value: ExprArray) -> Self {
        Expr::Primary(PrimaryExpr::from(value))
    }
}

impl From<ExprRepeat> for Expr {
    fn from(value: ExprRepeat) -> Self {
        Expr::Primary(PrimaryExpr::from(value))
    }
}

impl From<ExprLet> for Expr {
    fn from(value: ExprLet) -> Self {
        Expr::Primary(PrimaryExpr::from(value))
    }
}

impl From<ExprParen> for Expr {
    fn from(value: ExprParen) -> Self {
        Expr::Primary(PrimaryExpr::from(value))
    }
}

impl From<ExprGroup> for Expr {
    fn from(value: ExprGroup) -> Self {
        Expr::Primary(PrimaryExpr::from(value))
    }
}

impl From<ExprMacro> for Expr {
    fn from(value: ExprMacro) -> Self {
        Expr::Primary(PrimaryExpr::from(value))
    }
}

impl Parse for Expr {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        parse_expr(stream, true)
    }
}

impl ToTokens for Expr {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            Expr::Unary(v) => v.to_tokens(t),
            Expr::Binary(v) => v.to_tokens(t),
            Expr::Postfix(v) => v.to_tokens(t),
            Expr::Block(v) => v.to_tokens(t),
            Expr::Jump(v) => v.to_tokens(t),
            Expr::Primary(v) => v.to_tokens(t),
            Expr::Infer => {}
            Expr::Verbatim(v) => v.to_tokens(t),
        }
    }
}

// Parser

pub fn parse_expr(stream: &mut ParseStream, allow_struct: bool) -> Result<Expr, ParseError> {
    use crate::ast::precedence::Precedence;
    let lhs = unary::UnaryExpr::parse_from(stream, allow_struct)?;
    binary::BinaryExpr::parse_from(stream, lhs, Precedence::Min, allow_struct)
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use crate::ast::{BinOp, Pattern, Stmt, StmtBlock};
    use crate::token::ToTokenStream;

    fn parse<T: Parse>(src: &str) -> T {
        let ts = TokenStream::from_str(src).unwrap();
        ts.parse().parse::<T>().unwrap()
    }

    fn render<T: ToTokenStream>(v: &T) -> String {
        v.to_token_stream().to_string()
    }

    #[test]
    fn literals_and_paths() {
        assert!(matches!(parse::<Expr>("42"), Expr::Primary(PrimaryExpr::Lit(_))));
        assert!(matches!(parse::<Expr>("foo"), Expr::Primary(PrimaryExpr::Path(_))));
        assert!(matches!(parse::<Expr>("a::b::c"), Expr::Primary(PrimaryExpr::Path(_))));
        assert!(matches!(parse::<Expr>("true"), Expr::Primary(PrimaryExpr::Lit(_))));
    }

    #[test]
    fn binary_precedence() {
        // `a + b * c` parses as `a + (b * c)`
        let e = parse::<Expr>("a + b * c");
        match e {
            Expr::Binary(BinaryExpr::Binary(ExprBinary {
                op: BinOp::Add, right, ..
            })) => {
                assert!(matches!(
                    *right,
                    Expr::Binary(BinaryExpr::Binary(ExprBinary { op: BinOp::Mul, .. }))
                ));
            }
            _ => panic!("expected top-level Add"),
        }
    }

    #[test]
    fn binary_left_assoc() {
        // `a - b - c` parses as `(a - b) - c`
        let e = parse::<Expr>("a - b - c");
        match e {
            Expr::Binary(BinaryExpr::Binary(ExprBinary {
                op: BinOp::Sub, left, ..
            })) => {
                assert!(matches!(
                    *left,
                    Expr::Binary(BinaryExpr::Binary(ExprBinary { op: BinOp::Sub, .. }))
                ));
            }
            _ => panic!("expected left-assoc Sub"),
        }
    }

    #[test]
    fn postfix() {
        assert!(matches!(parse::<Expr>("f(x)"), Expr::Postfix(PostfixExpr::Call(_))));
        assert!(matches!(parse::<Expr>("a.b"), Expr::Postfix(PostfixExpr::Field(_))));
        assert!(matches!(parse::<Expr>("a.b()"), Expr::Postfix(PostfixExpr::MethodCall(_))));
        assert!(matches!(parse::<Expr>("a[0]"), Expr::Postfix(PostfixExpr::Index(_))));
        assert!(matches!(parse::<Expr>("x?"), Expr::Unary(UnaryExpr::Try(_))));
        assert!(matches!(parse::<Expr>("x.await"), Expr::Postfix(PostfixExpr::Await(_))));
        assert!(matches!(parse::<Expr>("a.0"), Expr::Postfix(PostfixExpr::Field(_))));
    }

    #[test]
    fn method_turbofish() {
        let e = parse::<Expr>("x.collect::<Vec<_>>()");
        match e {
            Expr::Postfix(PostfixExpr::MethodCall(m)) => assert!(m.turbofish.is_some()),
            _ => panic!("expected method call with turbofish"),
        }
    }

    #[test]
    fn path_turbofish() {
        // `Foo::<T>` in expression position.
        assert!(matches!(parse::<Expr>("Foo::<T>"), Expr::Primary(PrimaryExpr::Path(_))));
    }

    #[test]
    fn ranges() {
        assert!(matches!(parse::<Expr>("0..10"), Expr::Binary(BinaryExpr::Range(_))));
        assert!(matches!(parse::<Expr>("0..=10"), Expr::Binary(BinaryExpr::Range(_))));
        assert!(matches!(parse::<Expr>("a.."), Expr::Binary(BinaryExpr::Range(_))));
        assert!(matches!(parse::<Expr>("..b"), Expr::Binary(BinaryExpr::Range(_))));
        assert!(matches!(parse::<Expr>(".."), Expr::Binary(BinaryExpr::Range(_))));
    }

    #[test]
    fn if_while_let() {
        assert!(matches!(
            parse::<Expr>("if let Some(x) = o { x } else { 0 }"),
            Expr::Block(BlockExpr::If(_))
        ));
        assert!(matches!(
            parse::<Expr>("while let Some(x) = it.next() { }"),
            Expr::Block(BlockExpr::While(_))
        ));
    }

    #[test]
    fn block_exprs() {
        assert!(matches!(parse::<Expr>("async { 1 }"), Expr::Block(BlockExpr::Async(_))));
        assert!(matches!(parse::<Expr>("async move { x }"), Expr::Block(BlockExpr::Async(_))));
        assert!(matches!(parse::<Expr>("const { 1 }"), Expr::Block(BlockExpr::Const(_))));
        assert!(matches!(parse::<Expr>("try { 1 }"), Expr::Block(BlockExpr::TryBlock(_))));
    }

    #[test]
    fn closures_with_modifiers() {
        assert!(matches!(parse::<Expr>("async || 1"), Expr::Primary(PrimaryExpr::Closure(_))));
        assert!(matches!(
            parse::<Expr>("async move |x| x"),
            Expr::Primary(PrimaryExpr::Closure(_))
        ));
        assert!(matches!(parse::<Expr>("const || 1"), Expr::Primary(PrimaryExpr::Closure(_))));
    }

    #[test]
    fn labeled() {
        assert!(matches!(
            parse::<Expr>("'a: loop { break 'a 1 }"),
            Expr::Block(BlockExpr::Loop(_))
        ));
        assert!(matches!(parse::<Expr>("'a: { 1 }"), Expr::Block(BlockExpr::Brace(_))));
    }

    #[test]
    fn qualified_path_expr() {
        assert!(matches!(
            parse::<Expr>("<T as Trait>::CONST"),
            Expr::Primary(PrimaryExpr::Path(_))
        ));
        assert!(matches!(
            parse::<Expr>("::std::mem::swap"),
            Expr::Primary(PrimaryExpr::Path(_))
        ));
    }

    #[test]
    fn unary_and_ref() {
        assert!(matches!(parse::<Expr>("-x"), Expr::Unary(UnaryExpr::Unary(_))));
        assert!(matches!(parse::<Expr>("!x"), Expr::Unary(UnaryExpr::Unary(_))));
        assert!(matches!(parse::<Expr>("*x"), Expr::Unary(UnaryExpr::Unary(_))));
        assert!(matches!(parse::<Expr>("&x"), Expr::Unary(UnaryExpr::Reference(_))));
        assert!(matches!(parse::<Expr>("&mut x"), Expr::Unary(UnaryExpr::Reference(_))));
    }

    #[test]
    fn collections() {
        assert!(matches!(parse::<Expr>("(a, b)"), Expr::Primary(PrimaryExpr::Tuple(_))));
        assert!(matches!(parse::<Expr>("(a)"), Expr::Primary(PrimaryExpr::Paren(_))));
        assert!(matches!(parse::<Expr>("[a, b, c]"), Expr::Primary(PrimaryExpr::Array(_))));
        assert!(matches!(parse::<Expr>("[0; 4]"), Expr::Primary(PrimaryExpr::Repeat(_))));
    }

    #[test]
    fn cast_and_assign() {
        assert!(matches!(parse::<Expr>("x as u32"), Expr::Unary(UnaryExpr::Cast(_))));
        assert!(matches!(parse::<Expr>("x = y"), Expr::Binary(BinaryExpr::Assign(_))));
        assert!(matches!(parse::<Expr>("x += y"), Expr::Binary(BinaryExpr::AssignOp(_))));
    }

    #[test]
    fn control_flow() {
        assert!(matches!(
            parse::<Expr>("if a { b } else { c }"),
            Expr::Block(BlockExpr::If(_))
        ));
        assert!(matches!(parse::<Expr>("while a { }"), Expr::Block(BlockExpr::While(_))));
        assert!(matches!(parse::<Expr>("for x in xs { }"), Expr::Block(BlockExpr::ForLoop(_))));
        assert!(matches!(parse::<Expr>("loop { }"), Expr::Block(BlockExpr::Loop(_))));
        assert!(matches!(
            parse::<Expr>("match x { _ => 1 }"),
            Expr::Block(BlockExpr::Match(_))
        ));
        assert!(matches!(parse::<Expr>("{ a }"), Expr::Block(BlockExpr::Brace(_))));
        assert!(matches!(parse::<Expr>("unsafe { }"), Expr::Block(BlockExpr::Unsafe(_))));
        assert!(matches!(parse::<Expr>("return x"), Expr::Jump(JumpExpr::Return(_))));
    }

    #[test]
    fn struct_literal() {
        let e = parse::<Expr>("Foo { a: 1, b }");
        assert!(matches!(e, Expr::Primary(PrimaryExpr::Struct(_))));
        // struct literal is disallowed in `if` condition position
        assert!(matches!(parse::<Expr>("if x { }"), Expr::Block(BlockExpr::If(_))));
    }

    #[test]
    fn closures() {
        assert!(matches!(parse::<Expr>("|x| x"), Expr::Primary(PrimaryExpr::Closure(_))));
        assert!(matches!(
            parse::<Expr>("|x: u32| -> u32 { x }"),
            Expr::Primary(PrimaryExpr::Closure(_))
        ));
        assert!(matches!(parse::<Expr>("move || 1"), Expr::Primary(PrimaryExpr::Closure(_))));
        assert!(matches!(parse::<Expr>("|| {}"), Expr::Primary(PrimaryExpr::Closure(_))));
    }

    #[test]
    fn macro_call() {
        assert!(matches!(parse::<Expr>("vec![1, 2, 3]"), Expr::Primary(PrimaryExpr::Macro(_))));
        assert!(matches!(
            parse::<Expr>("println!(\"hi\")"),
            Expr::Primary(PrimaryExpr::Macro(_))
        ));
    }

    #[test]
    fn patterns() {
        assert!(matches!(parse::<Pattern>("x"), Pattern::Ident(_)));
        assert!(matches!(parse::<Pattern>("_"), Pattern::Wild));
        assert!(matches!(parse::<Pattern>("mut x"), Pattern::Ident(_)));
        assert!(matches!(parse::<Pattern>("&x"), Pattern::Reference(_)));
        assert!(matches!(parse::<Pattern>("(a, b)"), Pattern::Tuple(_)));
        assert!(matches!(parse::<Pattern>("Some(x)"), Pattern::TupleStruct(_)));
        assert!(matches!(parse::<Pattern>("Point { x, y }"), Pattern::Struct(_)));
        assert!(matches!(parse::<Pattern>("1"), Pattern::Lit(_)));
    }

    #[test]
    fn or_and_exotic_patterns() {
        assert!(matches!(parse::<Pattern>("A | B | C"), Pattern::Or(_)));
        assert!(matches!(parse::<Pattern>("| A | B"), Pattern::Or(_)));
        assert!(matches!(parse::<Pattern>("box x"), Pattern::Box(_)));
        assert!(matches!(parse::<Pattern>("const { 1 }"), Pattern::Const(_)));
        // single alternative stays a non-Or pattern
        assert!(matches!(parse::<Pattern>("x"), Pattern::Ident(_)));
    }

    #[test]
    fn statements_and_blocks() {
        let b = parse::<StmtBlock>("{ let x = 1; x + 1 }");
        assert_eq!(b.stmts.len(), 2);
        assert!(matches!(b.stmts[0], Stmt::Local(_)));
        assert!(matches!(b.stmts[1], Stmt::Expr(..)));

        let b2 = parse::<StmtBlock>("{ foo(); bar(); }");
        assert_eq!(b2.stmts.len(), 2);
        assert!(matches!(b2.stmts[0], Stmt::Expr(_, Some(_))));
    }

    #[test]
    fn roundtrips() {
        for src in ["a + b * c", "f (x , y)", "a . b . c", "x as u32", "- x", "& mut x"] {
            let e = parse::<Expr>(src);
            // re-render and re-parse must produce an equal token string
            let r = render(&e);
            let e2 = parse::<Expr>(&r);
            assert_eq!(render(&e2), r, "unstable roundtrip for {src}");
        }
    }
}
