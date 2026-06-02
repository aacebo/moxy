use crate::ast::Path;
use crate::parse::{ParseError, ParseStream};
use crate::token::keyword::{Crate, In, Pub, SelfValue, Super};
use crate::token::{Delim, Group, ToTokens};
use crate::{Parse, TokenStream, TokenTree};

#[doc = "The visibility of an item (`pub`, `pub`, `pub(in path)`, or inherited)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Visibility {
    Inherited,
    Public,
    Crate,
    SelfValue,
    Super,
    Restricted { in_token: bool, path: Path },
}

impl Parse for Visibility {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        if stream.peek::<Pub>().is_none() {
            return Ok(Visibility::Inherited);
        }
        let _ = stream.parse::<Pub>()?;

        // `pub(...)` restricted forms.
        if matches!(stream.curr(), Some(TokenTree::Group(g)) if g.delim() == Delim::Paren) {
            let group = stream.parse_group(Delim::Paren)?;
            let mut inner = group.parse();

            if inner.peek::<Crate>().is_some() {
                return Ok(Visibility::Crate);
            }

            if inner.peek::<SelfValue>().is_some() {
                return Ok(Visibility::SelfValue);
            }

            if inner.peek::<Super>().is_some() {
                return Ok(Visibility::Super);
            }

            // `pub(in path)`
            let in_token = if inner.peek::<In>().is_some() {
                let _ = inner.parse::<In>()?;
                true
            } else {
                false
            };

            let path = inner.parse::<Path>()?;
            return Ok(Visibility::Restricted { in_token, path });
        }

        Ok(Visibility::Public)
    }
}

impl ToTokens for Visibility {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            Visibility::Inherited => {}
            Visibility::Public => Pub::default().to_tokens(t),
            Visibility::Crate => {
                Pub::default().to_tokens(t);
                let mut inner = TokenStream::new();
                Crate::default().to_tokens(&mut inner);
                t.extend_one(TokenTree::Group(Group::new(Delim::Paren, inner)));
            }
            Visibility::SelfValue => {
                Pub::default().to_tokens(t);
                let mut inner = TokenStream::new();
                SelfValue::default().to_tokens(&mut inner);
                t.extend_one(TokenTree::Group(Group::new(Delim::Paren, inner)));
            }
            Visibility::Super => {
                Pub::default().to_tokens(t);
                let mut inner = TokenStream::new();
                Super::default().to_tokens(&mut inner);
                t.extend_one(TokenTree::Group(Group::new(Delim::Paren, inner)));
            }
            Visibility::Restricted { in_token, path } => {
                Pub::default().to_tokens(t);
                let mut inner = TokenStream::new();

                if *in_token {
                    In::default().to_tokens(&mut inner);
                }

                path.to_tokens(&mut inner);
                t.extend_one(TokenTree::Group(Group::new(Delim::Paren, inner)));
            }
        }
    }
}
