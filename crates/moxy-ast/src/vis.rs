use moxy_token::keyword::{Crate, In, Pub, SelfValue, Super};
use moxy_token::parse::{ParseError, ParseStream};
use moxy_token::{Delim, Group, Parse, ToTokens, TokenStream, TokenTree};

use crate::Path;

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
        crate_keyword: Crate,
    },
    SelfValue {
        pub_keyword: Pub,
        self_keyword: SelfValue,
    },
    Super {
        pub_keyword: Pub,
        super_keyword: Super,
    },
    Restricted {
        pub_keyword: Pub,
        in_keyword: Option<In>,
        path: Path,
    },
}

impl Parse for Visibility {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        if stream.peek::<Pub>().is_none() {
            return Ok(Visibility::Inherited);
        }
        let pub_keyword = stream.parse::<Pub>()?;

        // `pub(...)` restricted forms.
        if matches!(stream.curr(), Some(TokenTree::Group(g)) if g.delim() == Delim::Paren) {
            let group = stream.parse_group(Delim::Paren)?;
            let mut inner = group.parse();

            if inner.peek::<Crate>().is_some() {
                let crate_keyword = inner.parse::<Crate>()?;
                return Ok(Visibility::Crate {
                    pub_keyword,
                    crate_keyword,
                });
            }

            if inner.peek::<SelfValue>().is_some() {
                let self_keyword = inner.parse::<SelfValue>()?;
                return Ok(Visibility::SelfValue {
                    pub_keyword,
                    self_keyword,
                });
            }

            if inner.peek::<Super>().is_some() {
                let super_keyword = inner.parse::<Super>()?;
                return Ok(Visibility::Super {
                    pub_keyword,
                    super_keyword,
                });
            }

            // `pub(in path)`
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
            Visibility::Crate {
                pub_keyword,
                crate_keyword,
            } => {
                pub_keyword.to_tokens(t);
                let mut inner = TokenStream::new();
                crate_keyword.to_tokens(&mut inner);
                t.extend_one(TokenTree::Group(Group::new(Delim::Paren, inner)));
            }
            Visibility::SelfValue {
                pub_keyword,
                self_keyword,
            } => {
                pub_keyword.to_tokens(t);
                let mut inner = TokenStream::new();
                self_keyword.to_tokens(&mut inner);
                t.extend_one(TokenTree::Group(Group::new(Delim::Paren, inner)));
            }
            Visibility::Super {
                pub_keyword,
                super_keyword,
            } => {
                pub_keyword.to_tokens(t);
                let mut inner = TokenStream::new();
                super_keyword.to_tokens(&mut inner);
                t.extend_one(TokenTree::Group(Group::new(Delim::Paren, inner)));
            }
            Visibility::Restricted {
                pub_keyword,
                in_keyword,
                path,
            } => {
                pub_keyword.to_tokens(t);
                let mut inner = TokenStream::new();

                if let Some(in_kw) = in_keyword {
                    in_kw.to_tokens(&mut inner);
                }

                path.to_tokens(&mut inner);
                t.extend_one(TokenTree::Group(Group::new(Delim::Paren, inner)));
            }
        }
    }
}
