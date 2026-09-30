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
    let fields = named
        .fields
        .iter()
        .map(|field| field.parse_meta::<Args>("build").map(|args| (field, args)))
        .collect::<Result<Vec<_>, ParseError>>()?;

    Ok(moxy::template! {
        pub struct {{ builder_name }} {
            @for (field, _) in &fields {
                {{ field.ident }}: Option<{{ field.ty }}>,
            }
        }

        impl {{ &item.ident }} {
            pub fn builder() -> {{ &builder_name }} {
                {{ builder_name }} {
                    @for (field, _) in &fields {
                        {{ field.ident }}: None,
                    }
                }
            }
        }

        impl {{ &builder_name }} {
            @for (field, args) in &fields {
                @if let Some(rename) = args.as_ref().and_then(|args| args.rename.as_deref()) {
                    {{
                        let field_ident = field.ident.as_ref().expect("named fields have identifiers");
                        let setter_ident = moxy::token::Ident::new(rename).with_span(field_ident.span());

                        moxy::template! {
                            pub fn {{ setter_ident }}(mut self, value: {{ field.ty }}) -> Self {
                                self.{{ field_ident }} = Some(value);
                                self
                            }
                        }
                    }}
                } @else {
                    pub fn {{ field.ident }}(mut self, value: {{ field.ty }}) -> Self {
                        self.{{ field.ident }} = Some(value);
                        self
                    }
                }
            }

            pub fn build(self) -> {{ &item.ident }} {
                {{ item.ident }} {
                    @for (field, args) in &fields {
                        {{ field.ident }}:
                        @if let Some(a) = args && a.is_default {
                            self.{{ field.ident }}.unwrap_or_else(|| ::core::default::Default::default())
                        } @else {
                            self.{{ field.ident }}.expect(
                                concat!("missing required field: ", stringify!({{ field.ident }})),
                            )
                        },
                    }
                }
            }
        }
    })
}
