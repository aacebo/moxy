use moxy_derive::ToTokens;
use moxy_token::ToTokenStream;

#[derive(Default, ToTokens)]
#[template {
    println!("email: {}", &self.email);
}]
pub struct User {
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
}

#[derive(Default, ToTokens)]
#[debug]
#[template {
    println!("email: {}", {{ self.email }});
}]
pub struct User2 {
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
}

#[test]
fn derive_debug() {
    let user = User {
        name: String::default(),
        email: String::from("test"),
        phone: None,
    };

    assert_eq!(
        user.to_token_stream().to_string(),
        r#"println ! ("email: {}" , & self . email) ;"#,
    )
}

#[test]
fn derive_interp_string() {
    let user = User2 {
        name: String::default(),
        email: String::from("test"),
        phone: None,
    };

    assert_eq!(user.to_token_stream().to_string(), r#"println ! ("email: {}" , "test") ;"#,)
}
