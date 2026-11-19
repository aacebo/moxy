use moxy_token::{Ident, Span, Spanner, ToTokenStream, ToTokens, TokenStream};

use crate::*;

/// An associated type inside a trait definition (`type Name: Bound = Default;`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct TraitItemType {
    pub attrs: Attributes,
    /// A visibility, which rustc's parser accepts and the compiler rejects
    /// later, as in `pub fn f();`.
    pub vis: Visibility,
    pub type_keyword: Token![type],
    pub ident: Ident,
    pub generics: Generics,
    pub colon: Option<Token![:]>,
    pub bounds: List<TypeBound, Token![+]>,
    pub default: Option<(Token![=], Type)>,
    pub semi: Token![;],
}

impl Attributed for TraitItemType {
    fn attrs(&self) -> &[Attribute] {
        &self.attrs
    }
}

impl Parse for TraitItemType {
    fn peek(cursor: Cursor<'_>) -> bool {
        Attributes::skip(cursor)
            .and_then(|cursor| Visibility::skip(cursor))
            .map(|cursor| <Token![type]>::peek(cursor) && Ident::peek(cursor.offset(1)))
            .unwrap_or(false)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = <_ as Parse>::parse(parser)?;
        let vis = <_ as Parse>::parse(parser)?;
        let type_keyword = <_ as Parse>::parse(parser)?;
        let ident = <_ as Parse>::parse(parser)?;
        let mut generics: Generics = <_ as Parse>::parse(parser)?;
        let (colon, bounds, default) = parse_type_rest(parser, &mut generics)?;
        let semi = <_ as Parse>::parse(parser)?;

        Ok(Self {
            attrs,
            vis,
            type_keyword,
            ident,
            generics,
            colon,
            bounds,
            default,
            semi,
        })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = Attributes::skip(cursor)?;
        cursor = Visibility::skip(cursor)?;
        cursor = <Token![type]>::skip(cursor)?;
        cursor = Ident::skip(cursor)?;
        cursor = Generics::skip(cursor)?;
        cursor = skip_type_rest(cursor)?;
        <Token![;]>::skip(cursor)
    }
}

/// The bounds and default of an associated or foreign type, after its generics
pub(crate) type TypeRest = (Option<Token![:]>, List<TypeBound, Token![+]>, Option<(Token![=], Type)>);

// The bounds, where clause, and default of an associated or foreign type,
// after its generics, with the where clause read into `generics`
pub(crate) fn parse_type_rest(parser: &Parser, generics: &mut Generics) -> Result<TypeRest, ParseError> {
    let (colon, bounds) = if <Token![:]>::peek(parser.cursor()) {
        let colon = <_ as Parse>::parse(parser)?;
        (Some(colon), List::parse_while_present(parser)?)
    } else {
        (None, List::new())
    };

    // A where clause follows the bounds; without bounds, `Generics` has
    // already read it
    if generics.where_clause.is_none() {
        generics.where_clause = <_ as Parse>::parse(parser)?;
    }

    let default = if <Token![=]>::peek(parser.cursor()) {
        let eq = <_ as Parse>::parse(parser)?;
        Some((eq, <_ as Parse>::parse(parser)?))
    } else {
        None
    };

    // The where clause can also follow the default, where Rust prefers it
    if default.is_some() && generics.where_clause.is_none() {
        generics.where_clause = <_ as Parse>::parse(parser)?;
    }

    Ok((colon, bounds, default))
}

pub(crate) fn skip_type_rest(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
    if <Token![:]>::peek(cursor) {
        cursor = <Token![:]>::skip(cursor)?;
        cursor = List::<TypeBound, Token![+]>::skip_while_present(cursor)?;
    }

    cursor = Option::<WhereClause>::skip(cursor)?;

    if <Token![=]>::peek(cursor) {
        cursor = <Token![=]>::skip(cursor)?;
        cursor = Type::skip(cursor)?;
        cursor = Option::<WhereClause>::skip(cursor)?;
    }

    Some(cursor)
}

// Emits the generics, bounds, default, and where clause of an associated or
// foreign type; with a default, the where clause goes after it, where Rust
// prefers it
pub(crate) fn type_rest_to_tokens(
    generics: &Generics,
    colon: &Option<Token![:]>,
    bounds: &List<TypeBound, Token![+]>,
    default: &Option<(Token![=], Type)>,
    t: &mut TokenStream,
) {
    generics.lt.to_tokens(t);
    generics.params.to_tokens(t);
    generics.gt.to_tokens(t);
    colon.to_tokens(t);
    bounds.to_tokens(t);

    if let Some((eq, d)) = default {
        eq.to_tokens(t);
        d.to_tokens(t);
    }

    generics.where_clause.to_tokens(t);
}

impl Spanner for TraitItemType {
    fn span(&self) -> Span {
        self.to_token_stream().span()
    }
}

impl ToTokens for TraitItemType {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.vis.to_tokens(t);
        self.type_keyword.to_tokens(t);
        self.ident.to_tokens(t);
        type_rest_to_tokens(&self.generics, &self.colon, &self.bounds, &self.default, t);
        self.semi.to_tokens(t);
    }
}
