use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::{Parse, Span, Spanner, ToTokens, TokenStream};

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

/// A Rust expression. The primary recursive node covering all expression forms.
#[derive(Debug, Clone, PartialEq, Eq)]
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

impl Expr {
    pub fn is_unary(&self) -> bool {
        matches!(self, Self::Unary(_))
    }

    pub fn is_binary(&self) -> bool {
        matches!(self, Self::Binary(_))
    }

    pub fn is_postfix(&self) -> bool {
        matches!(self, Self::Postfix(_))
    }

    pub fn is_block(&self) -> bool {
        matches!(self, Self::Block(_))
    }

    pub fn is_jump(&self) -> bool {
        matches!(self, Self::Jump(_))
    }

    pub fn is_primary(&self) -> bool {
        matches!(self, Self::Primary(_))
    }

    pub fn is_infer(&self) -> bool {
        matches!(self, Self::Infer)
    }

    pub fn is_verbatim(&self) -> bool {
        matches!(self, Self::Verbatim(_))
    }

    pub fn as_unary(&self) -> Option<&UnaryExpr> {
        if let Self::Unary(v) = self { Some(v) } else { None }
    }

    pub fn as_binary(&self) -> Option<&BinaryExpr> {
        if let Self::Binary(v) = self { Some(v) } else { None }
    }

    pub fn as_postfix(&self) -> Option<&PostfixExpr> {
        if let Self::Postfix(v) = self { Some(v) } else { None }
    }

    pub fn as_block(&self) -> Option<&BlockExpr> {
        if let Self::Block(v) = self { Some(v) } else { None }
    }

    pub fn as_jump(&self) -> Option<&JumpExpr> {
        if let Self::Jump(v) = self { Some(v) } else { None }
    }

    pub fn as_primary(&self) -> Option<&PrimaryExpr> {
        if let Self::Primary(v) = self { Some(v) } else { None }
    }

    pub fn attrs(&self) -> Option<&crate::Attributes> {
        match self {
            Self::Unary(v) => Some(v.attrs()),
            Self::Binary(v) => Some(v.attrs()),
            Self::Postfix(v) => Some(v.attrs()),
            Self::Block(v) => Some(v.attrs()),
            Self::Jump(v) => Some(v.attrs()),
            Self::Primary(v) => Some(v.attrs()),
            Self::Infer | Self::Verbatim(_) => None,
        }
    }

    pub fn attrs_mut(&mut self) -> Option<&mut crate::Attributes> {
        match self {
            Self::Unary(v) => Some(v.attrs_mut()),
            Self::Binary(v) => Some(v.attrs_mut()),
            Self::Postfix(v) => Some(v.attrs_mut()),
            Self::Block(v) => Some(v.attrs_mut()),
            Self::Jump(v) => Some(v.attrs_mut()),
            Self::Primary(v) => Some(v.attrs_mut()),
            Self::Infer | Self::Verbatim(_) => None,
        }
    }
}

impl Spanner for Expr {
    fn span(&self) -> Span {
        match self {
            Expr::Unary(v) => v.span(),
            Expr::Binary(v) => v.span(),
            Expr::Postfix(v) => v.span(),
            Expr::Block(v) => v.span(),
            Expr::Jump(v) => v.span(),
            Expr::Primary(v) => v.span(),
            Expr::Infer => Span::call_site(),
            Expr::Verbatim(_) => Span::call_site(),
        }
    }
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
        assert_eq!(b.stmts.inner.len(), 2);
        assert!(matches!(b.stmts.inner[0], Stmt::Local(_)));
        assert!(matches!(b.stmts.inner[1], Stmt::Expr(..)));

        let b2 = moxy_token::parse!("{ foo(); bar(); }" as StmtBlock).unwrap();
        assert_eq!(b2.stmts.inner.len(), 2);
        assert!(matches!(b2.stmts.inner[0], Stmt::Expr(_, Some(_))));
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

    #[test]
    fn leading_attributes() {
        let e = moxy_token::parse!("#[a] if c { }" as Expr).unwrap();
        match &e {
            Expr::Block(BlockExpr::If(v)) => assert_eq!(v.attrs.len(), 1),
            other => panic!("expected if-expr, got {other:?}"),
        }
        assert_eq!(render(&e), "# [a] if c {}");

        let lit = moxy_token::parse!("#[a] 42" as Expr).unwrap();
        match &lit {
            Expr::Primary(PrimaryExpr::Lit(v)) => assert_eq!(v.attrs.len(), 1),
            other => panic!("expected lit, got {other:?}"),
        }

        // Multiple attributes on a closure.
        let cl = moxy_token::parse!("#[a] #[b] || x" as Expr).unwrap();
        match &cl {
            Expr::Primary(PrimaryExpr::Closure(v)) => assert_eq!(v.attrs.len(), 2),
            other => panic!("expected closure, got {other:?}"),
        }

        // No attributes -> empty.
        let bare = moxy_token::parse!("42" as Expr).unwrap();
        assert!(bare.clone().attrs_mut().is_none_or(|a| a.is_empty()));

        // Postfix: the leading attribute binds to the inner atom (receiver), since each
        // node parses its own attrs and the postfix wrapper sees none. Round-trip is stable.
        let pf = moxy_token::parse!("#[a] foo . bar ()" as Expr).unwrap();
        match &pf {
            Expr::Postfix(PostfixExpr::MethodCall(m)) => {
                assert!(m.attrs.is_empty());
                match m.receiver.as_ref() {
                    Expr::Primary(PrimaryExpr::Path(p)) => assert_eq!(p.attrs.len(), 1),
                    other => panic!("expected path receiver, got {other:?}"),
                }
            }
            other => panic!("expected method call, got {other:?}"),
        }
        assert_eq!(render(&pf), "# [a] foo . bar ()");
    }

    #[test]
    fn attribute_roundtrip() {
        for src in ["# [a] 42", "# [cfg (test)] if c {}", "# [a] # [b] || x"] {
            let e: Expr = moxy_token::parse!(src).unwrap();
            let out = render(&e);
            debug_assert_eq!(out, src, "unstable attr roundtrip: \"{src}\" => \"{out}\"");
        }
    }
}
