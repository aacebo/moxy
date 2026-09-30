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
    let message = variant.message.as_ref();
    let body = match &variant.kind {
        variants::Kind::Unit => moxy::template! {
            let ::moxy::ast::MetaContent::Unit = &entry.content else {
                return Err({{ parse_error(
                    message,
                    key,
                    moxy::template! { ::moxy::token::Spanner::span(&entry) },
                    moxy::template! { ::moxy::token::ToTokenStream::to_token_stream(&entry.path) },
                    "unit enum variant does not accept a value",
                ) }});
            };

            Ok(Self::{{ ident }})
        },
        variants::Kind::Newtype(ty) => moxy::template! {
            let value = <{{ ty }} as ::moxy::ast::FromMeta>::from_meta(&entry)?;

            Ok(Self::{{ ident }}(value))
        },
        variants::Kind::Named(fields) => expand_named(ident, fields, message, key),
    };

    moxy::template! {
        if name == {{ key }} {
            return { {{ body }} };
        }
    }
}

fn expand_named(ident: &Ident, fields: &[fields::Field], message: Option<&LitStr>, key: &LitStr) -> TokenStream {
    moxy::template! {
        let ::moxy::ast::MetaContent::List(group) = &entry.content else {
            return Err({{ parse_error(
                message,
                key,
                moxy::template! { ::moxy::token::Spanner::span(&entry) },
                moxy::template! { ::moxy::token::ToTokenStream::to_token_stream(&entry.path) },
                "named enum variant requires parenthesized arguments",
            ) }});
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
                        return Err({{ parse_error(
                            field.message.as_ref(),
                            &field.key,
                            moxy::template! { ::moxy::token::Spanner::span(&entry) },
                            moxy::template! { ::moxy::token::ToTokenStream::to_token_stream(&entry.path) },
                            "duplicate meta argument",
                        ) }});
                    }

                    let value = <{{ field.ty }} as ::moxy::ast::FromMeta>::from_meta(&entry)?;

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
                                    return Err({{ parse_error(
                                        field.message.as_ref(),
                                        &field.key,
                                        moxy::template! { ::moxy::token::Span::call_site() },
                                        moxy::template! { {{ &field.key }} },
                                        "missing required meta argument",
                                    ) }});
                                }
                            }
                        },
                    }
                }},
            }
        })
    }
}
