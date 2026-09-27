use moxy::ast::ParseError;
use moxy::token::TokenStream;

#[moxy::attribute]
pub fn trace(_meta: TokenStream, item: TokenStream) -> Result<TokenStream, ParseError> {
    let item = moxy::parse!(item as moxy::ast::ItemFn)?;

    Ok(moxy::template! {
        {{ &item.sig }} {
            print!("start...");
            let result = {{ &item.body }};
            println!("end");
            result
        }
    })
}
