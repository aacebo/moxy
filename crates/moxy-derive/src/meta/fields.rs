use super::*;

pub struct Field {
    pub member: Ident,
    pub binding: Ident,
    pub key: LitStr,
    pub ty: moxy::ast::Type,
    pub default: Option<TokenStream>,
}

impl Field {
    pub fn parse(field: &moxy::ast::Field, index: usize) -> Result<Self, moxy::ast::ParseError> {
        let Some(member) = field.ident.clone() else {
            return Err(moxy::ast::ParseError::new(field.span(), "`meta` requires named fields"));
        };

        let mut key = LitStr::new(member.text(), member.span());
        let mut default = None;

        for attr in &field.attrs {
            let Some(ident) = attr.path.as_ident() else {
                continue;
            };

            if ident != "meta" {
                continue;
            }

            let moxy::ast::MetaContent::List(group) = &attr.content else {
                return Err(moxy::ast::ParseError::new(attr.span(), "expected `#[meta(...)]`"));
            };

            let parser = moxy::ast::Parser::from_tokens(&group.tokens);
            let rules = Punctuated::<MetaRule, moxy::ast::Token![,]>::parse_terminated(&parser)?;

            for rule in rules {
                match rule {
                    MetaRule::Rename(name) => {
                        if key.value() != member.text() {
                            return Err(moxy::ast::ParseError::new(name.span(), "duplicate `rename` rule"));
                        }

                        key = name;
                    }
                    MetaRule::Default(value) => {
                        if default.is_some() {
                            return Err(moxy::ast::ParseError::new(
                                value.as_ref().map(Spanner::span).unwrap_or(attr.span()),
                                "duplicate `default` rule",
                            ));
                        }
                        default = Some(match value {
                            Some(value) => value.to_token_stream(),
                            None => moxy::template! {
                                <{{ &field.ty }} as ::std::default::Default>::default()
                            },
                        });
                    }
                }
            }
        }

        Ok(Self {
            member,
            key,
            ty: field.ty.clone(),
            default,
            binding: Ident::new(format!("__moxy_field_{index}")).with_span(field.span()),
        })
    }
}
