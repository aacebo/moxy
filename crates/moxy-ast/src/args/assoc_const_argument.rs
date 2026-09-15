use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::{AngleArguments, Cursor, Expr, GenericArgument, Ident, Parse, ParseError, Parser, Token};

/// An associated const binding (`N = 8`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct AssocConstArgument {
    pub ident: Ident,
    pub generics: Option<AngleArguments>,
    pub eq_punct: Token![=],
    pub expr: Expr,
}

impl AssocConstArgument {
    pub fn to_generic_argument(&self) -> GenericArgument {
        GenericArgument::AssocConst(self.clone())
    }

    pub fn into_generic_argument(self) -> GenericArgument {
        GenericArgument::AssocConst(self)
    }
}

impl Parse for AssocConstArgument {
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor.peek::<Ident>() && (cursor.offset(1).peek::<AngleArguments>() || cursor.offset(1).peek::<Token![=]>())
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            ident: parser.parse()?,
            generics: parser.parse()?,
            eq_punct: parser.parse()?,
            expr: parser.parse()?,
        })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = cursor.skip::<Ident>()?;
        cursor = cursor.skip::<Option<AngleArguments>>()?;
        cursor = cursor.skip::<Token![=]>()?;
        cursor.skip::<Expr>()
    }
}

impl Spanner for AssocConstArgument {
    fn span(&self) -> Span {
        self.ident.span().join(self.expr.span())
    }
}

impl ToTokens for AssocConstArgument {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.ident.to_tokens(t);

        if let Some(g) = &self.generics {
            g.to_tokens(t);
        }

        self.eq_punct.to_tokens(t);
        self.expr.to_tokens(t);
    }
}
