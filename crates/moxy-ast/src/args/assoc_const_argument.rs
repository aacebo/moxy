use moxy_token::{Ident, Punct, Span, Spanner, ToTokens, TokenStream, TokenTree};

use crate::{Cursor, Expr, GenericArgument, Parse, ParseError, Parser, PathArguments, Token};

/// An associated const binding (`N = 8`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct AssocConstArgument {
    pub ident: Ident,
    /// The arguments after the name: `<T>` in `N<T> = 8` or `(T)` in `N(T) = 8`.
    pub args: PathArguments,
    pub eq_punct: Token![=],
    pub expr: Expr,
}

impl AssocConstArgument {
    /// The nested nodes an associated binding counts as against
    /// [`MAX_NESTED_NODES`](moxy_token::MAX_NESTED_NODES), half a level each
    /// of the levels its parse takes: the binding as a generic argument, one
    /// level, and its expression, one more level, as in `A<B = A<B = u8>>`.
    pub const NESTED_NODES: usize = 2;

    /// The nested nodes the expression of an associated binding counts as beyond
    /// [`AssocConstArgument::NESTED_NODES`].
    pub const EXPR_NESTED_NODES: usize = 2 * crate::args::ASSOC_EXTRA_LEVELS;

    pub fn to_generic_argument(&self) -> GenericArgument {
        GenericArgument::AssocConst(Box::new(self.clone()))
    }

    pub fn into_generic_argument(self) -> GenericArgument {
        GenericArgument::AssocConst(Box::new(self))
    }
}

impl Parse for AssocConstArgument {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(cursor) = Ident::skip(cursor) else {
            return false;
        };

        let Some(cursor) = PathArguments::skip(cursor) else {
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
            ident: <_ as Parse>::parse(parser)?,
            args: <_ as Parse>::parse(parser)?,
            eq_punct: <_ as Parse>::parse(parser)?,
            expr: <_ as Parse>::parse(parser)?,
        })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = Ident::skip(cursor)?;
        cursor = PathArguments::skip(cursor)?;
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
        self.args.to_tokens(t);
        self.eq_punct.to_tokens(t);
        self.expr.to_tokens(t);
    }
}
