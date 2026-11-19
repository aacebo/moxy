use moxy_token::{Ident, Span, Spanner, ToTokenStream, ToTokens, TokenStream};

use crate::{AngleArguments, Cursor, GenericArgument, List, Parse, ParseError, Parser, Token, TypeBound};

/// An associated type bound constraint (`Item: Bound`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ConstraintArgument {
    pub ident: Ident,
    pub generics: Option<AngleArguments>,
    pub colon_punct: Token![:],
    pub bounds: List<TypeBound, Token![+]>,
}

impl ConstraintArgument {
    pub fn to_generic_argument(&self) -> GenericArgument {
        GenericArgument::Constraint(Box::new(self.clone()))
    }

    pub fn into_generic_argument(self) -> GenericArgument {
        GenericArgument::Constraint(Box::new(self))
    }
}

impl Parse for ConstraintArgument {
    fn peek(cursor: Cursor<'_>) -> bool {
        let Some(cursor) = Ident::skip(cursor) else {
            return false;
        };

        let Some(cursor) = Option::<AngleArguments>::skip(cursor) else {
            return false;
        };

        let Some(cursor) = <Token![:]>::skip(cursor) else {
            return false;
        };

        TypeBound::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            ident: <_ as Parse>::parse(parser)?,
            generics: <_ as Parse>::parse(parser)?,
            colon_punct: <_ as Parse>::parse(parser)?,
            bounds: List::parse_while_present(parser)?,
        })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = Ident::skip(cursor)?;
        cursor = Option::<AngleArguments>::skip(cursor)?;
        cursor = <Token![:]>::skip(cursor)?;
        List::<TypeBound, Token![+]>::skip_while_present(cursor)
    }
}

impl Spanner for ConstraintArgument {
    fn span(&self) -> Span {
        self.to_token_stream().span()
    }
}

impl ToTokens for ConstraintArgument {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.ident.to_tokens(t);
        self.generics.to_tokens(t);
        self.colon_punct.to_tokens(t);
        self.bounds.to_tokens(t);
    }
}
