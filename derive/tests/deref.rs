use moxy_derive::Deref;

#[derive(Deref)]
pub struct TupleEmail(String);

#[derive(Deref)]
pub struct Email {
    raw: String,
}

#[derive(Deref)]
pub struct User {
    pub name: String,
    #[moxy(deref)]
    pub email: String,
    pub phone: String,
}

#[test]
pub fn test_len() {
    assert_eq!(TupleEmail("testing".into()).len(), 7);
    assert_eq!(
        Email {
            raw: "testing".into()
        }
        .len(),
        7
    );
    assert_eq!(
        User {
            name: "test".into(),
            email: "testing".into(),
            phone: "".into(),
        }
        .len(),
        7
    );
}
