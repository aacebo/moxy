use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::{AngleArguments, Cursor, GenericArgument, Ident, Parse, ParseError, Parser, Token, Type};

/// An associated type binding (`Item = T`).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct AssocTypeArgument {
    pub ident: Ident,
    pub generics: Option<AngleArguments>,
    pub eq_punct: Token![=],
    pub ty: Type,
}

impl AssocTypeArgument {
    pub fn to_generic_argument(&self) -> GenericArgument {
        GenericArgument::AssocType(self.clone())
    }

    pub fn into_generic_argument(self) -> GenericArgument {
        GenericArgument::AssocType(self)
    }
}

impl Parse for AssocTypeArgument {
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor.peek::<Ident>() && (cursor.offset(1).peek::<AngleArguments>() || cursor.offset(1).peek::<Token![=]>())
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            ident: parser.parse()?,
            generics: parser.parse()?,
            eq_punct: parser.parse()?,
            ty: parser.parse()?,
        })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = cursor.skip::<Ident>()?;
        cursor = cursor.after::<Option<AngleArguments>>()?;
        cursor = cursor.skip::<Token![=]>()?;
        cursor.skip::<Type>()
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

        if let Some(g) = &self.generics {
            g.to_tokens(t);
        }

        self.eq_punct.to_tokens(t);
        self.ty.to_tokens(t);
    }
}
