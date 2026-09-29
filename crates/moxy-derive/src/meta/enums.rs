use super::*;

pub fn expand(target: &moxy::ast::ItemEnum) -> TokenStream {
    let mut variants = Vec::new();

    for variant in target.variants.iter() {
        match variants::Variant::parse(variant) {
            Ok(variant) => variants.push(variant),
            Err(err) => {
                let diagnostic = err.to_compile_error();

                return moxy::template! {
                    {{ diagnostic }}
                    unreachable!()
                };
            }
        }
    }

    let arms = variants.iter().map(expand_variant).collect::<Vec<_>>();

    moxy::template! {
        let entry = <::moxy::ast::Meta as ::moxy::ast::Parse>::parse(parser)?;

        if !parser.is_empty() {
            return Err(parser.error("expected exactly one enum meta variant"));
        }

        let Some(name) = entry.path.as_ident() else {
            return Err(::moxy::ast::ParseError::new(
                ::moxy::token::Spanner::span(&entry),
                "expected a simple enum variant name",
            ));
        };

        @for (arm in arms.iter()) {
            {{ arm }}
        }

        Err(::moxy::ast::ParseError::new(
            ::moxy::token::Spanner::span(&entry),
            "unknown enum meta variant",
        ))
    }
}

fn expand_variant(variant: &variants::Variant) -> TokenStream {
    let key = &variant.key;
    let ident = &variant.ident;
    let body = match &variant.kind {
        variants::Kind::Unit => moxy::template! {
            let ::moxy::ast::MetaContent::Unit = &entry.content else {
                return Err(::moxy::ast::ParseError::new(
                    ::moxy::token::Spanner::span(&entry),
                    "unit enum variant does not accept a value",
                ));
            };

            Ok(Self::{{ ident }})
        },
        variants::Kind::Newtype(ty) => moxy::template! {
            let value_parser = match &entry.content {
                ::moxy::ast::MetaContent::List(group) => ::moxy::ast::Parser::from_tokens(&group.tokens),
                ::moxy::ast::MetaContent::Expr { expr, .. } => ::moxy::ast::Parser::from_tokens(expr),
                ::moxy::ast::MetaContent::Unit => {
                    return Err(::moxy::ast::ParseError::new(
                        ::moxy::token::Spanner::span(&entry),
                        "enum variant requires a value",
                    ));
                }
            };

            let value = <{{ ty }} as ::moxy::ast::Parse>::parse(&value_parser)?;

            if !value_parser.is_empty() {
                return Err(value_parser.error("unexpected trailing enum variant input"));
            }

            Ok(Self::{{ ident }}(value))
        },
        variants::Kind::Named(fields) => expand_named(ident, fields),
    };

    moxy::template! {
        if name == {{ key }} {
            return { {{ body }} };
        }
    }
}

fn expand_named(ident: &Ident, fields: &[fields::Field]) -> TokenStream {
    moxy::template! {
        let ::moxy::ast::MetaContent::List(group) = &entry.content else {
            return Err(::moxy::ast::ParseError::new(
                ::moxy::token::Spanner::span(&entry),
                "named enum variant requires parenthesized arguments",
            ));
        };
        let parser = ::moxy::ast::Parser::from_tokens(&group.tokens);
        let entries = ::moxy::ast::Punctuated::<
            ::moxy::ast::Meta,
            ::moxy::ast::Token![,],
        >::parse_terminated(&parser)?;

        @for (field in fields) {
            let mut {{ field.binding }}: Option<{{ field.ty }}> = None;
        }

        for entry in entries {
            let Some(name) = entry.path.as_ident() else {
                return Err(::moxy::ast::ParseError::new(
                    ::moxy::token::Spanner::span(&entry),
                    "expected a simple meta argument name",
                ));
            };

            @for (field in fields) {
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

        Ok(Self::{{ ident }} {
            @for (field in fields) {
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
