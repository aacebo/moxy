use crate::ast::{Attribute, Expr, Pattern, Type};
use crate::parse::{ParseError, ParseStream};
use crate::token::ToTokens;
use crate::token::keyword::{Else, Let};
use crate::token::punct::{Colon, Eq, Semi};
use crate::{Parse, Span, TokenStream};

#[doc = "A `let` binding statement."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct StmtLocal {
    pub span: Span,
    pub attrs: Vec<Attribute>,
    pub pat: Pattern,
    pub ty: Option<Type>,
    pub init: Option<StmtLocalInit>,
}

#[doc = "The initializer of a `let` binding."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct StmtLocalInit {
    pub span: Span,
    pub expr: Expr,
    pub diverge: Option<Box<Expr>>,
}

impl Parse for StmtLocal {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let attrs = stream.parse_vec::<Attribute>()?;
        let _ = stream.parse::<Let>()?;
        let pat = stream.parse::<Pattern>()?;

        let ty = if stream.peek::<Colon>().is_some() {
            let _ = stream.parse::<Colon>()?;
            Some(stream.parse::<Type>()?)
        } else {
            None
        };

        let init = if stream.peek::<Eq>().is_some() {
            let _ = stream.parse::<Eq>()?;
            let expr = stream.parse::<Expr>()?;

            let diverge = if stream.peek::<Else>().is_some() {
                let _ = stream.parse::<Else>()?;
                Some(Box::new(stream.parse::<Expr>()?))
            } else {
                None
            };

            Some(StmtLocalInit {
                span: Span::default(),
                expr,
                diverge,
            })
        } else {
            None
        };

        let _ = stream.parse::<Semi>();
        Ok(Self {
            span: Span::default(),
            attrs,
            pat,
            ty,
            init,
        })
    }
}

impl ToTokens for StmtLocal {
    fn to_tokens(&self, t: &mut TokenStream) {
        for a in &self.attrs {
            a.to_tokens(t);
        }
        Let::default().to_tokens(t);
        self.pat.to_tokens(t);

        if let Some(ty) = &self.ty {
            Colon::default().to_tokens(t);
            ty.to_tokens(t);
        }

        if let Some(init) = &self.init {
            Eq::default().to_tokens(t);
            init.expr.to_tokens(t);

            if let Some(div) = &init.diverge {
                Else::default().to_tokens(t);
                div.to_tokens(t);
            }
        }

        Semi::default().to_tokens(t);
    }
}
