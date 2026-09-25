use moxy_token::{Punct, TokenTree};
use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::{AngleArguments, Cursor, Expr, GenericArgument, Ident, Parse, ParseError, Parser, Token};

/// An associated const binding (`N = 8`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
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
        let Some(cursor) = Ident::skip(cursor) else {
            return false;
        };

        let Some(cursor) = Option::<AngleArguments>::skip(cursor) else {
            return false;
        };

        let Some(cursor) = <Token![=]>::skip(cursor) else {
            return false;
        };

        match cursor.curr() {
            Some(TokenTree::Literal(_)) => true,
            Some(TokenTree::Group(group)) => group.delim.is_brace(),
            Some(TokenTree::Punct(Punct::Minus(_) | Punct::Not(_))) => true,
            _ => false,
        }
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
        cursor = Ident::skip(cursor)?;
        cursor = Option::<AngleArguments>::skip(cursor)?;
        cursor = <Token![=]>::skip(cursor)?;
        Expr::skip(cursor)
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
        self.generics.to_tokens(t);
        self.eq_punct.to_tokens(t);
        self.expr.to_tokens(t);
    }
}
