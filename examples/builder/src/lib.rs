use moxy::ast::{ItemStruct, ParseError};
use moxy::token::TokenStream;

#[moxy::attribute]
pub fn builder(_meta: TokenStream, tokens: TokenStream) -> Result<TokenStream, ParseError> {
    let item = moxy::parse!(tokens as ItemStruct)?;

    let Some(named) = item.fields.as_named() else {
        return Ok(moxy::error!(
            "builder requires a struct with named fields",
            span = item.ident.span(),
            [moxy::help!("write `struct Config { host: String }`")]
        )
        .emit());
    };

    let fields = &named.fields.inner;
    let builder_name = moxy::token::ident!(format!("{}Builder", item.ident.text()));
    let empty = fields.is_empty();

    Ok(moxy::template! {
        {{ item }}

        pub struct {{ builder_name }} {
            @for (field in fields) {
                {{ field.ident }}: Option<{{ field.ty }}>,
            }
        }

        impl {{ &item.ident }} {
            pub fn builder() -> {{ &builder_name }} {
                {{ builder_name }} {
                    @for (field in fields) {
                        {{ field.ident }}: None,
                    }
                }
            }
        }

        impl {{ &builder_name }} {
            @for (field in fields) {
                pub fn {{ field.ident }}(mut self, value: {{ field.ty }}) -> Self {
                    self.{{ field.ident }} = Some(value);
                    self
                }
            }

            @if (empty) {
                pub fn build(self) -> {{ &item.ident }} {
                    {{ item.ident }} {}
                }
            } @else {
                pub fn build(self) -> {{ &item.ident }} {
                    {{ item.ident }} {
                        @for (field in fields) {
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
