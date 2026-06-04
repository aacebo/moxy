use moxy_token::parse::{ParseError, ParseStream};
use moxy_token::{Parse, ToTokens, TokenStream};

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
    use crate::precedence::Precedence;
    let lhs = unary::UnaryExpr::parse_from(stream, allow_struct)?;
    binary::BinaryExpr::parse_from(stream, lhs, Precedence::Min, allow_struct)
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use moxy_token::ToTokenStream;

    use super::*;
    use crate::{BinOp, Pattern, Stmt, StmtBlock};

    fn render<T: ToTokenStream>(v: &T) -> String {
        v.to_token_stream().to_string()
    }

    #[test]
    fn literals_and_paths() {
        assert!(matches!(
            moxy_token::parse!("42" as Expr).unwrap(),
            Expr::Primary(PrimaryExpr::Lit(_))
        ));
        assert!(matches!(
            moxy_token::parse!("foo" as Expr).unwrap(),
            Expr::Primary(PrimaryExpr::Path(_))
        ));
        assert!(matches!(
            moxy_token::parse!("a::b::c" as Expr).unwrap(),
            Expr::Primary(PrimaryExpr::Path(_))
        ));
        assert!(matches!(
            moxy_token::parse!("true" as Expr).unwrap(),
            Expr::Primary(PrimaryExpr::Lit(_))
        ));
    }

    #[test]
    fn binary_precedence() {
        let e = moxy_token::parse!("a + b * c" as Expr).unwrap();
        match e {
            Expr::Binary(BinaryExpr::Binary(ExprBinary {
                op: BinOp::Add(_),
                right,
                ..
            })) => {
                assert!(matches!(
                    *right,
                    Expr::Binary(BinaryExpr::Binary(ExprBinary { op: BinOp::Mul(_), .. }))
                ));
            }
            _ => panic!("expected top-level Add"),
        }
    }

    #[test]
    fn binary_left_assoc() {
        let e = moxy_token::parse!("a - b - c" as Expr).unwrap();
        match e {
            Expr::Binary(BinaryExpr::Binary(ExprBinary {
                op: BinOp::Sub(_), left, ..
            })) => {
                assert!(matches!(
                    *left,
                    Expr::Binary(BinaryExpr::Binary(ExprBinary { op: BinOp::Sub(_), .. }))
                ));
            }
            _ => panic!("expected left-assoc Sub"),
        }
    }

    #[test]
    fn postfix() {
        assert!(matches!(
            moxy_token::parse!("f(x)" as Expr).unwrap(),
            Expr::Postfix(PostfixExpr::Call(_))
        ));
        assert!(matches!(
            moxy_token::parse!("a.b" as Expr).unwrap(),
            Expr::Postfix(PostfixExpr::Field(_))
        ));
        assert!(matches!(
            moxy_token::parse!("a.b()" as Expr).unwrap(),
            Expr::Postfix(PostfixExpr::MethodCall(_))
        ));
        assert!(matches!(
            moxy_token::parse!("a[0]" as Expr).unwrap(),
            Expr::Postfix(PostfixExpr::Index(_))
        ));
        assert!(matches!(
            moxy_token::parse!("x?" as Expr).unwrap(),
            Expr::Unary(UnaryExpr::Try(_))
        ));
        assert!(matches!(
            moxy_token::parse!("x.await" as Expr).unwrap(),
            Expr::Postfix(PostfixExpr::Await(_))
        ));
        assert!(matches!(
            moxy_token::parse!("a.0" as Expr).unwrap(),
            Expr::Postfix(PostfixExpr::Field(_))
        ));
    }

    #[test]
    fn method_turbofish() {
        let e = moxy_token::parse!("x.collect::<Vec<_>>()" as Expr).unwrap();
        match e {
            Expr::Postfix(PostfixExpr::MethodCall(m)) => assert!(m.turbofish.is_some()),
            _ => panic!("expected method call with turbofish"),
        }
    }

    #[test]
    fn path_turbofish() {
        assert!(matches!(
            moxy_token::parse!("Foo::<T>" as Expr).unwrap(),
            Expr::Primary(PrimaryExpr::Path(_))
        ));
    }

    #[test]
    fn ranges() {
        assert!(matches!(
            moxy_token::parse!("0..10" as Expr).unwrap(),
            Expr::Binary(BinaryExpr::Range(_))
        ));
        assert!(matches!(
            moxy_token::parse!("0..=10" as Expr).unwrap(),
            Expr::Binary(BinaryExpr::Range(_))
        ));
        assert!(matches!(
            moxy_token::parse!("a.." as Expr).unwrap(),
            Expr::Binary(BinaryExpr::Range(_))
        ));
        assert!(matches!(
            moxy_token::parse!("..b" as Expr).unwrap(),
            Expr::Binary(BinaryExpr::Range(_))
        ));
        assert!(matches!(
            moxy_token::parse!(".." as Expr).unwrap(),
            Expr::Binary(BinaryExpr::Range(_))
        ));
    }

    #[test]
    fn if_while_let() {
        assert!(matches!(
            moxy_token::parse!("if let Some(x) = o { x } else { 0 }" as Expr).unwrap(),
            Expr::Block(BlockExpr::If(_))
        ));
        assert!(matches!(
            moxy_token::parse!("while let Some(x) = it.next() { }" as Expr).unwrap(),
            Expr::Block(BlockExpr::While(_))
        ));
    }

    #[test]
    fn block_exprs() {
        assert!(matches!(
            moxy_token::parse!("async { 1 }" as Expr).unwrap(),
            Expr::Block(BlockExpr::Async(_))
        ));
        assert!(matches!(
            moxy_token::parse!("async move { x }" as Expr).unwrap(),
            Expr::Block(BlockExpr::Async(_))
        ));
        assert!(matches!(
            moxy_token::parse!("const { 1 }" as Expr).unwrap(),
            Expr::Block(BlockExpr::Const(_))
        ));
        assert!(matches!(
            moxy_token::parse!("try { 1 }" as Expr).unwrap(),
            Expr::Block(BlockExpr::TryBlock(_))
        ));
    }

    #[test]
    fn closures_with_modifiers() {
        assert!(matches!(
            moxy_token::parse!("async || 1" as Expr).unwrap(),
            Expr::Primary(PrimaryExpr::Closure(_))
        ));
        assert!(matches!(
            moxy_token::parse!("async move |x| x" as Expr).unwrap(),
            Expr::Primary(PrimaryExpr::Closure(_))
        ));
        assert!(matches!(
            moxy_token::parse!("const || 1" as Expr).unwrap(),
            Expr::Primary(PrimaryExpr::Closure(_))
        ));
    }

    #[test]
    fn labeled() {
        assert!(matches!(
            moxy_token::parse!("'a: loop { break 'a 1 }" as Expr).unwrap(),
            Expr::Block(BlockExpr::Loop(_))
        ));
        assert!(matches!(
            moxy_token::parse!("'a: { 1 }" as Expr).unwrap(),
            Expr::Block(BlockExpr::Brace(_))
        ));
    }

    #[test]
    fn qualified_path_expr() {
        assert!(matches!(
            moxy_token::parse!("<T as Trait>::CONST" as Expr).unwrap(),
            Expr::Primary(PrimaryExpr::Path(_))
        ));
        assert!(matches!(
            moxy_token::parse!("::std::mem::swap" as Expr).unwrap(),
            Expr::Primary(PrimaryExpr::Path(_))
        ));
    }

    #[test]
    fn unary_and_ref() {
        assert!(matches!(
            moxy_token::parse!("-x" as Expr).unwrap(),
            Expr::Unary(UnaryExpr::Unary(_))
        ));
        assert!(matches!(
            moxy_token::parse!("!x" as Expr).unwrap(),
            Expr::Unary(UnaryExpr::Unary(_))
        ));
        assert!(matches!(
            moxy_token::parse!("*x" as Expr).unwrap(),
            Expr::Unary(UnaryExpr::Unary(_))
        ));
        assert!(matches!(
            moxy_token::parse!("&x" as Expr).unwrap(),
            Expr::Unary(UnaryExpr::Reference(_))
        ));
        assert!(matches!(
            moxy_token::parse!("&mut x" as Expr).unwrap(),
            Expr::Unary(UnaryExpr::Reference(_))
        ));
    }

    #[test]
    fn collections() {
        assert!(matches!(
            moxy_token::parse!("(a, b)" as Expr).unwrap(),
            Expr::Primary(PrimaryExpr::Tuple(_))
        ));
        assert!(matches!(
            moxy_token::parse!("(a)" as Expr).unwrap(),
            Expr::Primary(PrimaryExpr::Paren(_))
        ));
        assert!(matches!(
            moxy_token::parse!("[a, b, c]" as Expr).unwrap(),
            Expr::Primary(PrimaryExpr::Array(_))
        ));
        assert!(matches!(
            moxy_token::parse!("[0; 4]" as Expr).unwrap(),
            Expr::Primary(PrimaryExpr::Repeat(_))
        ));
    }

    #[test]
    fn cast_and_assign() {
        assert!(matches!(
            moxy_token::parse!("x as u32" as Expr).unwrap(),
            Expr::Unary(UnaryExpr::Cast(_))
        ));
        assert!(matches!(
            moxy_token::parse!("x = y" as Expr).unwrap(),
            Expr::Binary(BinaryExpr::Assign(_))
        ));
        assert!(matches!(
            moxy_token::parse!("x += y" as Expr).unwrap(),
            Expr::Binary(BinaryExpr::AssignOp(_))
        ));
    }

    #[test]
    fn control_flow() {
        assert!(matches!(
            moxy_token::parse!("if a { b } else { c }" as Expr).unwrap(),
            Expr::Block(BlockExpr::If(_))
        ));
        assert!(matches!(
            moxy_token::parse!("while a { }" as Expr).unwrap(),
            Expr::Block(BlockExpr::While(_))
        ));
        assert!(matches!(
            moxy_token::parse!("for x in xs { }" as Expr).unwrap(),
            Expr::Block(BlockExpr::ForLoop(_))
        ));
        assert!(matches!(
            moxy_token::parse!("loop { }" as Expr).unwrap(),
            Expr::Block(BlockExpr::Loop(_))
        ));
        assert!(matches!(
            moxy_token::parse!("match x { _ => 1 }" as Expr).unwrap(),
            Expr::Block(BlockExpr::Match(_))
        ));
        assert!(matches!(
            moxy_token::parse!("{ a }" as Expr).unwrap(),
            Expr::Block(BlockExpr::Brace(_))
        ));
        assert!(matches!(
            moxy_token::parse!("unsafe { }" as Expr).unwrap(),
            Expr::Block(BlockExpr::Unsafe(_))
        ));
        assert!(matches!(
            moxy_token::parse!("return x" as Expr).unwrap(),
            Expr::Jump(JumpExpr::Return(_))
        ));
    }

    #[test]
    fn struct_literal() {
        let e = moxy_token::parse!("Foo { a: 1, b }" as Expr).unwrap();
        assert!(matches!(e, Expr::Primary(PrimaryExpr::Struct(_))));
        assert!(matches!(
            moxy_token::parse!("if x { }" as Expr).unwrap(),
            Expr::Block(BlockExpr::If(_))
        ));
    }

    #[test]
    fn closures() {
        assert!(matches!(
            moxy_token::parse!("|x| x" as Expr).unwrap(),
            Expr::Primary(PrimaryExpr::Closure(_))
        ));
        assert!(matches!(
            moxy_token::parse!("|x: u32| -> u32 { x }" as Expr).unwrap(),
            Expr::Primary(PrimaryExpr::Closure(_))
        ));
        assert!(matches!(
            moxy_token::parse!("move || 1" as Expr).unwrap(),
            Expr::Primary(PrimaryExpr::Closure(_))
        ));
        assert!(matches!(
            moxy_token::parse!("|| {}" as Expr).unwrap(),
            Expr::Primary(PrimaryExpr::Closure(_))
        ));
    }

    #[test]
    fn macro_call() {
        assert!(matches!(
            moxy_token::parse!("vec![1, 2, 3]" as Expr).unwrap(),
            Expr::Primary(PrimaryExpr::Macro(_))
        ));
        assert!(matches!(
            moxy_token::parse!("println!(\"hi\")" as Expr).unwrap(),
            Expr::Primary(PrimaryExpr::Macro(_))
        ));
    }

    #[test]
    fn patterns() {
        assert!(matches!(moxy_token::parse!("x" as Pattern).unwrap(), Pattern::Ident(_)));
        assert!(matches!(moxy_token::parse!("_" as Pattern).unwrap(), Pattern::Wild));
        assert!(matches!(moxy_token::parse!("mut x" as Pattern).unwrap(), Pattern::Ident(_)));
        assert!(matches!(moxy_token::parse!("&x" as Pattern).unwrap(), Pattern::Reference(_)));
        assert!(matches!(moxy_token::parse!("(a, b)" as Pattern).unwrap(), Pattern::Tuple(_)));
        assert!(matches!(
            moxy_token::parse!("Some(x)" as Pattern).unwrap(),
            Pattern::TupleStruct(_)
        ));
        assert!(matches!(
            moxy_token::parse!("Point { x, y }" as Pattern).unwrap(),
            Pattern::Struct(_)
        ));
        assert!(matches!(moxy_token::parse!("1" as Pattern).unwrap(), Pattern::Lit(_)));
    }

    #[test]
    fn or_and_exotic_patterns() {
        assert!(matches!(moxy_token::parse!("A | B | C" as Pattern).unwrap(), Pattern::Or(_)));
        assert!(matches!(moxy_token::parse!("| A | B" as Pattern).unwrap(), Pattern::Or(_)));
        assert!(matches!(moxy_token::parse!("box x" as Pattern).unwrap(), Pattern::Box(_)));
        assert!(matches!(
            moxy_token::parse!("const { 1 }" as Pattern).unwrap(),
            Pattern::Const(_)
        ));
        assert!(matches!(moxy_token::parse!("x" as Pattern).unwrap(), Pattern::Ident(_)));
    }

    #[test]
    fn statements_and_blocks() {
        let b = moxy_token::parse!("{ let x = 1; x + 1 }" as StmtBlock).unwrap();
        assert_eq!(b.stmts.len(), 2);
        assert!(matches!(b.stmts[0], Stmt::Local(_)));
        assert!(matches!(b.stmts[1], Stmt::Expr(..)));

        let b2 = moxy_token::parse!("{ foo(); bar(); }" as StmtBlock).unwrap();
        assert_eq!(b2.stmts.len(), 2);
        assert!(matches!(b2.stmts[0], Stmt::Expr(_, Some(_))));
    }

    #[test]
    fn roundtrips() {
        for src in ["a + b * c", "f (x , y)", "a . b . c", "x as u32", "- x", "& mut x"] {
            let e: Expr = moxy_token::parse!(src).unwrap();
            let r = render(&e);
            let e2: Expr = moxy_token::parse!(r).unwrap();
            assert_eq!(render(&e2), r, "unstable roundtrip for {src}");
        }
    }
}
