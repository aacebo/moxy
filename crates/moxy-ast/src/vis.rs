use moxy_token::keyword::{Crate, In, Pub, SelfValue, Super};
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::{Delim, Parse, Span, Spanner, ToTokens, TokenStream, TokenTree};

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
        crate_keyword: Delimited<Crate>,
    },
    SelfValue {
        pub_keyword: Pub,
        self_keyword: Delimited<SelfValue>,
    },
    Super {
        pub_keyword: Pub,
        super_keyword: Delimited<Super>,
    },
    Restricted {
        pub_keyword: Pub,
        path: Delimited<(In, Path)>,
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
                    crate_keyword: Delimited::paren(span, crate_keyword),
                });
            }

            if inner.peek::<SelfValue>().is_some() {
                let self_keyword = inner.parse::<SelfValue>()?;

                return Ok(Visibility::SelfValue {
                    pub_keyword,
                    self_keyword: Delimited::paren(span, self_keyword),
                });
            }

            if inner.peek::<Super>().is_some() {
                let super_keyword = inner.parse::<Super>()?;

                return Ok(Visibility::Super {
                    pub_keyword,
                    super_keyword: Delimited::paren(span, super_keyword),
                });
            }

            let in_keyword = inner.parse::<In>()?;
            let path = inner.parse::<Path>()?;

            return Ok(Visibility::Restricted {
                pub_keyword,
                path: Delimited::paren(span, (in_keyword, path)),
            });
        }

        Ok(Visibility::Public { pub_keyword })
    }
}

impl Spanner for Visibility {
    fn span(&self) -> Span {
        match self {
            Visibility::Inherited => Span::call_site(),
            Visibility::Public { pub_keyword } => pub_keyword.span(),
            Visibility::Crate {
                pub_keyword,
                crate_keyword,
            } => pub_keyword.span().join(crate_keyword.span()),
            Visibility::SelfValue {
                pub_keyword,
                self_keyword,
            } => pub_keyword.span().join(self_keyword.span()),
            Visibility::Super {
                pub_keyword,
                super_keyword,
            } => pub_keyword.span().join(super_keyword.span()),
            Visibility::Restricted { pub_keyword, path } => pub_keyword.span().join(path.close()),
        }
    }
}

impl ToTokens for Visibility {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            Visibility::Inherited => {}
            Visibility::Public { pub_keyword } => pub_keyword.to_tokens(t),
            Visibility::Crate {
                pub_keyword,
                crate_keyword,
            } => {
                pub_keyword.to_tokens(t);
                crate_keyword.to_tokens(t);
            }
            Visibility::SelfValue {
                pub_keyword,
                self_keyword,
            } => {
                pub_keyword.to_tokens(t);
                self_keyword.to_tokens(t);
            }
            Visibility::Super {
                pub_keyword,
                super_keyword,
            } => {
                pub_keyword.to_tokens(t);
                super_keyword.to_tokens(t);
            }
            Visibility::Restricted { pub_keyword, path } => {
                pub_keyword.to_tokens(t);
                let mut inner = TokenStream::new();
                let (in_keyword, p) = &path.inner;
                in_keyword.to_tokens(&mut inner);
                p.to_tokens(&mut inner);
                let mut group = moxy_token::Group::new(Delim::Paren, inner);
                group.set_span(path.span);
                t.extend_one(moxy_token::TokenTree::Group(group));
            }
        }
    }
}
