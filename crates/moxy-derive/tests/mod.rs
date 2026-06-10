use moxy_derive::Moxy;

#[derive(Moxy)]
#[moxy(debug)]
pub struct User {
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
}

#[test]
fn derive_debug() {}
