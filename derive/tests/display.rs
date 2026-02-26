use moxy_derive::Display;

#[derive(Display)]
pub struct NamedDefault {
    name: String,
    email: String,
}

#[derive(Display)]
pub struct TupleDefault(String, String);

#[derive(Display)]
pub struct UnitDefault;

#[derive(Display)]
pub struct Ignored {
    name: String,
    #[moxy(display(skip))]
    #[allow(unused)]
    email: String,
}

#[derive(Display)]
#[moxy(display("hi! my name is {name} and my email is {email}"))]
pub struct CustomFmt {
    name: String,
    email: String,
}

#[derive(Display)]
#[moxy(display(debug))]
pub struct DebugMode {
    name: String,
    email: String,
}

#[derive(Display)]
#[moxy(display(compact))]
pub struct CompactMode {
    name: String,
    email: String,
}

#[derive(Display)]
#[moxy(display(keyvalue))]
pub struct KeyValueMode {
    name: String,
    email: String,
}

#[derive(Display)]
#[moxy(display(map))]
pub struct MapMode {
    name: String,
    email: String,
}

#[derive(Display)]
#[moxy(display(debug))]
pub struct TupleDebug(String, i32);

#[derive(Display)]
#[moxy(display(compact))]
pub struct TupleCompact(String, i32);

#[derive(Display)]
#[moxy(display(pretty))]
pub struct DefaultPretty {
    name: String,
    email: String,
}

#[derive(Display)]
#[moxy(display(pretty))]
pub struct TupleDefaultPretty(String, i32);

#[derive(Display)]
#[moxy(display(debug, pretty))]
pub struct DebugPretty {
    name: String,
    email: String,
}

#[derive(Display)]
#[moxy(display(debug, pretty))]
pub struct TupleDebugPretty(String, i32);

#[derive(Display)]
#[moxy(display(keyvalue, pretty))]
pub struct KeyValuePretty {
    name: String,
    email: String,
}

#[derive(Display)]
#[moxy(display(map, pretty))]
pub struct MapPretty {
    name: String,
    email: String,
}

#[derive(Display)]
#[moxy(display(alias = "User"))]
pub struct AliasedStruct {
    name: String,
    email: String,
}

#[derive(Display)]
pub struct AliasedField {
    #[moxy(display(alias = "full_name"))]
    name: String,
    email: String,
}

#[derive(Display)]
#[moxy(display(debug, alias = "U"))]
pub struct AliasedDebug {
    #[moxy(display(alias = "n"))]
    name: String,
    email: String,
}

#[derive(Display)]
#[moxy(display("{}", self.name))]
pub struct SelfFieldAccess {
    name: String,
}

#[derive(Display)]
#[moxy(display("{}", self.greeting()))]
pub struct SelfMethodCall {
    name: String,
}

impl SelfMethodCall {
    fn greeting(&self) -> String {
        format!("Hello, {}!", self.name)
    }
}

#[derive(Display)]
#[moxy(display("double: {}", count * 2))]
pub struct FieldExpr {
    count: i32,
}

#[test]
fn test_named_default() {
    let u = NamedDefault {
        name: "John".into(),
        email: "john@example.com".into(),
    };
    assert_eq!(
        u.to_string(),
        "NamedDefault { name: John, email: john@example.com }"
    );
}

#[test]
fn test_tuple_default() {
    let t = TupleDefault("hello".into(), "world".into());
    assert_eq!(t.to_string(), "TupleDefault(hello, world)");
}

#[test]
fn test_unit_default() {
    assert_eq!(UnitDefault.to_string(), "UnitDefault");
}

#[test]
fn test_skip() {
    let u = Ignored {
        name: "John".into(),
        email: "john@example.com".into(),
    };
    assert_eq!(u.to_string(), "Ignored { name: John }");
}

#[test]
fn test_custom_fmt() {
    let c = CustomFmt {
        name: "John".into(),
        email: "john@example.com".into(),
    };
    assert_eq!(
        c.to_string(),
        "hi! my name is John and my email is john@example.com"
    );
}

#[test]
fn test_debug_mode() {
    let d = DebugMode {
        name: "John".into(),
        email: "john@example.com".into(),
    };
    assert_eq!(
        d.to_string(),
        "DebugMode { name: \"John\", email: \"john@example.com\" }"
    );
}

#[test]
fn test_compact_mode() {
    let c = CompactMode {
        name: "John".into(),
        email: "john@example.com".into(),
    };
    assert_eq!(c.to_string(), "John john@example.com");
}

#[test]
fn test_keyvalue_mode() {
    let k = KeyValueMode {
        name: "John".into(),
        email: "john@example.com".into(),
    };
    assert_eq!(k.to_string(), "name=John email=john@example.com");
}

#[test]
fn test_map_mode() {
    let m = MapMode {
        name: "John".into(),
        email: "john@example.com".into(),
    };
    assert_eq!(m.to_string(), "{ name: John, email: john@example.com }");
}

#[test]
fn test_tuple_debug() {
    let t = TupleDebug("hello".into(), 42);
    assert_eq!(t.to_string(), "TupleDebug(\"hello\", 42)");
}

#[test]
fn test_tuple_compact() {
    let t = TupleCompact("hello".into(), 42);
    assert_eq!(t.to_string(), "hello 42");
}

#[test]
fn test_default_pretty() {
    let d = DefaultPretty {
        name: "John".into(),
        email: "john@example.com".into(),
    };
    assert_eq!(
        d.to_string(),
        "DefaultPretty {\n    name: John,\n    email: john@example.com,\n}"
    );
}

#[test]
fn test_tuple_default_pretty() {
    let t = TupleDefaultPretty("hello".into(), 42);
    assert_eq!(t.to_string(), "TupleDefaultPretty(\n    hello,\n    42,\n)");
}

#[test]
fn test_debug_pretty() {
    let d = DebugPretty {
        name: "John".into(),
        email: "john@example.com".into(),
    };
    assert_eq!(
        d.to_string(),
        "DebugPretty {\n    name: \"John\",\n    email: \"john@example.com\",\n}"
    );
}

#[test]
fn test_tuple_debug_pretty() {
    let t = TupleDebugPretty("hello".into(), 42);
    assert_eq!(
        t.to_string(),
        "TupleDebugPretty(\n    \"hello\",\n    42,\n)"
    );
}

#[test]
fn test_keyvalue_pretty() {
    let k = KeyValuePretty {
        name: "John".into(),
        email: "john@example.com".into(),
    };
    assert_eq!(k.to_string(), "name=John\nemail=john@example.com");
}

#[test]
fn test_map_pretty() {
    let m = MapPretty {
        name: "John".into(),
        email: "john@example.com".into(),
    };
    assert_eq!(
        m.to_string(),
        "{\n    name: John,\n    email: john@example.com,\n}"
    );
}

#[test]
fn test_aliased_struct() {
    let a = AliasedStruct {
        name: "John".into(),
        email: "john@example.com".into(),
    };
    assert_eq!(
        a.to_string(),
        "User { name: John, email: john@example.com }"
    );
}

#[test]
fn test_aliased_field() {
    let a = AliasedField {
        name: "John".into(),
        email: "john@example.com".into(),
    };
    assert_eq!(
        a.to_string(),
        "AliasedField { full_name: John, email: john@example.com }"
    );
}

#[test]
fn test_aliased_debug() {
    let a = AliasedDebug {
        name: "John".into(),
        email: "john@example.com".into(),
    };
    assert_eq!(
        a.to_string(),
        "U { n: \"John\", email: \"john@example.com\" }"
    );
}

#[test]
fn test_self_field_access() {
    let s = SelfFieldAccess {
        name: "John".into(),
    };
    assert_eq!(s.to_string(), "John");
}

#[test]
fn test_self_method_call() {
    let s = SelfMethodCall {
        name: "John".into(),
    };
    assert_eq!(s.to_string(), "Hello, John!");
}

#[test]
fn test_field_expr() {
    let f = FieldExpr { count: 5 };
    assert_eq!(f.to_string(), "double: 10");
}
