use moxy_derive::Build;

// ── Test structs ─────────────────────────────────────────────────────────────

/// All fields annotated — basic case.
#[derive(Build, Default)]
pub struct Config {
    #[moxy(build)]
    pub host: String,
    #[moxy(build)]
    pub port: u16,
}

/// Partial annotation — `timeout` is not annotated and receives `Default::default()`.
#[derive(Build, Default)]
pub struct Connection {
    #[moxy(build)]
    pub host: String,
    #[moxy(build)]
    pub port: u16,
    pub timeout: u64,
}

/// Generic struct — type parameter must flow through the builder.
#[derive(Build, Default)]
pub struct Wrapper<T: Default> {
    #[moxy(build)]
    pub value: T,
}

/// No fields annotated — builder compiles and build() succeeds with all defaults.
#[derive(Build, Default)]
pub struct Empty {
    pub count: u32,
}

// ── Tests ─────────────────────────────────────────────────────────────────────

/// All fields annotated; fluent chaining produces correct values.
#[test]
fn test_all_fields_build() {
    let config = Config::new().host("localhost").port(8080_u16).build();

    assert_eq!(config.host, "localhost");
    assert_eq!(config.port, 8080);
}

/// Non-annotated field receives its Default value (0u64).
#[test]
fn test_partial_fields_default() {
    let conn = Connection::new().host("127.0.0.1").port(5432_u16).build();

    assert_eq!(conn.host, "127.0.0.1");
    assert_eq!(conn.port, 5432);
    assert_eq!(conn.timeout, 0u64);
}

/// Generated builder type follows the `<Name>Builder` naming convention.
#[test]
fn test_builder_naming_convention() {
    let _: ConfigBuilder = Config::new();
    let _: ConnectionBuilder = Connection::new();
    let _: WrapperBuilder<u32> = Wrapper::new();
    let _: EmptyBuilder = Empty::new();
}

/// Type parameter flows through the generic builder to the built value.
#[test]
fn test_generic_struct() {
    let w: Wrapper<u32> = Wrapper::new().value(42u32).build();
    assert_eq!(w.value, 42u32);
}

/// When no fields are annotated the builder still compiles and build() succeeds.
#[test]
fn test_no_annotated_fields() {
    let e = Empty::new().build();
    assert_eq!(e.count, 0u32);
}

/// Custom builder method name overrides the field name.
#[derive(Build, Default)]
pub struct Credentials {
    #[moxy(build("username"))]
    pub user: String,
    #[moxy(build)]
    pub password: String,
}

#[test]
fn test_custom_method_name() {
    let c = Credentials::new()
        .username("alice")
        .password("secret")
        .build();

    assert_eq!(c.user, "alice");
    assert_eq!(c.password, "secret");
}

/// Default value provided inline — field is optional in the builder.
#[derive(Build, Default)]
pub struct Server {
    #[moxy(build(default = "localhost"))]
    pub host: String,
    #[moxy(build(default = 8080u16))]
    pub port: u16,
    #[moxy(build)]
    pub name: String,
}

#[test]
fn test_default_value_used_when_unset() {
    let s = Server::new().name("api").build();
    assert_eq!(s.host, "localhost");
    assert_eq!(s.port, 8080u16);
    assert_eq!(s.name, "api");
}

#[test]
fn test_default_value_overridden() {
    let s = Server::new()
        .host("example.com")
        .port(443u16)
        .name("web")
        .build();
    assert_eq!(s.host, "example.com");
    assert_eq!(s.port, 443u16);
}

/// Expression default — arbitrary expression is evaluated each time build() is called.
#[derive(Build, Default)]
pub struct Collection {
    #[moxy(build(default = Vec::new()))]
    pub items: Vec<String>,
    #[moxy(build(default = "prefix".to_string()))]
    pub prefix: String,
}

#[test]
fn test_expression_default() {
    let c = Collection::new().build();
    assert!(c.items.is_empty());
    assert_eq!(c.prefix, "prefix");
}

#[test]
fn test_expression_default_overridden() {
    let c = Collection::new()
        .items(vec!["a".to_string()])
        .prefix("custom")
        .build();
    assert_eq!(c.items, vec!["a".to_string()]);
    assert_eq!(c.prefix, "custom");
}

/// Custom method name combined with a default value.
#[derive(Build, Default)]
pub struct Service {
    #[moxy(build("addr", default = "0.0.0.0"))]
    pub address: String,
}

#[test]
fn test_custom_name_with_default() {
    let s = Service::new().build();
    assert_eq!(s.address, "0.0.0.0");
}

#[test]
fn test_custom_name_with_default_overridden() {
    let s = Service::new().addr("127.0.0.1").build();
    assert_eq!(s.address, "127.0.0.1");
}

/// Option<T> field is optional — setter accepts T, wraps in Some.
#[derive(Build, Default)]
pub struct Profile {
    #[moxy(build)]
    pub name: String,
    #[moxy(build)]
    pub bio: Option<String>,
    #[moxy(build)]
    pub age: Option<u32>,
}

#[test]
fn test_option_field_unset() {
    let p = Profile::new().name("alice").build();

    assert_eq!(p.name, "alice");
    assert_eq!(p.bio, None);
    assert_eq!(p.age, None);
}

#[test]
fn test_option_field_set() {
    let p = Profile::new().name("alice").bio("hello").age(30u32).build();

    assert_eq!(p.name, "alice");
    assert_eq!(p.bio, Some("hello".to_string()));
    assert_eq!(p.age, Some(30));
}

/// Constant used as a default value.
const DEFAULT_RETRIES: u32 = 3;

#[derive(Build, Default)]
pub struct Client {
    #[moxy(build(default = DEFAULT_RETRIES))]
    pub retries: u32,
}

#[test]
fn test_constant_default() {
    let c = Client::new().build();
    assert_eq!(c.retries, 3);
}

#[test]
fn test_constant_default_overridden() {
    let c = Client::new().retries(10u32).build();
    assert_eq!(c.retries, 10);
}
