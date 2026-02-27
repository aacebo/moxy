use moxy_derive::Set;

#[derive(Set)]
struct Config {
    #[moxy(set)]
    host: String,
    #[moxy(set)]
    port: u16,
    #[allow(dead_code)]
    read_only: bool,
}

#[test]
fn test_basic_setter() {
    let mut cfg = Config {
        host: String::new(),
        port: 0,
        read_only: false,
    };

    cfg.set_host("localhost");
    cfg.set_port(8080_u16);

    assert_eq!(cfg.host, "localhost");
    assert_eq!(cfg.port, 8080);
}

#[test]
fn test_chaining() {
    let mut cfg = Config {
        host: String::new(),
        port: 0,
        read_only: false,
    };

    cfg.set_host("localhost").set_port(8080_u16);

    assert_eq!(cfg.host, "localhost");
    assert_eq!(cfg.port, 8080);
}

#[derive(Set)]
struct Profile {
    #[moxy(set)]
    bio: Option<String>,
}

#[test]
fn test_option_setter() {
    let mut p = Profile { bio: None };
    p.set_bio("hello");
    assert_eq!(p.bio, Some("hello".to_string()));
}

#[derive(Set)]
struct Row {
    #[moxy(set("update_id"))]
    row_id: u64,
}

#[test]
fn test_custom_name() {
    let mut r = Row { row_id: 0 };
    r.update_id(99_u64);
    assert_eq!(r.row_id, 99);
}

#[derive(Set)]
struct Normalized {
    #[moxy(set(on = value.to_lowercase()))]
    host: String,
}

#[test]
fn test_on_transform() {
    let mut n = Normalized {
        host: String::new(),
    };
    n.set_host("LOCALHOST");
    assert_eq!(n.host, "localhost");
}

static mut SET_CALLBACK_COUNT: u32 = 0;

#[derive(Set)]
struct Observed {
    #[moxy(set(on = { unsafe { SET_CALLBACK_COUNT += 1 }; value }))]
    value: String,
}

#[test]
fn test_on_side_effect() {
    let mut o = Observed {
        value: String::new(),
    };

    unsafe { SET_CALLBACK_COUNT = 0 };
    o.set_value("a");
    assert_eq!(o.value, "a");
    assert_eq!(unsafe { SET_CALLBACK_COUNT }, 1);

    o.set_value("b");
    assert_eq!(o.value, "b");
    assert_eq!(unsafe { SET_CALLBACK_COUNT }, 2);
}

#[derive(Set)]
struct WithDocs {
    /// The host address
    #[moxy(set)]
    host: String,
}

#[test]
fn test_doc_forwarding() {
    let mut w = WithDocs {
        host: String::new(),
    };
    w.set_host("localhost");
    assert_eq!(w.host, "localhost");
}
