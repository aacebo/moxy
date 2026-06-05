use moxy_token::keyword::{Else, Let};
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::{Colon, Eq, Semi};
use moxy_token::{Parse, Span, Spanner, ToTokens, TokenStream};

use crate::{Attribute, Expr, Pattern, Type};

#[doc = "A `let` binding statement."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct StmtLocal {
    pub attrs: Vec<Attribute>,
    pub let_keyword: Let,
    pub pat: Pattern,
    pub ty: Option<(Colon, Type)>,
    pub init: Option<StmtLocalInit>,
    pub semi: Option<Semi>,
}

#[doc = "The initializer of a `let` binding."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct StmtLocalInit {
    pub eq: Eq,
    pub expr: Expr,
    pub diverge: Option<(Else, Box<Expr>)>,
}

impl Parse for StmtLocal {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse::<Vec<Attribute>>()?;
        let let_keyword = stream.parse::<Let>()?;
        let pat = stream.parse::<Pattern>()?;

        let ty = if stream.peek::<Colon>().is_some() {
            let colon = stream.parse::<Colon>()?;
            Some((colon, stream.parse::<Type>()?))
        } else {
            None
        };

        let init = if stream.peek::<Eq>().is_some() {
            let eq = stream.parse::<Eq>()?;
            let expr = stream.parse::<Expr>()?;

            let diverge = if stream.peek::<Else>().is_some() {
                let else_keyword = stream.parse::<Else>()?;
                Some((else_keyword, Box::new(stream.parse::<Expr>()?)))
            } else {
                None
            };

            Some(StmtLocalInit { eq, expr, diverge })
        } else {
            None
        };

        let semi = stream.parse_if::<Semi>();
        Ok(Self {
            attrs,
            let_keyword,
            pat,
            ty,
            init,
            semi,
        })
    }
}

impl Spanner for StmtLocalInit {
    fn span(&self) -> Span {
        self.eq.span().join(self.expr.span())
    }
}

impl Spanner for StmtLocal {
    fn span(&self) -> Span {
        let start = if let Some(a) = self.attrs.first() {
            a.span()
        } else {
            self.let_keyword.span()
        };
        let end = if let Some(s) = &self.semi {
            s.span()
        } else if let Some(init) = &self.init {
            init.span()
        } else {
            self.pat.span()
        };
        start.join(end)
    }
}

impl ToTokens for StmtLocal {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        self.let_keyword.to_tokens(t);
        self.pat.to_tokens(t);

        if let Some((colon, ty)) = &self.ty {
            colon.to_tokens(t);
            ty.to_tokens(t);
        }

        if let Some(init) = &self.init {
            init.eq.to_tokens(t);
            init.expr.to_tokens(t);

            if let Some((else_keyword, div)) = &init.diverge {
                else_keyword.to_tokens(t);
                div.to_tokens(t);
            }
        }

        self.semi.to_tokens(t);
    }
}
