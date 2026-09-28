use moxy_ast::Punctuated;
use moxy_token::TokenStream;

pub fn expand(tokens: TokenStream) -> TokenStream {
    let target = match moxy::parse!(tokens as moxy::ast::Declaration) {
        Err(err) => return err.to_compile_error(),
        Ok(v) => v,
    };

    moxy::template! {
        impl ::moxy::ast::Parse for {{ target.ident() }} {
            fn peek(cursor: ::moxy::ast::Cursor<'_>) -> bool {
                <::moxy::ast::Meta as ::moxy::ast::Parse>::peek(cursor)
            }

            fn parse(parser: &::moxy::ast::Parser) -> Result<Self, ::moxy::ast::ParseError> {
                let meta = <::moxy::ast::Meta as ::moxy::ast::Parse>::parse(parser)?;

                Ok(@match (&target) {
                    ::moxy::ast::Declaration::Struct(target) => {{{ expand_struct(target) }}},
                    _ => { compiler_error!("unsupported host type") },
                })
            }

            fn skip(cursor: ::moxy::ast::Cursor<'_>) -> Option<::moxy::ast::Cursor<'_>> {
                <::moxy::ast::Meta as ::moxy::ast::Parse>::skip(cursor)
            }
        }
    }
}

fn expand_struct(target: &moxy::ast::ItemStruct) -> TokenStream {
    moxy::template! {
        @match (&target.fields) {
            ::moxy::ast::Fields::Named(::moxy::ast::FieldsNamed { fields }) => {
                Self {
                    @for (field in fields.iter()) {
                        {{ expand_field(field) }},
                    }
                }
            },
            _ => {
                compiler_error!("tuple structs are not supported")
            }
        }
    }
}

fn expand_field(field: &moxy::ast::Field) -> TokenStream {
    let mut name = field.ident.clone().unwrap();
    let mut init = None;

    for attr in &field.attrs {
        let Some(ident) = attr.path.as_ident() else {
            continue;
        };

        if ident != "meta" {
            continue;
        }

        let moxy::ast::MetaContent::List(group) = &attr.content else {
            continue;
        };

        let parser = moxy::ast::Parser::from_tokens(&group.tokens);
        let list = match Punctuated::<MetaRule, moxy::ast::Token![,]>::parse_separated_nonempty(&parser) {
            Err(err) => return err.to_compile_error(),
            Ok(v) => v,
        };

        for rule in list {
            if let MetaRule::Rename(lit) = rule {
                name = moxy::token::Ident::new(lit.value()).with_span(lit.span());
            } else if let MetaRule::Default(expr) = rule {
                init = Some(match expr {
                    Some(v) => v,
                    None => {
                        let tokens = moxy::template! {
                            <{{ field.ty }} as ::std::default::Default>::default()
                        };

                        match moxy::parse!(tokens) {
                            Err(err) => return err.to_compile_error(),
                            Ok(v) => v,
                        }
                    }
                });
            }
        }
    }

    moxy::template! {
        {{ name }}: {{ init }}
    }
}

#[derive(Clone)]
enum MetaRule {
    Default(Option<moxy::ast::Expr>),
    Rename(moxy::token::LitStr),
}

impl moxy::token::Spanner for MetaRule {
    fn span(&self) -> moxy::token::Span {
        match self {
            Self::Default(v) => v.span(),
            Self::Rename(v) => v.span(),
        }
    }
}

impl moxy::token::ToTokens for MetaRule {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Default(v) => v.to_tokens(tokens),
            Self::Rename(v) => v.to_tokens(tokens),
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
            return parser.error("expected ident").into();
        };

        if ident == "default" {
            match &meta.content {
                moxy::ast::MetaContent::Unit => Ok(Self::Default(None)),
                moxy::ast::MetaContent::Expr { eq: _, expr } => Ok(Self::Default(Some(moxy::parse!(expr)?))),
                _ => parser.error("invalid input for rule `default`").into(),
            }
        } else if ident == "rename" {
            match &meta.content {
                moxy::ast::MetaContent::Expr { eq: _, expr } => Ok(Self::Rename(moxy::parse!(expr)?)),
                _ => parser.error("invalid input for rule `rename`").into(),
            }
        } else {
            parser.error(format!("invalid rule {ident}")).into()
        }
    }

    fn skip(cursor: moxy::ast::Cursor<'_>) -> Option<moxy::ast::Cursor<'_>> {
        moxy::ast::Meta::skip(cursor)
    }
}
