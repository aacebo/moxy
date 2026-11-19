#![allow(unused)]

use std::str::FromStr;

use moxy_ast::{Cursor, Parse, ParseError, Parser};
use moxy_token::{Delim, Lit, Punct, Span, ToTokens, TokenStream, TokenTree};

#[doc = "Literal passthrough tokens in a template: any tokens not matched by interpolation or control flow."]
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug))]
pub struct TmplTokens {
    pub span: Span,
    pub tokens: TokenStream,
}

impl Parse for TmplTokens {
    fn peek(cursor: Cursor<'_>) -> bool {
        match cursor.curr() {
            Some(TokenTree::Punct(Punct::At(_))) if super::directive_at(cursor) => false,
            None => false,
            Some(TokenTree::Group(group)) => !super::Node::is_interp_group(group) && !super::Node::group_has_interp(group),
            Some(_) => true,
        }
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let span = parser.span();
        let mut tokens = TokenStream::new();

        while let Some(token) = parser.curr() {
            if matches!(token, TokenTree::Punct(Punct::At(_))) && super::directive_at(parser.cursor()) {
                break;
            }

            if let TokenTree::Group(group) = token
                && (super::Node::is_interp_group(group) || super::Node::group_has_interp(group))
            {
                break;
            }

            tokens.extend_one(token.clone());
            parser.advance();
        }

        if tokens.is_empty() {
            return parser.error("expected template tokens").into();
        }

        Ok(Self { span, tokens })
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        let mut skipped = false;

        while let Some(token) = cursor.curr() {
            if matches!(token, TokenTree::Punct(Punct::At(_))) && super::directive_at(cursor) {
                break;
            }

            if let TokenTree::Group(group) = token
                && (super::Node::is_interp_group(group) || super::Node::group_has_interp(group))
            {
                break;
            }

            cursor = cursor.offset(1);
            skipped = true;
        }

        skipped.then_some(cursor)
    }
}

impl ToTokens for TmplTokens {
    fn to_tokens(&self, out: &mut TokenStream) {
        let mut plain = TokenStream::new();

        for token in self.tokens.iter() {
            match token {
                // Text cannot carry a `$crate` identifier or an invisible group
                // from a `macro_rules!` fragment, so these are built as tokens,
                // as are literals that would not lex again
                TokenTree::Ident(ident) if ident.text() == DOLLAR_CRATE => {
                    emit_text(&std::mem::take(&mut plain), out);
                    out.extend(
                        TokenStream::from_str(&format!(
                            "::moxy::token::ToTokens::to_tokens(&::moxy::token::Ident::new({DOLLAR_CRATE:?}), &mut __moxy_tmpl);"
                        ))
                        .unwrap(),
                    );
                }
                // A literal the lexer does not read, which the compiler gave
                TokenTree::Literal(Lit::Verbatim(literal)) => {
                    emit_text(&std::mem::take(&mut plain), out);
                    out.extend(
                        TokenStream::from_str(&format!(
                            "::moxy::token::ToTokens::to_tokens(&::moxy::token::Lit::Verbatim(::moxy::token::LitVerbatim::new({:?}, ::moxy::token::Span::call_site())), &mut __moxy_tmpl);",
                            literal.repr()
                        ))
                        .unwrap(),
                    );
                }
                TokenTree::Group(group) if needs_tokens(token) => {
                    emit_text(&std::mem::take(&mut plain), out);
                    let inner = Self {
                        span: self.span,
                        tokens: group.tokens.clone(),
                    };
                    super::emit_group(group.delim, &inner, out);
                }
                other => plain.extend_one(other.clone()),
            }
        }

        emit_text(&plain, out);
    }
}

const DOLLAR_CRATE: &str = "$crate";

// Whether a token is, or holds, a token that only survives when built as a token
fn needs_tokens(token: &TokenTree) -> bool {
    match token {
        TokenTree::Ident(ident) => ident.text() == DOLLAR_CRATE,
        TokenTree::Literal(literal) => literal.is_verbatim(),
        TokenTree::Group(group) => group.delim == Delim::None || group.tokens.iter().any(needs_tokens),
        _ => false,
    }
}

// The literal tokens are lexed once per thread and copied on each expansion;
// their spans point into the thread's source map, so the copy stays valid
fn emit_text(tokens: &TokenStream, out: &mut TokenStream) {
    if tokens.is_empty() {
        return;
    }

    let src = tokens.to_string();

    out.extend(
        TokenStream::from_str(&format!(
            "{{ ::std::thread_local! {{ static __MOXY_TOKENS: ::moxy::token::TokenStream = ::moxy::token::TokenStream::from_string({src:?}.to_owned()).expect(\"moxy template literal tokens must lex\"); }} __MOXY_TOKENS.with(|tokens| ::moxy::token::ToTokens::to_tokens(tokens, &mut __moxy_tmpl)); }}"
        ))
        .unwrap(),
    );
}
