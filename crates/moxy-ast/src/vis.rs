use moxy_token::{Token, Delim, Span, Spanner, ToTokens, TokenStream};

use crate::{Parse, Peek, ParseError, Parser, Delimited, Path};

/// The visibility of an item (`pub`, `pub`, `pub(in path)`, or inherited).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Visibility {
    Inherited,
    Public {
        pub_keyword: Token![pub],
    },
    Crate {
        pub_keyword: Token![pub],
        crate_keyword: Delimited<Token![crate]>,
    },
    SelfValue {
        pub_keyword: Token![pub],
        self_keyword: Delimited<Token![self]>,
    },
    Super {
        pub_keyword: Token![pub],
        super_keyword: Delimited<Token![super]>,
    },
    Restricted {
        pub_keyword: Token![pub],
        path: Delimited<(Token![in], Path)>,
    },
}

impl Visibility {
    pub fn is_inherited(&self) -> bool {
        matches!(self, Self::Inherited)
    }

    pub fn is_public(&self) -> bool {
        matches!(self, Self::Public { .. })
    }

    pub fn is_crate(&self) -> bool {
        matches!(self, Self::Crate { .. })
    }

    pub fn is_self_value(&self) -> bool {
        matches!(self, Self::SelfValue { .. })
    }

    pub fn is_super(&self) -> bool {
        matches!(self, Self::Super { .. })
    }

    pub fn is_restricted(&self) -> bool {
        matches!(self, Self::Restricted { .. })
    }
}

impl Parse for Visibility {
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        if !parser.peek::<Token![pub]>() {
            return Ok(Self::Inherited);
        }

        let pub_keyword = parser.parse::<Token![pub]>()?;

        // `pub(...)` restricted forms.
        if matches!(parser.curr(), Some(tt) if tt.delim() == Some(Delim::Paren)) {
            let (span, inner) = parser.parse_group_spanned(Delim::Paren)?;
            let inner = Parser::from_tokens(&inner);

            if inner.peek::<Token![crate]>() {
                let crate_keyword = inner.parse::<Token![crate]>()?;

                return Ok(Self::Crate {
                    pub_keyword,
                    crate_keyword: Delimited::paren(span, crate_keyword),
                });
            }

            if inner.peek::<Token![self]>() {
                let self_keyword = inner.parse::<Token![self]>()?;

                return Ok(Self::SelfValue {
                    pub_keyword,
                    self_keyword: Delimited::paren(span, self_keyword),
                });
            }

            if inner.peek::<Token![super]>() {
                let super_keyword = inner.parse::<Token![super]>()?;

                return Ok(Self::Super {
                    pub_keyword,
                    super_keyword: Delimited::paren(span, super_keyword),
                });
            }

            let in_keyword = inner.parse::<Token![in]>()?;
            let path = inner.parse::<Path>()?;

            return Ok(Self::Restricted {
                pub_keyword,
                path: Delimited::paren(span, (in_keyword, path)),
            });
        }

        Ok(Self::Public { pub_keyword })
    }
}

impl Spanner for Visibility {
    fn span(&self) -> Span {
        match self {
            Self::Inherited => Span::call_site(),
            Self::Public { pub_keyword } => pub_keyword.span(),
            Self::Crate {
                pub_keyword,
                crate_keyword,
            } => pub_keyword.span().join(crate_keyword.span()),
            Self::SelfValue {
                pub_keyword,
                self_keyword,
            } => pub_keyword.span().join(self_keyword.span()),
            Self::Super {
                pub_keyword,
                super_keyword,
            } => pub_keyword.span().join(super_keyword.span()),
            Self::Restricted { pub_keyword, path } => pub_keyword.span().join(path.close()),
        }
    }
}

impl ToTokens for Visibility {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            Self::Inherited => {}
            Self::Public { pub_keyword } => pub_keyword.to_tokens(t),
            Self::Crate {
                pub_keyword,
                crate_keyword,
            } => {
                pub_keyword.to_tokens(t);
                crate_keyword.to_tokens(t);
            }
            Self::SelfValue {
                pub_keyword,
                self_keyword,
            } => {
                pub_keyword.to_tokens(t);
                self_keyword.to_tokens(t);
            }
            Self::Super {
                pub_keyword,
                super_keyword,
            } => {
                pub_keyword.to_tokens(t);
                super_keyword.to_tokens(t);
            }
            Self::Restricted { pub_keyword, path } => {
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
