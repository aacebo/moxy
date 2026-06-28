use moxy_derive::ToTokens;

#[derive(Default, ToTokens)]
#[template]
pub struct User {
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
}

#[test]
fn derive_debug() {
    User::default();
}
