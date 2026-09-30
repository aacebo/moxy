use super::*;

pub enum Kind {
    Unit,
    Newtype(moxy::ast::Type),
    Named(Vec<fields::Field>),
}

pub struct Variant {
    pub ident: Ident,
    pub key: LitStr,
    pub kind: Kind,
    pub message: Option<LitStr>,
}

impl Variant {
    pub fn parse(variant: &moxy::ast::Variant) -> Result<Self, moxy::ast::ParseError> {
        if variant.discriminant.is_some() {
            return Err(moxy::ast::ParseError::new(
                variant.span(),
                "`Meta` enum variants cannot have discriminants",
            ));
        }

        let snake = variant.ident.to_snake_case();
        let mut key = LitStr::new(snake.text(), variant.ident.span());
        let mut renamed = false;
        let mut message = None;

        for attr in &variant.attrs {
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
                        if renamed {
                            return Err(moxy::ast::ParseError::new(name.span(), "duplicate `rename` rule"));
                        }

                        renamed = true;
                        key = name;
                    }
                    MetaRule::Default(value) => {
                        return Err(moxy::ast::ParseError::new(
                            value.as_ref().map(Spanner::span).unwrap_or(attr.span()),
                            "`default` is not supported on enum variants",
                        ));
                    }
                    MetaRule::Message(value) => {
                        if message.is_some() {
                            return Err(moxy::ast::ParseError::new(value.span(), "duplicate `message` rule"));
                        }

                        message = Some(value);
                    }
                }
            }
        }

        let kind = match &variant.fields {
            moxy::ast::Fields::Unit => Kind::Unit,
            moxy::ast::Fields::Unnamed(fields) => {
                let values = fields.fields.iter().collect::<Vec<_>>();

                if values.len() != 1 {
                    return Err(moxy::ast::ParseError::new(
                        variant.fields.span(),
                        "`Meta` tuple variants must contain exactly one field",
                    ));
                }

                Kind::Newtype(values[0].ty.clone())
            }
            moxy::ast::Fields::Named(fields) => {
                let mut parsed = Vec::new();

                for (index, field) in fields.fields.iter().enumerate() {
                    parsed.push(fields::Field::parse(field, index)?);
                }

                Kind::Named(parsed)
            }
        };

        Ok(Self {
            ident: variant.ident.clone(),
            key,
            kind,
            message,
        })
    }
}
