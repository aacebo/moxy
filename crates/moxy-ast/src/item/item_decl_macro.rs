use moxy_token::{Delim, Group, Ident, Span, Spanner, ToTokenStream, ToTokens, TokenStream, TokenTree};

use crate::*;

/// A declarative macro item of the `macro` form: `macro m($x:expr) { $x }`,
/// with parameters and a body, or `macro m { (a) => {}, (b) => {} }`, with
/// rules. rustc's parser accepts it and the compiler gates it on the
/// `decl_macro` feature.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemDeclMacro {
    pub attrs: Attributes,
    pub vis: Visibility,
    pub macro_keyword: Token![macro],
    pub ident: Ident,
    /// The parameters in parentheses of the form with one body.
    pub params: Option<Group>,
    /// The body in braces, or the rules when there are no parameters.
    pub body: Group,
}

impl Attributed for ItemDeclMacro {
    fn attrs(&self) -> &[Attribute] {
        &self.attrs
    }
}

impl Parse for ItemDeclMacro {
    fn peek(cursor: Cursor<'_>) -> bool {
        let cursor = Attributes::skip(cursor).unwrap_or(cursor);
        let cursor = Visibility::skip(cursor).unwrap_or(cursor);
        <Token![macro]>::peek(cursor) && Ident::peek(cursor.offset(1))
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = <_ as Parse>::parse(parser)?;
        let vis = <_ as Parse>::parse(parser)?;
        let macro_keyword = <_ as Parse>::parse(parser)?;
        let ident = <_ as Parse>::parse(parser)?;
        let params = if parser.is_delimited(Delim::Paren) {
            Some(<_ as Parse>::parse(parser)?)
        } else {
            None
        };

        if !parser.is_delimited(Delim::Brace) {
            let expected = if params.is_some() { "`{`" } else { "`(` or `{`" };
            return parser.expected(expected).into();
        }

        Ok(Self {
            attrs,
            vis,
            macro_keyword,
            ident,
            params,
            body: <_ as Parse>::parse(parser)?,
        })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = Attributes::skip(cursor)?;
        cursor = Visibility::skip(cursor)?;
        cursor = <Token![macro]>::skip(cursor)?;
        cursor = Ident::skip(cursor)?;

        if cursor.is_delimited(Delim::Paren) {
            cursor = Group::skip(cursor)?;
        }

        if !cursor.is_delimited(Delim::Brace) {
            return None;
        }

        Group::skip(cursor)
    }
}

impl Spanner for ItemDeclMacro {
    fn span(&self) -> Span {
        self.to_token_stream().span()
    }
}

impl ToTokens for ItemDeclMacro {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.vis.to_tokens(t);
        self.macro_keyword.to_tokens(t);
        self.ident.to_tokens(t);

        if let Some(params) = &self.params {
            t.extend_one(TokenTree::Group(params.clone()));
        }

        t.extend_one(TokenTree::Group(self.body.clone()));
    }
}
