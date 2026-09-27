use moxy::ast::ParseError;
use moxy::token::TokenStream;

#[moxy::function]
pub fn render(_tokens: TokenStream) -> Result<TokenStream, ParseError> {
    Ok(moxy::template! {
        println!("world, hello")
    })
}
