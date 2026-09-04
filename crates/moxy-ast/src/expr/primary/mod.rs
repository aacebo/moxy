mod expr_array;
mod expr_closure;
mod expr_group;
mod expr_infer;
mod expr_let;
mod expr_lit;
mod expr_macro;
mod expr_paren;
mod expr_path;
mod expr_repeat;
mod expr_struct;
mod expr_tuple;

use crate::{ParseError, Parser, Token};

pub use expr_array::*;
pub use expr_closure::*;
pub use expr_group::*;
pub use expr_let::*;
pub use expr_lit::*;
pub use expr_macro::*;
pub use expr_paren::*;
pub use expr_path::*;
pub use expr_repeat::*;
pub use expr_struct::*;
pub use expr_tuple::*;
use moxy_token::{Delim, LexError, Punct, Span, Spanner, ToTokens, TokenStream, TokenTree};

use super::block::{
    ExprAsync, ExprBrace, ExprConst, ExprForLoop, ExprIf, ExprLoop, ExprMatch, ExprTryBlock, ExprUnsafe, ExprWhile,
};
use super::jump::{ExprBreak, ExprContinue, ExprReturn, ExprYield};
use super::{BlockExpr, Expr, JumpExpr};
use crate::{
    Asyncness, Attributes, ClosureParam, Constness, Delimited, FieldValue, Label, Movability, Pattern, Punctuated, ReturnType,
};

/// Primary/leaf expressions (literals, paths, closures, collections, struct literals, macros).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum PrimaryExpr {
    Lit(ExprLit),
    Path(ExprPath),
    Struct(ExprStruct),
    Closure(ExprClosure),
    Tuple(ExprTuple),
    Array(ExprArray),
    Repeat(ExprRepeat),
    Let(ExprLet),
    Paren(ExprParen),
    Group(ExprGroup),
    Macro(ExprMacro),
}

impl PrimaryExpr {
    pub fn attrs(&self) -> &Attributes {
        match self {
            Self::Lit(v) => &v.attrs,
            Self::Path(v) => &v.attrs,
            Self::Struct(v) => &v.attrs,
            Self::Closure(v) => &v.attrs,
            Self::Tuple(v) => &v.attrs,
            Self::Array(v) => &v.attrs,
            Self::Repeat(v) => &v.attrs,
            Self::Let(v) => &v.attrs,
            Self::Paren(v) => &v.attrs,
            Self::Group(v) => &v.attrs,
            Self::Macro(v) => &v.attrs,
        }
    }

    pub fn attrs_mut(&mut self) -> &mut Attributes {
        match self {
            Self::Lit(v) => &mut v.attrs,
            Self::Path(v) => &mut v.attrs,
            Self::Struct(v) => &mut v.attrs,
            Self::Closure(v) => &mut v.attrs,
            Self::Tuple(v) => &mut v.attrs,
            Self::Array(v) => &mut v.attrs,
            Self::Repeat(v) => &mut v.attrs,
            Self::Let(v) => &mut v.attrs,
            Self::Paren(v) => &mut v.attrs,
            Self::Group(v) => &mut v.attrs,
            Self::Macro(v) => &mut v.attrs,
        }
    }

    pub fn is_lit(&self) -> bool {
        matches!(self, Self::Lit(_))
    }

    pub fn is_path(&self) -> bool {
        matches!(self, Self::Path(_))
    }

    pub fn is_struct(&self) -> bool {
        matches!(self, Self::Struct(_))
    }

    pub fn is_closure(&self) -> bool {
        matches!(self, Self::Closure(_))
    }

    pub fn is_tuple(&self) -> bool {
        matches!(self, Self::Tuple(_))
    }

    pub fn is_array(&self) -> bool {
        matches!(self, Self::Array(_))
    }

    pub fn is_repeat(&self) -> bool {
        matches!(self, Self::Repeat(_))
    }

    pub fn is_let(&self) -> bool {
        matches!(self, Self::Let(_))
    }

    pub fn is_paren(&self) -> bool {
        matches!(self, Self::Paren(_))
    }

    pub fn is_group(&self) -> bool {
        matches!(self, Self::Group(_))
    }

    pub fn is_macro(&self) -> bool {
        matches!(self, Self::Macro(_))
    }

    pub fn as_lit(&self) -> Option<&ExprLit> {
        if let Self::Lit(v) = self { Some(v) } else { None }
    }

    pub fn as_path(&self) -> Option<&ExprPath> {
        if let Self::Path(v) = self { Some(v) } else { None }
    }

    pub fn as_struct(&self) -> Option<&ExprStruct> {
        if let Self::Struct(v) = self { Some(v) } else { None }
    }

    pub fn as_closure(&self) -> Option<&ExprClosure> {
        if let Self::Closure(v) = self { Some(v) } else { None }
    }

    pub fn as_tuple(&self) -> Option<&ExprTuple> {
        if let Self::Tuple(v) = self { Some(v) } else { None }
    }

    pub fn as_array(&self) -> Option<&ExprArray> {
        if let Self::Array(v) = self { Some(v) } else { None }
    }

    pub fn as_repeat(&self) -> Option<&ExprRepeat> {
        if let Self::Repeat(v) = self { Some(v) } else { None }
    }

    pub fn as_let(&self) -> Option<&ExprLet> {
        if let Self::Let(v) = self { Some(v) } else { None }
    }

    pub fn as_paren(&self) -> Option<&ExprParen> {
        if let Self::Paren(v) = self { Some(v) } else { None }
    }

    pub fn as_group(&self) -> Option<&ExprGroup> {
        if let Self::Group(v) = self { Some(v) } else { None }
    }

    pub fn as_macro(&self) -> Option<&ExprMacro> {
        if let Self::Macro(v) = self { Some(v) } else { None }
    }

    pub fn into_expr(self) -> super::Expr {
        super::Expr::from(self)
    }
}

impl Spanner for PrimaryExpr {
    fn span(&self) -> Span {
        match self {
            Self::Lit(v) => v.span(),
            Self::Path(v) => v.span(),
            Self::Struct(v) => v.span(),
            Self::Closure(v) => v.span(),
            Self::Tuple(v) => v.span(),
            Self::Array(v) => v.span(),
            Self::Repeat(v) => v.span(),
            Self::Let(v) => v.span(),
            Self::Paren(v) => v.span(),
            Self::Group(v) => v.span(),
            Self::Macro(v) => v.span(),
        }
    }
}

impl ToTokens for PrimaryExpr {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            Self::Lit(v) => v.to_tokens(t),
            Self::Path(v) => v.to_tokens(t),
            Self::Struct(v) => v.to_tokens(t),
            Self::Closure(v) => v.to_tokens(t),
            Self::Tuple(v) => v.to_tokens(t),
            Self::Array(v) => v.to_tokens(t),
            Self::Repeat(v) => v.to_tokens(t),
            Self::Let(v) => v.to_tokens(t),
            Self::Paren(v) => v.to_tokens(t),
            Self::Group(v) => v.to_tokens(t),
            Self::Macro(v) => v.to_tokens(t),
        }
    }
}

impl From<ExprLit> for PrimaryExpr {
    fn from(v: ExprLit) -> Self {
        Self::Lit(v)
    }
}

impl From<ExprPath> for PrimaryExpr {
    fn from(v: ExprPath) -> Self {
        Self::Path(v)
    }
}

impl From<ExprStruct> for PrimaryExpr {
    fn from(v: ExprStruct) -> Self {
        Self::Struct(v)
    }
}

impl From<ExprClosure> for PrimaryExpr {
    fn from(v: ExprClosure) -> Self {
        Self::Closure(v)
    }
}

impl From<ExprTuple> for PrimaryExpr {
    fn from(v: ExprTuple) -> Self {
        Self::Tuple(v)
    }
}

impl From<ExprArray> for PrimaryExpr {
    fn from(v: ExprArray) -> Self {
        Self::Array(v)
    }
}

impl From<ExprRepeat> for PrimaryExpr {
    fn from(v: ExprRepeat) -> Self {
        Self::Repeat(v)
    }
}

impl From<ExprLet> for PrimaryExpr {
    fn from(v: ExprLet) -> Self {
        Self::Let(v)
    }
}

impl From<ExprParen> for PrimaryExpr {
    fn from(v: ExprParen) -> Self {
        Self::Paren(v)
    }
}

impl From<ExprGroup> for PrimaryExpr {
    fn from(v: ExprGroup) -> Self {
        Self::Group(v)
    }
}

impl From<ExprMacro> for PrimaryExpr {
    fn from(v: ExprMacro) -> Self {
        Self::Macro(v)
    }
}

// Parser

impl ExprClosure {
    pub fn parse_from(parser: &Parser, attrs: Attributes) -> Result<Self, ParseError> {
        let constness = parser.parse::<Constness>()?;
        let asyncness = parser.parse::<Asyncness>()?;
        let capture = parser.parse_if::<Token![move]>();
        let (pipes, inputs) = if parser.peek::<Token![||]>() {
            let oror = parser.parse::<Token![||]>()?;
            (ClosurePipes::Empty(oror), Punctuated::new())
        } else {
            let open = parser.parse::<Token![|]>()?;
            let mut params = Punctuated::new();

            while !parser.peek::<Token![|]>() && !parser.is_empty() {
                params.push_value(parser.parse::<ClosureParam>()?);
                if parser.peek::<Token![,]>() {
                    params.push_punct(parser.parse::<Token![,]>()?);
                } else {
                    break;
                }
            }

            let close = parser.parse::<Token![|]>()?;
            (ClosurePipes::Params(open, close), params)
        };

        let output = parser.parse::<ReturnType>()?;
        let body = Box::new(super::parse_expr(parser, true)?);

        Ok(Self {
            attrs,
            lifetimes: None,
            constness,
            movability: Movability::Movable,
            asyncness,
            capture,
            pipes,
            inputs,
            output,
            body,
        })
    }
}

impl ExprStruct {
    pub fn parse_body(
        parser: &Parser,
    ) -> Result<(Punctuated<FieldValue, Token![,]>, Option<(Token![..], Box<Expr>)>), ParseError> {
        let mut fields = Punctuated::new();
        let mut rest = None;

        while !parser.is_empty() {
            if parser.peek::<Token![..]>() {
                let dotdot = parser.parse::<Token![..]>()?;
                rest = Some((dotdot, Box::new(super::parse_expr(parser, true)?)));
                break;
            }

            fields.push_value(parser.parse::<FieldValue>()?);

            if parser.peek::<Token![,]>() {
                fields.push_punct(parser.parse::<Token![,]>()?);
            } else {
                break;
            }
        }

        Ok((fields, rest))
    }
}

impl ExprRepeat {
    pub fn try_parse(
        parser: &Parser,
        bracket_span: moxy_token::span::DelimSpan,
        attrs: Attributes,
    ) -> Result<Option<Self>, ParseError> {
        let lookahead = parser.lookahead();

        if super::parse_expr(&lookahead, true).is_err() {
            return Ok(None);
        }

        if !lookahead.peek::<Token![;]>() {
            return Ok(None);
        }

        let elem = super::parse_expr(parser, true)?;
        let semi = parser.parse::<Token![;]>()?;
        let len = super::parse_expr(parser, true)?;

        Ok(Some(Self {
            attrs,
            content: Delimited::bracket(
                bracket_span,
                RepeatInner {
                    elem: Box::new(elem),
                    semi,
                    len: Box::new(len),
                },
            ),
        }))
    }
}

impl Expr {
    pub fn parse_if(parser: &Parser) -> Result<Option<Box<Self>>, ParseError> {
        if parser.is_empty() || parser.peek::<Token![;]>() || parser.peek::<Token![,]>() {
            return Ok(None);
        }

        if parser.peek::<Expr>() {
            Ok(Some(Box::new(parser.parse()?)))
        } else {
            Ok(None)
        }
    }
}

impl PrimaryExpr {
    pub fn parse_from(parser: &Parser, allow_struct: bool, attrs: Attributes) -> Result<Expr, ParseError> {
        let at = parser.span();

        if matches!(parser.curr(), Some(tt) if tt.delim() == Some(Delim::Paren)) {
            let (paren_span, group_tokens) = parser.parse_group_spanned(Delim::Paren)?;
            let inner = Parser::from_tokens(&group_tokens);
            let elems: Punctuated<Expr, Token![,]> = Punctuated::parse_terminated(&inner)?;

            return Ok(if elems.len() == 1 && !elems.is_trailing() {
                let expr = Box::new(elems.into_iter().next().unwrap());
                Expr::Primary(Self::Paren(ExprParen {
                    attrs,
                    content: Delimited::paren(paren_span, expr),
                }))
            } else {
                Expr::Primary(Self::Tuple(ExprTuple {
                    attrs,
                    elems: Delimited::paren(paren_span, elems),
                }))
            });
        }

        if matches!(parser.curr(), Some(tt) if tt.delim() == Some(Delim::Bracket)) {
            let (bracket_span, group_tokens) = parser.parse_group_spanned(Delim::Bracket)?;
            let inner = Parser::from_tokens(&group_tokens);

            if let Some(rep) = ExprRepeat::try_parse(&inner, bracket_span, attrs.clone())? {
                return Ok(Expr::Primary(Self::Repeat(rep)));
            }

            let elems = Punctuated::parse_terminated(&inner)?;

            return Ok(Expr::Primary(Self::Array(ExprArray {
                attrs,
                elems: Delimited::bracket(bracket_span, elems),
            })));
        }

        // Labeled block / loop: `'a: { }`, `'a: loop { }`, etc.
        if Label::is_prefix(parser) {
            let label = Some(parser.parse::<Label>()?);

            if parser.peek::<Token![while]>() {
                return Ok(Expr::Block(BlockExpr::While(ExprWhile::parse_from(parser, label, attrs)?)));
            }

            if parser.peek::<Token![for]>() {
                return Ok(Expr::Block(BlockExpr::ForLoop(ExprForLoop::parse_from(
                    parser, label, attrs,
                )?)));
            }

            if parser.peek::<Token![loop]>() {
                return Ok(Expr::Block(BlockExpr::Loop(ExprLoop::parse_from(parser, label, attrs)?)));
            }

            return Ok(Expr::Block(BlockExpr::Brace(ExprBrace {
                attrs,
                label,
                block: parser.parse()?,
            })));
        }

        if matches!(parser.curr(), Some(tt) if tt.delim() == Some(Delim::Brace)) {
            return Ok(Expr::Block(BlockExpr::Brace(ExprBrace {
                attrs,
                label: None,
                block: parser.parse()?,
            })));
        }

        if parser.peek::<Token![if]>() {
            return ExprIf::parse_from(parser, attrs);
        }

        if parser.peek::<Token![while]>() {
            return Ok(Expr::Block(BlockExpr::While(ExprWhile::parse_from(parser, None, attrs)?)));
        }

        if parser.peek::<Token![for]>() {
            return Ok(Expr::Block(BlockExpr::ForLoop(ExprForLoop::parse_from(parser, None, attrs)?)));
        }

        if parser.peek::<Token![loop]>() {
            return Ok(Expr::Block(BlockExpr::Loop(ExprLoop::parse_from(parser, None, attrs)?)));
        }

        if parser.peek::<Token![match]>() {
            return ExprMatch::parse_from(parser, attrs);
        }

        if parser.peek::<Token![unsafe]>() {
            return Ok(Expr::Block(BlockExpr::Unsafe(ExprUnsafe::parse_from(parser, attrs)?)));
        }

        if parser.peek::<Token![const]>() && ExprBrace::is_next(parser) {
            return Ok(Expr::Block(BlockExpr::Const(ExprConst::parse_from(parser, attrs)?)));
        }

        if parser.peek::<Token![async]>() && ExprAsync::is_block(parser) {
            return Ok(Expr::Block(BlockExpr::Async(ExprAsync::parse_from(parser, attrs)?)));
        }

        if parser.peek::<Token![try]>() && ExprBrace::is_next(parser) {
            return Ok(Expr::Block(BlockExpr::TryBlock(ExprTryBlock::parse_from(parser, attrs)?)));
        }

        if parser.peek::<Token![return]>() {
            let return_keyword = parser.parse::<Token![return]>()?;

            return Ok(Expr::Jump(JumpExpr::Return(ExprReturn {
                attrs,
                return_keyword,
                expr: Expr::parse_if(parser)?,
            })));
        }

        if parser.peek::<Token![yield]>() {
            let yield_keyword = parser.parse::<Token![yield]>()?;

            return Ok(Expr::Jump(JumpExpr::Yield(ExprYield {
                attrs,
                yield_keyword,
                expr: Expr::parse_if(parser)?,
            })));
        }

        if parser.peek::<Token![break]>() {
            let break_keyword = parser.parse::<Token![break]>()?;
            let label = Label::parse_opt_break(parser);

            return Ok(Expr::Jump(JumpExpr::Break(ExprBreak {
                attrs,
                break_keyword,
                label,
                expr: Expr::parse_if(parser)?,
            })));
        }

        if parser.peek::<Token![continue]>() {
            let continue_keyword = parser.parse::<Token![continue]>()?;
            let label = Label::parse_opt_break(parser);

            return Ok(Expr::Jump(JumpExpr::Continue(ExprContinue {
                attrs,
                continue_keyword,
                label,
            })));
        }

        if parser.peek::<Token![let]>() {
            let let_keyword = parser.parse::<Token![let]>()?;
            let pat = Box::new(parser.parse::<Pattern>()?);
            let eq = parser.parse::<Token![=]>()?;
            let expr = Box::new(super::parse_expr(parser, false)?);

            return Ok(Expr::Primary(Self::Let(ExprLet {
                attrs,
                let_keyword,
                pat,
                eq,
                expr,
            })));
        }

        if ExprClosure::is_start(parser) {
            return Ok(Expr::Primary(Self::Closure(ExprClosure::parse_from(parser, attrs)?)));
        }

        if matches!(parser.curr(), Some(tt) if ExprLit::is_literal(tt)) {
            return Ok(Expr::Primary(Self::Lit(ExprLit {
                attrs,
                lit: parser.parse()?,
            })));
        }

        if let Some(mac) = parser.parse_if::<crate::MacroCall>() {
            return Ok(Expr::Primary(Self::Macro(ExprMacro { attrs, mac })));
        }

        // Qualified path `<T as Trait>::assoc` in expression position.
        if parser.peek::<Token![<]>() {
            let (qself, path) = crate::ty::QSelf::parse_qualified(parser)?;

            return Ok(Expr::Primary(Self::Path(ExprPath {
                attrs,
                qself: Some(qself),
                path,
            })));
        }

        if matches!(
            parser.curr(),
            Some(TokenTree::Ident(_) | TokenTree::Keyword(_) | TokenTree::Punct(Punct::PathSep(_)))
        ) {
            use crate::Path;
            let path = parser.parse::<Path>()?;

            if allow_struct && matches!(parser.curr(), Some(tt) if tt.delim() == Some(Delim::Brace)) {
                let body = Delimited::parse_brace_with(parser, |inner| {
                    let (fields, rest) = ExprStruct::parse_body(inner)?;
                    Ok(StructBody { fields, rest })
                })?;
                return Ok(Expr::Primary(Self::Struct(ExprStruct {
                    attrs,
                    qself: None,
                    path,
                    body,
                })));
            }

            return Ok(Expr::Primary(Self::Path(ExprPath {
                attrs,
                qself: None,
                path,
            })));
        }

        Err(LexError::new(at).message("expected expression").into())
    }
}
