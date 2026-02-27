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

/// Calling build() without setting a required field panics.
#[test]
#[should_panic]
fn test_missing_field_panics() {
    Config::new()
        .host("localhost")
        // intentionally omit .port()
        .build();
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
    #[moxy(build(default = "localhost".to_string()))]
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
