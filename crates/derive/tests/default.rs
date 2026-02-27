use moxy_derive::Default;

#[derive(Default)]
struct User {
    #[moxy(default = "bob")]
    pub name: String,
    email: Option<String>,
}

#[test]
fn test_partial_annotation() {
    let user = User::default();

    assert_eq!(user.name, "bob");
    assert_eq!(user.email, None);
}

#[derive(Default)]
struct Server {
    #[moxy(default = "0.0.0.0")]
    pub bind: String,
    #[moxy(default = 8080u16)]
    pub port: u16,
    #[moxy(default = true)]
    pub tls: bool,
}

#[test]
fn test_all_fields_annotated() {
    let s = Server::default();

    assert_eq!(s.bind, "0.0.0.0");
    assert_eq!(s.port, 8080);
    assert_eq!(s.tls, true);
}

const MAX_RETRIES: u32 = 3;

#[derive(Default)]
struct Client {
    #[moxy(default = MAX_RETRIES)]
    pub retries: u32,
}

#[test]
fn test_constant_default() {
    let c = Client::default();
    assert_eq!(c.retries, 3);
}

#[derive(Default)]
struct Collection {
    #[moxy(default = Vec::new())]
    pub items: Vec<String>,
    #[moxy(default = "prefix".to_string())]
    pub label: String,
}

#[test]
fn test_expression_default() {
    let c = Collection::default();

    assert!(c.items.is_empty());
    assert_eq!(c.label, "prefix");
}

#[derive(Default)]
struct Container<T: std::default::Default> {
    #[moxy(default = 10u32)]
    pub capacity: u32,
    pub value: T,
}

#[test]
fn test_generic_struct() {
    let c: Container<String> = Container::default();

    assert_eq!(c.capacity, 10);
    assert_eq!(c.value, String::default());
}

#[derive(Default)]
struct Pair(#[moxy(default = "hello")] String, u32);

#[test]
fn test_tuple_struct() {
    let p = Pair::default();

    assert_eq!(p.0, "hello");
    assert_eq!(p.1, 0);
}

#[derive(Default)]
struct Marker;

#[test]
fn test_unit_struct() {
    let _ = Marker::default();
}
