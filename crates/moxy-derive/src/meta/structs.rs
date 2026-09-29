use super::*;

pub fn expand(target: &moxy::ast::ItemStruct) -> TokenStream {
    let moxy::ast::Fields::Named(named) = &target.fields else {
        return target.fields.span().error("`Meta` requires a named-field struct").emit();
    };

    let mut fields = Vec::new();

    for (index, field) in named.fields.iter().enumerate() {
        match fields::Field::parse(field, index) {
            Ok(field) => fields.push(field),
            Err(err) => return err.to_compile_error(),
        }
    }

    moxy::template! {
        let entries = ::moxy::ast::Punctuated::<
            ::moxy::ast::Meta,
            ::moxy::ast::Token![,],
        >::parse_terminated(parser)?;

        @for (field in &fields) {
            let mut {{ field.binding }}: Option<{{ field.ty }}> = None;
        }

        for entry in entries {
            let Some(name) = entry.path.as_ident() else {
                return Err(::moxy::ast::ParseError::new(
                    ::moxy::token::Spanner::span(&entry),
                    "expected a simple meta argument name",
                ));
            };

            @for (field in &fields) {
                if name == {{ field.key }} {
                    if {{ field.binding }}.is_some() {
                        return Err(::moxy::ast::ParseError::new(
                            ::moxy::token::Spanner::span(&entry),
                            "duplicate meta argument",
                        ));
                    }

                    let value_parser = match &entry.content {
                        ::moxy::ast::MetaContent::List(group) => ::moxy::ast::Parser::from_tokens(&group.tokens),
                        ::moxy::ast::MetaContent::Expr { expr, .. } => ::moxy::ast::Parser::from_tokens(expr),
                        ::moxy::ast::MetaContent::Unit => {
                            return Err(::moxy::ast::ParseError::new(
                                ::moxy::token::Spanner::span(&entry),
                                "expected a value for meta argument",
                            ));
                        }
                    };

                    let value = <{{ field.ty }} as ::moxy::ast::Parse>::parse(&value_parser)?;

                    if !value_parser.is_empty() {
                        return Err(value_parser.error("unexpected trailing meta argument input"));
                    }

                    {{ field.binding }} = Some(value);
                    continue;
                }
            }

            return Err(::moxy::ast::ParseError::new(
                ::moxy::token::Spanner::span(&entry),
                "unknown meta argument",
            ));
        }

        Ok(Self {
            @for (field in &fields) {
                {{
                    match &field.default {
                        Some(default) => moxy::template! {
                            {{ field.member }}: match {{ field.binding }} {
                                Some(value) => value,
                                None => {{ default }},
                            }
                        },
                        None => moxy::template! {
                            {{ field.member }}: match {{ field.binding }} {
                                Some(value) => value,
                                None => {
                                    return Err(::moxy::ast::ParseError::new(
                                        ::moxy::token::Span::call_site(),
                                        "missing required meta argument",
                                    ));
                                }
                            }
                        },
                    }
                }},
            }
        })
    }
}
