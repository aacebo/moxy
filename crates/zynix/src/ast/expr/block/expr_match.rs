use crate::ast::{Attribute, Expr, Pattern};
use crate::parse::{ParseError, ParseStream};
use crate::token::keyword::{If, Match};
use crate::token::punct::{Comma, FatArrow};
use crate::token::{Delim, Group, ToTokens};
use crate::{Parse, Span, TokenStream, TokenTree};

#[doc = "A match expression: `match x { pat => expr, ... }`."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ExprMatch {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub expr: Box<Expr>,
    pub arms: Vec<MatchArm>,
}

impl ToTokens for ExprMatch {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        Match::default().to_tokens(t);
        self.expr.to_tokens(t);
        let mut inner = TokenStream::new();
        for arm in &self.arms {
            arm.to_tokens(&mut inner);
        }
        t.extend_one(TokenTree::Group(Group::new(Delim::Brace, inner)));
    }
}

#[doc = "A single arm of a `match` expression (`pat (if guard)? => body`)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct MatchArm {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub pat: Pattern,
    pub guard: Option<Box<Expr>>,
    pub body: Expr,
}

impl Parse for MatchArm {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse_vec::<Attribute>()?;
        let pat = stream.parse::<Pattern>()?;

        let guard = if stream.peek::<If>().is_some() {
            let _ = stream.parse::<If>()?;
            Some(Box::new(stream.parse::<Expr>()?))
        } else {
            None
        };

        let _ = stream.parse::<FatArrow>()?;
        let body = stream.parse::<Expr>()?;
        let _ = stream.parse::<Comma>();
        Ok(Self {
            span: Span::default(),
            attrs,
            pat,
            guard,
            body,
        })
    }
}

impl ToTokens for MatchArm {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.pat.to_tokens(t);

        if let Some(g) = &self.guard {
            If::default().to_tokens(t);
            g.to_tokens(t);
        }

        FatArrow::default().to_tokens(t);
        self.body.to_tokens(t);
        Comma::default().to_tokens(t);
    }
}
