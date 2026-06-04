use crate::span::{DelimSpan, Spanner};
use crate::{Delim, Group, Span, ToTokens, TokenStream, TokenTree};

macro_rules! define_delim {
    ($($name:ident => $delim:ident),+ $(,)?) => {
        $(
            #[derive(Debug, Default, Copy, Clone)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize))]
            pub struct $name {
                pub span: DelimSpan,
            }

            impl $name {
                pub const DELIM: Delim = Delim::$delim;

                pub fn new(span: DelimSpan) -> Self {
                    Self { span }
                }

                pub fn span(&self) -> DelimSpan {
                    self.span
                }

                pub fn set_span(&mut self, span: DelimSpan) {
                    self.span = span;
                }

                /// Emit `inner` wrapped in this delimiter, preserving the stored
                /// open/close spans.
                pub fn surround(&self, tokens: &mut TokenStream, inner: TokenStream) {
                    let mut group = Group::new(Delim::$delim, inner);
                    group.set_span(self.span);
                    tokens.extend_one(TokenTree::Group(group));
                }
            }

            // Equality ignores the span so AST nodes embedding a delimiter token
            // compare structurally.
            impl PartialEq for $name {
                fn eq(&self, _: &Self) -> bool {
                    true
                }
            }

            impl Eq for $name {}

            impl std::hash::Hash for $name {
                fn hash<H: std::hash::Hasher>(&self, _: &mut H) {}
            }

            impl ToTokens for $name {
                fn to_tokens(&self, tokens: &mut TokenStream) {
                    self.surround(tokens, TokenStream::new());
                }
            }

            impl Spanner for $name {
                fn span(&self) -> Span {
                    self.span.span()
                }
            }
        )+
    };
}

define_delim! {
    Paren => Paren,
    Brace => Brace,
    Bracket => Bracket,
}
