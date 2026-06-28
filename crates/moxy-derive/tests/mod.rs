use moxy_derive::ToTokens;

#[derive(Default, ToTokens)]
#[debug]
#[template {

}]
pub struct User {
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
}

#[test]
fn derive_debug() {
    User::default();
}
