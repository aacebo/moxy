use moxy_token::{Delim, Ident, Span, Spanner, ToTokenStream, ToTokens, TokenStream, TokenTree};

use crate::*;

/// A const generic parameter (`const N: usize = 0`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ConstParam {
    pub attrs: Attributes,
    pub const_keyword: Token![const],
    pub ident: Ident,
    pub colon_punct: Token![:],
    pub ty: Type,
    pub default_eq_punct: Option<Token![=]>,
    pub default: Option<Expr>,
}

impl Attributed for ConstParam {
    fn attrs(&self) -> &[Attribute] {
        &self.attrs
    }
}

impl Parse for ConstParam {
    fn peek(cursor: Cursor<'_>) -> bool {
        Attributes::skip(cursor)
            .map(|cursor| <Token![const]>::peek(cursor))
            .unwrap_or(false)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = <_ as Parse>::parse(parser)?;
        let const_keyword = <_ as Parse>::parse(parser)?;
        let ident = <_ as Parse>::parse(parser)?;
        let colon_punct = <_ as Parse>::parse(parser)?;
        let ty = <_ as Parse>::parse(parser)?;
        let (default_eq_punct, default) = if <Token![=]>::peek(parser.cursor()) {
            let eq_punct = <_ as Parse>::parse(parser)?;
            let start = parser.cursor();
            // A default is a block, a literal, a negated literal, or a name,
            // which ends before the `>` that closes the parameters
            let expr = expr::parse::unary(parser, Attributes::default())?;

            if !simple_default(parser.cursor().range(start).iter()) {
                return Err(ParseError::new(
                    expr.span(),
                    "expressions must be enclosed in braces to be used as const generic arguments",
                ));
            }

            (Some(eq_punct), Some(expr))
        } else {
            (None, None)
        };

        Ok(Self {
            attrs,
            const_keyword,
            ident,
            colon_punct,
            ty,
            default_eq_punct,
            default,
        })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        cursor = Attributes::skip(cursor)?;
        cursor = <Token![const]>::skip(cursor)?;
        cursor = Ident::skip(cursor)?;
        cursor = <Token![:]>::skip(cursor)?;
        cursor = Type::skip(cursor)?;

        if <Token![=]>::peek(cursor) {
            let start = <Token![=]>::skip(cursor)?;
            cursor = expr::skip::pattern_bound(start)?;

            if !simple_default(cursor.range(start).iter()) {
                return None;
            }
        }

        Some(cursor)
    }
}

impl Spanner for ConstParam {
    fn span(&self) -> Span {
        self.to_token_stream().span()
    }
}

impl ToTokens for ConstParam {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.const_keyword.to_tokens(t);
        self.ident.to_tokens(t);
        self.colon_punct.to_tokens(t);
        self.ty.to_tokens(t);
        self.default_eq_punct.to_tokens(t);
        self.default.to_tokens(t);
    }
}

// Whether the tokens of a const parameter's default are a block, a literal, a
// negated literal, or a name, as rustc requires: `= A::B` and `= (1)` need
// braces
fn simple_default<'a>(mut tokens: impl Iterator<Item = &'a TokenTree>) -> bool {
    let first = tokens.next();
    let second = tokens.next();

    match (first, second, tokens.next()) {
        (Some(TokenTree::Group(group)), None, _) => group.delim == Delim::Brace,
        (Some(TokenTree::Literal(_) | TokenTree::Ident(_)), None, _) => true,
        (Some(name), None, _) => name.is_keyword_self_type() || name.is_keyword_self_value(),
        (Some(minus), Some(TokenTree::Literal(_)), None) => minus.is_punct_minus(),
        _ => false,
    }
}
