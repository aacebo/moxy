use moxy_token::{Delim, Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A tuple-struct pattern, e.g. `Point(x, y)`.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct PatTupleStruct {
    pub attrs: Attributes,
    pub qself: Option<QSelf>,
    pub path: Path,
    pub elems: Delimited<Punctuated<Pattern, Token![,]>>,
}

impl Spanner for PatTupleStruct {
    fn span(&self) -> Span {
        self.attrs.span().join(self.elems.span())
    }
}

impl Parse for PatTupleStruct {
    fn peek(cursor: Cursor<'_>) -> bool {
        cursor.peek::<QSelf>() || cursor.peek::<Path>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = parser.parse()?;
        let qself = parser.parse()?;
        let path = parser.parse()?;
        let (span, parser) = parser.parse_group_spanned(Delim::Paren)?;

        Ok(Self {
            attrs,
            qself,
            path,
            elems: Delimited::paren(span, Punctuated::parse_separated_nonempty(&parser)?),
        })
    }
}

impl ToTokens for PatTupleStruct {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.qself.to_tokens(t);
        self.path.to_tokens(t);
        self.elems.to_tokens(t);
    }
}
