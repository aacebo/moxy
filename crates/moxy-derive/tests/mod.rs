use moxy_derive::ToTokens;

#[derive(Default, ToTokens)]
#[template {
    fn to_tokens(&self, tokens: &mut moxy_token::TokenStream) {
    }
}]
pub struct User {
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
}

#[test]
fn derive_debug() {

}
