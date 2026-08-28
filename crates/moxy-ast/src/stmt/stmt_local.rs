use moxy_token::Token;
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::{Parse, Span, Spanner, ToTokens, TokenStream};

use crate::{Attributes, Expr, Pattern, Type};

/// A `let` binding statement.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct StmtLocal {
    pub attrs: Attributes,
    pub let_keyword: Token![let],
    pub pat: Pattern,
    pub ty: Option<(Token![:], Type)>,
    pub init: Option<StmtLocalInit>,
    pub semi: Option<Token![;]>,
}

/// The initializer of a `let` binding.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct StmtLocalInit {
    pub eq: Token![=],
    pub expr: Expr,
    pub diverge: Option<(Token![else], Box<Expr>)>,
}

impl Parse for StmtLocal {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse::<Attributes>()?;
        let let_keyword = stream.parse::<Token![let]>()?;
        let pat = stream.parse::<Pattern>()?;
        let ty = if stream.peek::<Token![:]>() {
            let colon = stream.parse::<Token![:]>()?;
            Some((colon, stream.parse::<Type>()?))
        } else {
            None
        };

        let init = if stream.peek::<Token![=]>() {
            let eq = stream.parse::<Token![=]>()?;
            let expr = stream.parse::<Expr>()?;

            let diverge = if stream.peek::<Token![else]>() {
                let else_keyword = stream.parse::<Token![else]>()?;
                Some((else_keyword, Box::new(stream.parse::<Expr>()?)))
            } else {
                None
            };

            Some(StmtLocalInit { eq, expr, diverge })
        } else {
            None
        };

        let semi = stream.parse_if::<Token![;]>();

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
        let end = if let Some(s) = &self.semi {
            s.span()
        } else if let Some(init) = &self.init {
            init.span()
        } else {
            self.pat.span()
        };

        self.attrs.span().join(end)
    }
}

impl ToTokens for StmtLocal {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
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

impl StmtLocal {
    pub fn into_stmt(self) -> super::Stmt {
        super::Stmt::Local(Box::new(self))
    }
}
