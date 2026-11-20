use moxy_token::{Ident, Span, Spanner, ToTokens, TokenStream};

use crate::{Cursor, GenericArgument, Parse, ParseError, Parser, PathArguments, Token, Type};

/// An associated type binding (`Item = T`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct AssocTypeArgument {
    pub ident: Ident,
    /// The arguments after the name: `<T>` in `Item<T> = U` or `(T)` in `Item(T) = U`.
    pub args: PathArguments,
    pub eq_punct: Token![=],
    pub ty: Type,
}

impl AssocTypeArgument {
    pub fn to_generic_argument(&self) -> GenericArgument {
        GenericArgument::AssocType(Box::new(self.clone()))
    }

    pub fn into_generic_argument(self) -> GenericArgument {
        GenericArgument::AssocType(Box::new(self))
    }
}

impl Parse for AssocTypeArgument {
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

        Type::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            ident: <_ as Parse>::parse(parser)?,
            args: <_ as Parse>::parse(parser)?,
            eq_punct: <_ as Parse>::parse(parser)?,
            ty: <_ as Parse>::parse(parser)?,
        })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = Ident::skip(cursor)?;
        cursor = PathArguments::skip(cursor)?;
        cursor = <Token![=]>::skip(cursor)?;
        Type::skip(cursor)
    }
}

impl Spanner for AssocTypeArgument {
    fn span(&self) -> Span {
        self.ident.span().join(self.ty.span())
    }
}

impl ToTokens for AssocTypeArgument {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.ident.to_tokens(t);
        self.args.to_tokens(t);
        self.eq_punct.to_tokens(t);
        self.ty.to_tokens(t);
    }
}
