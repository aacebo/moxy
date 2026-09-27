use moxy::ast::ParseError;
use moxy::token::TokenStream;

#[moxy::function]
pub fn apply(tokens: TokenStream) -> Result<TokenStream, ParseError> {
    let item = moxy::parse!(tokens as moxy::ast::ItemEnum)?;

    Ok(moxy::template! {
        {{ item }}

        impl {{ &item.ident }} {
            pub fn as_str(&self) -> &'static str {
                match self {
                    @for (variant in item.variants.iter()) {
                        Self::{{ variant.ident }} => stringify!({{ variant.ident }}),
                    }
                }
            }
        }
    })
}
