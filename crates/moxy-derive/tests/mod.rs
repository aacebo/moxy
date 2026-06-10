use moxy_derive::ToTokens;

#[derive(ToTokens)]
#[template {
    impl User { }
}]
pub struct User {
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
}

#[test]
fn derive_debug() {}
