use moxy::ast::{Attributed, ParseError};
use moxy::diagnostic::SpanExt;
use moxy::token::{Spanner, TokenStream};

#[derive(moxy::FromMeta)]
struct Args {
    #[meta(default)]
    rename: Option<String>,

    #[meta(rename = "default", default)]
    is_default: bool,
}

#[moxy::derive(Builder, attributes(build))]
pub fn builder(declaration: moxy::ast::Declaration) -> Result<TokenStream, ParseError> {
    let moxy::ast::Declaration::Struct(item) = &declaration else {
        return declaration.span().error("invalid host type, expected struct").into();
    };

    let Some(named) = item.fields.as_named() else {
        return Ok(moxy::error!(
            "builder requires a struct with named fields",
            span = item.ident.span(),
            [moxy::help!("write `struct Config { host: String }`")]
        )
        .emit());
    };

    let builder_name = moxy::token::ident!(format!("{}Builder", item.ident.text()));
    let mut fields = Vec::new();
    let mut initializers: Vec<TokenStream> = Vec::new();
    let mut setters: Vec<TokenStream> = Vec::new();

    for field in named.fields.iter() {
        let args = field.parse_meta::<Args>("build")?;
        let field_ident = field.ident.as_ref().expect("named fields have identifiers");
        let setter_ident = args
            .as_ref()
            .and_then(|args| args.rename.as_deref())
            .map(|rename| moxy::token::Ident::new(rename).with_span(field_ident.span()))
            .unwrap_or_else(|| field_ident.clone());
        let tokens = if let Some(args) = &args
            && args.is_default
        {
            moxy::template! {
                {{ field.ident }}: self.{{ field.ident }}.unwrap_or_else(|| ::core::default::Default::default())
            }
        } else {
            moxy::template! {
                {{ field.ident }}: self.{{ field.ident }}.expect(
                    concat!("missing required field: ", stringify!({{ field.ident }})),
                )
            }
        };

        fields.push(field);
        initializers.push(tokens);
        setters.push(moxy::template! {
            pub fn {{ setter_ident }}(mut self, value: {{ field.ty }}) -> Self {
                self.{{ field_ident }} = Some(value);
                self
            }
        });
    }

    Ok(moxy::template! {
        pub struct {{ builder_name }} {
            @for (field in fields.iter()) {
                {{ field.ident }}: Option<{{ field.ty }}>,
            }
        }

        impl {{ &item.ident }} {
            pub fn builder() -> {{ &builder_name }} {
                {{ builder_name }} {
                    @for (field in fields.iter()) {
                        {{ field.ident }}: None,
                    }
                }
            }
        }

        impl {{ &builder_name }} {
            @for (setter in setters.iter()) {
                {{ setter }}
            }

            pub fn build(self) -> {{ &item.ident }} {
                {{ item.ident }} {
                    @for (initializer in initializers.iter()) {
                        {{ initializer }},
                    }
                }
            }
        }
    })
}
