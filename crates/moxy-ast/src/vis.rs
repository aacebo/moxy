use moxy_token::keyword::{Crate, In, Pub, SelfValue, Super};
use moxy_token::parse::{ParseError, ParseStream};
use moxy_token::{Delim, Parse, ToTokens, TokenStream, TokenTree};

use crate::{Delimited, Path};

#[doc = "The visibility of an item (`pub`, `pub`, `pub(in path)`, or inherited)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Visibility {
    Inherited,
    Public {
        pub_keyword: Pub,
    },
    Crate {
        pub_keyword: Pub,
        paren: Delimited<Crate>,
    },
    SelfValue {
        pub_keyword: Pub,
        paren: Delimited<SelfValue>,
    },
    Super {
        pub_keyword: Pub,
        paren: Delimited<Super>,
    },
    Restricted {
        pub_keyword: Pub,
        in_keyword: Option<In>,
        path: Path,
        paren_span: moxy_token::span::DelimSpan,
    },
}

impl Parse for Visibility {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        if stream.peek::<Pub>().is_none() {
            return Ok(Visibility::Inherited);
        }
        let pub_keyword = stream.parse::<Pub>()?;

        // `pub(...)` restricted forms.
        if matches!(stream.curr(), Some(tt) if tt.delim() == Some(Delim::Paren)) {
            let (span, group_tokens) = stream.parse_group_spanned(Delim::Paren)?;
            let mut inner = group_tokens.parse();

            if inner.peek::<Crate>().is_some() {
                let crate_keyword = inner.parse::<Crate>()?;
                return Ok(Visibility::Crate {
                    pub_keyword,
                    paren: Delimited::paren(span, crate_keyword),
                });
            }

            if inner.peek::<SelfValue>().is_some() {
                let self_keyword = inner.parse::<SelfValue>()?;
                return Ok(Visibility::SelfValue {
                    pub_keyword,
                    paren: Delimited::paren(span, self_keyword),
                });
            }

            if inner.peek::<Super>().is_some() {
                let super_keyword = inner.parse::<Super>()?;
                return Ok(Visibility::Super {
                    pub_keyword,
                    paren: Delimited::paren(span, super_keyword),
                });
            }

            let in_keyword = if inner.peek::<In>().is_some() {
                Some(inner.parse::<In>()?)
            } else {
                None
            };

            let path = inner.parse::<Path>()?;
            return Ok(Visibility::Restricted {
                pub_keyword,
                in_keyword,
                path,
                paren_span: span,
            });
        }

        Ok(Visibility::Public { pub_keyword })
    }
}

impl ToTokens for Visibility {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            Visibility::Inherited => {}
            Visibility::Public { pub_keyword } => pub_keyword.to_tokens(t),
            Visibility::Crate { pub_keyword, paren } => {
                pub_keyword.to_tokens(t);
                paren.to_tokens(t);
            }
            Visibility::SelfValue { pub_keyword, paren } => {
                pub_keyword.to_tokens(t);
                paren.to_tokens(t);
            }
            Visibility::Super { pub_keyword, paren } => {
                pub_keyword.to_tokens(t);
                paren.to_tokens(t);
            }
            Visibility::Restricted {
                pub_keyword,
                in_keyword,
                path,
                paren_span,
            } => {
                pub_keyword.to_tokens(t);
                let mut inner = TokenStream::new();
                if let Some(in_kw) = in_keyword {
                    in_kw.to_tokens(&mut inner);
                }
                path.to_tokens(&mut inner);
                let mut group = moxy_token::Group::new(Delim::Paren, inner);
                group.set_span(*paren_span);
                t.extend_one(moxy_token::TokenTree::Group(group));
            }
        }
    }
}
