use moxy_token::{Ident, Span, Spanner, ToTokenStream, ToTokens, TokenStream};

use crate::{Cursor, GenericArgument, List, Parse, ParseError, Parser, PathArguments, Token, TypeBound};

/// An associated type bound constraint (`Item: Bound`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ConstraintArgument {
    pub ident: Ident,
    /// The arguments after the name: `<T>` in `Item<T>: Bound` or `(T)` in `Item(T): Bound`.
    pub args: PathArguments,
    pub colon_punct: Token![:],
    pub bounds: List<TypeBound, Token![+]>,
}

impl ConstraintArgument {
    /// The nested nodes an associated type bound counts as against
    /// [`MAX_NESTED_NODES`](moxy_token::MAX_NESTED_NODES), half a level each
    /// of the levels its parse takes: the bound as a generic argument, one
    /// level, and its bounds, two more levels, as in `A<B: A<B: C>>`.
    pub const NESTED_NODES: usize = 2;

    /// The nested nodes the bounds of an associated type bound count as
    /// beyond [`ConstraintArgument::NESTED_NODES`].
    pub const BOUNDS_NESTED_NODES: usize = 2 * crate::args::CONSTRAINT_EXTRA_LEVELS;

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

        let Some(cursor) = PathArguments::skip(cursor) else {
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
            args: <_ as Parse>::parse(parser)?,
            colon_punct: <_ as Parse>::parse(parser)?,
            bounds: List::parse_while_present(parser)?,
        })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = Ident::skip(cursor)?;
        cursor = PathArguments::skip(cursor)?;
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
        self.args.to_tokens(t);
        self.colon_punct.to_tokens(t);
        self.bounds.to_tokens(t);
    }
}
