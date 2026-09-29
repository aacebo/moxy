use moxy_ast::Punctuated;
use moxy_diagnostic::SpanExt;
use moxy_token::{Ident, LitStr, Spanner, ToTokenStream, TokenStream};

pub fn expand(tokens: TokenStream) -> TokenStream {
    let target = match moxy::parse!(tokens as moxy::ast::Declaration) {
        Err(err) => return err.to_compile_error(),
        Ok(v) => v,
    };

    let moxy::ast::Declaration::Struct(target) = target else {
        return target.span().error("`Meta` can only be derived for structs").emit();
    };

    let body = expand_struct(&target);
    let ident = &target.ident;

    moxy::template! {
        impl ::moxy::ast::Parse for {{ ident }} {
            fn peek(cursor: ::moxy::ast::Cursor<'_>) -> bool {
                <::moxy::ast::Meta as ::moxy::ast::Parse>::peek(cursor)
            }

            fn parse(parser: &::moxy::ast::Parser) -> Result<Self, ::moxy::ast::ParseError> {
                {{ body }}
            }

            fn skip(mut cursor: ::moxy::ast::Cursor<'_>) -> Option<::moxy::ast::Cursor<'_>> {
                cursor = <::moxy::ast::Meta as ::moxy::ast::Parse>::skip(cursor)?;

                while <::moxy::ast::Token![,]>::peek(cursor) {
                    cursor = <::moxy::ast::Token![,]>::skip(cursor)?;
                    cursor = <::moxy::ast::Meta as ::moxy::ast::Parse>::skip(cursor)?;
                }

                Some(cursor)
            }
        }
    }
}

fn expand_struct(target: &moxy::ast::ItemStruct) -> TokenStream {
    let moxy::ast::Fields::Named(named) = &target.fields else {
        return target.fields.span().error("`Meta` requires a named-field struct").emit();
    };

    let mut fields = Vec::new();
    for (index, field) in named.fields.iter().enumerate() {
        match Field::parse(field, index) {
            Ok(field) => fields.push(field),
            Err(err) => return err.to_compile_error(),
        }
    }

    let bindings = fields.iter().map(Field::expand_binding).collect::<Vec<_>>();
    let matches = fields.iter().map(Field::expand_match).collect::<Vec<_>>();
    let values = fields.iter().map(Field::expand_value).collect::<Vec<_>>();

    moxy::template! {
        let entries = ::moxy::ast::Punctuated::<
            ::moxy::ast::Meta,
            ::moxy::ast::Token![,],
        >::parse_terminated(parser)?;

        @for (binding in bindings.iter()) {
            {{ binding }}
        }

        for entry in entries {
            let Some(name) = entry.path.as_ident() else {
                return Err(::moxy::ast::ParseError::new(
                    ::moxy::token::Spanner::span(&entry),
                    "expected a simple meta argument name",
                ));
            };

            @for (case in matches.iter()) {
                {{ case }}
            }

            return Err(::moxy::ast::ParseError::new(
                ::moxy::token::Spanner::span(&entry),
                "unknown meta argument",
            ));
        }

        Ok(Self {
            @for (value in values.iter()) {
                {{ value }},
            }
        })
    }
}

struct Field {
    member: Ident,
    key: LitStr,
    ty: moxy::ast::Type,
    default: Option<TokenStream>,
    binding: Ident,
}

impl Field {
    fn parse(field: &moxy::ast::Field, index: usize) -> Result<Self, moxy::ast::ParseError> {
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

    fn expand_binding(&self) -> TokenStream {
        let binding = &self.binding;
        let ty = &self.ty;

        moxy::template! {
            let mut {{ binding }}: Option<{{ ty }}> = None;
        }
    }

    fn expand_match(&self) -> TokenStream {
        let key = &self.key;
        let binding = &self.binding;
        let ty = &self.ty;

        moxy::template! {
            if name == {{ key }} {
                if {{ binding }}.is_some() {
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

                let value = <{{ ty }} as ::moxy::ast::Parse>::parse(&value_parser)?;

                if !value_parser.is_empty() {
                    return Err(value_parser.error("unexpected trailing meta argument input"));
                }

                {{ binding }} = Some(value);
                continue;
            }
        }
    }

    fn expand_value(&self) -> TokenStream {
        let member = &self.member;
        let binding = &self.binding;

        match &self.default {
            Some(default) => moxy::template! {
                {{ member }}: match {{ binding }} {
                    Some(value) => value,
                    None => {{ default }},
                }
            },
            None => moxy::template! {
                {{ member }}: match {{ binding }} {
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
    }
}

#[derive(Clone)]
enum MetaRule {
    Default(Option<moxy::ast::Expr>),
    Rename(LitStr),
}

impl Spanner for MetaRule {
    fn span(&self) -> moxy::token::Span {
        match self {
            Self::Default(value) => value.span(),
            Self::Rename(value) => value.span(),
        }
    }
}

impl moxy::ast::Parse for MetaRule {
    fn peek(cursor: moxy::ast::Cursor<'_>) -> bool {
        moxy::ast::Meta::peek(cursor)
    }

    fn parse(parser: &moxy::ast::Parser) -> Result<Self, moxy::ast::ParseError> {
        let meta = <moxy::ast::Meta as moxy::ast::Parse>::parse(parser)?;
        let Some(ident) = meta.path.as_ident() else {
            return parser.error("expected a meta rule name").into();
        };

        match ident.text() {
            "default" => match &meta.content {
                moxy::ast::MetaContent::Unit => Ok(Self::Default(None)),
                moxy::ast::MetaContent::Expr { expr, .. } => Ok(Self::Default(Some(moxy::parse!(expr)?))),
                _ => parser.error("`default` must be bare or use `=`").into(),
            },
            "rename" => match &meta.content {
                moxy::ast::MetaContent::Expr { expr, .. } => Ok(Self::Rename(moxy::parse!(expr)?)),
                _ => parser.error("`rename` must use a string literal value").into(),
            },
            _ => parser.error(format!("unknown meta rule `{ident}`")).into(),
        }
    }

    fn skip(cursor: moxy::ast::Cursor<'_>) -> Option<moxy::ast::Cursor<'_>> {
        moxy::ast::Meta::skip(cursor)
    }
}
