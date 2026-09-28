use moxy::ast::ParseError;
use moxy::diagnostic::SpanExt;
use moxy::token::{Spanner, TokenStream};

#[moxy::derive(Builder)]
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
    let fields = &named.fields;

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
            @for (field in fields.iter()) {
                pub fn {{ field.ident }}(mut self, value: {{ field.ty }}) -> Self {
                    self.{{ field.ident }} = Some(value);
                    self
                }
            }

            @if (fields.is_empty()) {
                pub fn build(self) -> {{ &item.ident }} {
                    {{ item.ident }} {}
                }
            } @else {
                pub fn build(self) -> {{ &item.ident }} {
                    {{ item.ident }} {
                        @for (field in fields.iter()) {
                            {{ field.ident }}: self.{{ field.ident }}.expect(
                                concat!("missing required field: ", stringify!({{ field.ident }})),
                            ),
                        }
                    }
                }
            }
        }
    })
}
