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
    let v = NamedDefault {
        name: "John".into(),
        email: "john@example.com".into(),
    };
    println!("{v}");
    assert_eq!(
        v.to_string(),
        "NamedDefault { name: John, email: john@example.com }"
    );
}

#[test]
fn test_tuple_default() {
    let v = TupleDefault("hello".into(), "world".into());
    println!("{v}");
    assert_eq!(v.to_string(), "TupleDefault(hello, world)");
}

#[test]
fn test_unit_default() {
    let v = UnitDefault;
    println!("{v}");
    assert_eq!(v.to_string(), "UnitDefault");
}

#[test]
fn test_skip() {
    let v = Ignored {
        name: "John".into(),
        email: "john@example.com".into(),
    };
    println!("{v}");
    assert_eq!(v.to_string(), "Ignored { name: John }");
}

#[test]
fn test_custom_fmt() {
    let v = CustomFmt {
        name: "John".into(),
        email: "john@example.com".into(),
    };
    println!("{v}");
    assert_eq!(
        v.to_string(),
        "hi! my name is John and my email is john@example.com"
    );
}

#[test]
fn test_debug_mode() {
    let v = DebugMode {
        name: "John".into(),
        email: "john@example.com".into(),
    };
    println!("{v}");
    assert_eq!(
        v.to_string(),
        "DebugMode { name: \"John\", email: \"john@example.com\" }"
    );
}

#[test]
fn test_compact_mode() {
    let v = CompactMode {
        name: "John".into(),
        email: "john@example.com".into(),
    };
    println!("{v}");
    assert_eq!(v.to_string(), "John john@example.com");
}

#[test]
fn test_keyvalue_mode() {
    let v = KeyValueMode {
        name: "John".into(),
        email: "john@example.com".into(),
    };
    println!("{v}");
    assert_eq!(v.to_string(), "name=John email=john@example.com");
}

#[test]
fn test_map_mode() {
    let v = MapMode {
        name: "John".into(),
        email: "john@example.com".into(),
    };
    println!("{v}");
    assert_eq!(v.to_string(), "{ name: John, email: john@example.com }");
}

#[test]
fn test_tuple_debug() {
    let v = TupleDebug("hello".into(), 42);
    println!("{v}");
    assert_eq!(v.to_string(), "TupleDebug(\"hello\", 42)");
}

#[test]
fn test_tuple_compact() {
    let v = TupleCompact("hello".into(), 42);
    println!("{v}");
    assert_eq!(v.to_string(), "hello 42");
}

#[test]
fn test_default_pretty() {
    let v = DefaultPretty {
        name: "John".into(),
        email: "john@example.com".into(),
    };
    println!("{v}");
    assert_eq!(
        v.to_string(),
        "DefaultPretty {\n    name: John,\n    email: john@example.com,\n}"
    );
}

#[test]
fn test_tuple_default_pretty() {
    let v = TupleDefaultPretty("hello".into(), 42);
    println!("{v}");
    assert_eq!(v.to_string(), "TupleDefaultPretty(\n    hello,\n    42,\n)");
}

#[test]
fn test_debug_pretty() {
    let v = DebugPretty {
        name: "John".into(),
        email: "john@example.com".into(),
    };
    println!("{v}");
    assert_eq!(
        v.to_string(),
        "DebugPretty {\n    name: \"John\",\n    email: \"john@example.com\",\n}"
    );
}

#[test]
fn test_tuple_debug_pretty() {
    let v = TupleDebugPretty("hello".into(), 42);
    println!("{v}");
    assert_eq!(
        v.to_string(),
        "TupleDebugPretty(\n    \"hello\",\n    42,\n)"
    );
}

#[test]
fn test_keyvalue_pretty() {
    let v = KeyValuePretty {
        name: "John".into(),
        email: "john@example.com".into(),
    };
    println!("{v}");
    assert_eq!(v.to_string(), "name=John\nemail=john@example.com");
}

#[test]
fn test_map_pretty() {
    let v = MapPretty {
        name: "John".into(),
        email: "john@example.com".into(),
    };
    println!("{v}");
    assert_eq!(
        v.to_string(),
        "{\n    name: John,\n    email: john@example.com,\n}"
    );
}

#[test]
fn test_aliased_struct() {
    let v = AliasedStruct {
        name: "John".into(),
        email: "john@example.com".into(),
    };
    println!("{v}");
    assert_eq!(
        v.to_string(),
        "User { name: John, email: john@example.com }"
    );
}

#[test]
fn test_aliased_field() {
    let v = AliasedField {
        name: "John".into(),
        email: "john@example.com".into(),
    };
    println!("{v}");
    assert_eq!(
        v.to_string(),
        "AliasedField { full_name: John, email: john@example.com }"
    );
}

#[test]
fn test_aliased_debug() {
    let v = AliasedDebug {
        name: "John".into(),
        email: "john@example.com".into(),
    };
    println!("{v}");
    assert_eq!(
        v.to_string(),
        "U { n: \"John\", email: \"john@example.com\" }"
    );
}

#[test]
fn test_self_field_access() {
    let v = SelfFieldAccess {
        name: "John".into(),
    };
    println!("{v}");
    assert_eq!(v.to_string(), "John");
}

#[test]
fn test_self_method_call() {
    let v = SelfMethodCall {
        name: "John".into(),
    };
    println!("{v}");
    assert_eq!(v.to_string(), "Hello, John!");
}

#[test]
fn test_field_expr() {
    let v = FieldExpr { count: 5 };
    println!("{v}");
    assert_eq!(v.to_string(), "double: 10");
}

#[cfg(feature = "json")]
mod json {
    use moxy_derive::Display;

    #[derive(Display, serde::Serialize)]
    #[moxy(display(json))]
    pub struct JsonNamed {
        name: String,
        age: i32,
    }

    #[derive(Display, serde::Serialize)]
    #[moxy(display(json, pretty))]
    pub struct JsonPretty {
        name: String,
        age: i32,
    }

    #[derive(Display, serde::Serialize)]
    #[moxy(display(json))]
    pub struct JsonTuple(String, i32);

    #[derive(Display, serde::Serialize)]
    #[moxy(display(json))]
    pub struct JsonSkip {
        name: String,
        #[moxy(display(skip))]
        #[allow(unused)]
        secret: String,
    }

    #[derive(Display, serde::Serialize)]
    #[moxy(display(json, alias = "Person"))]
    pub struct JsonAlias {
        #[moxy(display(alias = "full_name"))]
        name: String,
    }

    #[test]
    fn test_json_named() {
        let v = JsonNamed {
            name: "John".into(),
            age: 30,
        };
        println!("{v}");
        assert_eq!(v.to_string(), r#"{"age":30,"name":"John"}"#);
    }

    #[test]
    fn test_json_pretty() {
        let v = JsonPretty {
            name: "John".into(),
            age: 30,
        };
        println!("{v}");
        assert_eq!(v.to_string(), "{\n  \"age\": 30,\n  \"name\": \"John\"\n}");
    }

    #[test]
    fn test_json_tuple() {
        let v = JsonTuple("hello".into(), 42);
        println!("{v}");
        assert_eq!(v.to_string(), r#"["hello",42]"#);
    }

    #[test]
    fn test_json_skip() {
        let v = JsonSkip {
            name: "John".into(),
            secret: "hidden".into(),
        };
        println!("{v}");
        assert_eq!(v.to_string(), r#"{"name":"John"}"#);
    }

    #[test]
    fn test_json_alias() {
        let v = JsonAlias {
            name: "John".into(),
        };
        println!("{v}");
        assert_eq!(v.to_string(), r#"{"full_name":"John"}"#);
    }
}

#[cfg(feature = "color")]
mod color {
    use colored::Colorize;
    use moxy_derive::Display;

    #[derive(Display)]
    #[moxy(display(color))]
    pub struct ColorNamed {
        name: String,
        email: String,
    }

    #[derive(Display)]
    #[moxy(display(color, pretty))]
    pub struct ColorPretty {
        name: String,
        email: String,
    }

    #[derive(Display)]
    #[moxy(display(color))]
    pub struct ColorTuple(String, i32);

    #[test]
    fn test_color_named() {
        let v = ColorNamed {
            name: "John".into(),
            email: "john@example.com".into(),
        };
        println!("{v}");
        let expected = format!(
            "{} {{ {}: {}, {}: {} }}",
            "ColorNamed".cyan().bold(),
            "name".blue(),
            "John",
            "email".blue(),
            "john@example.com",
        );
        assert_eq!(v.to_string(), expected);
    }

    #[test]
    fn test_color_pretty() {
        let v = ColorPretty {
            name: "John".into(),
            email: "john@example.com".into(),
        };
        println!("{v}");
        let expected = format!(
            "{} {{\n    {}: {},\n    {}: {},\n}}",
            "ColorPretty".cyan().bold(),
            "name".blue(),
            "John",
            "email".blue(),
            "john@example.com",
        );
        assert_eq!(v.to_string(), expected);
    }

    #[test]
    fn test_color_tuple() {
        let v = ColorTuple("hello".into(), 42);
        println!("{v}");
        let expected = format!("{}({}, {})", "ColorTuple".cyan().bold(), "hello", 42,);
        assert_eq!(v.to_string(), expected);
    }

    #[derive(Display)]
    #[moxy(display(debug, color))]
    pub struct DebugColor {
        name: String,
        email: String,
    }

    #[derive(Display)]
    #[moxy(display(map, color))]
    pub struct MapColor {
        name: String,
        email: String,
    }

    #[derive(Display)]
    #[moxy(display(keyvalue, color))]
    pub struct KeyValueColor {
        name: String,
        email: String,
    }

    #[test]
    fn test_debug_color() {
        let v = DebugColor {
            name: "John".into(),
            email: "john@example.com".into(),
        };
        println!("{v}");
        let expected = format!(
            "{} {{ {}: {:?}, {}: {:?} }}",
            "DebugColor".cyan().bold(),
            "name".blue(),
            "John",
            "email".blue(),
            "john@example.com",
        );
        assert_eq!(v.to_string(), expected);
    }

    #[test]
    fn test_map_color() {
        let v = MapColor {
            name: "John".into(),
            email: "john@example.com".into(),
        };
        println!("{v}");
        let expected = format!(
            "{{ {}: {}, {}: {} }}",
            "name".blue(),
            "John",
            "email".blue(),
            "john@example.com",
        );
        assert_eq!(v.to_string(), expected);
    }

    #[test]
    fn test_keyvalue_color() {
        let v = KeyValueColor {
            name: "John".into(),
            email: "john@example.com".into(),
        };
        println!("{v}");
        let expected = format!(
            "{}={} {}={}",
            "name".blue(),
            "John",
            "email".blue(),
            "john@example.com",
        );
        assert_eq!(v.to_string(), expected);
    }
}
