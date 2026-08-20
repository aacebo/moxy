use moxy_template::{paste, template};
use moxy_token::TokenStream;

paste! {
    struct {{ Generated Record }} {
        {{ field_ value }}: i32,
    }
}

#[test]
fn templates_generate_lexically_parseable_items_across_control_flow() {
    let name = "Generated";
    let include_extra = true;
    let fields = ["first", "second"];
    let tokens = template! {
        pub struct {{ name }} {
            @for (field in fields) { {{ field }}: String, }
            @if (include_extra) { extra: bool, } @else { fallback: bool, }
        }
    };
    let rendered = tokens.to_string();
    assert_eq!(
        rendered,
        "pub struct Generated {first : String , second : String , extra : bool ,}"
    );
    assert_eq!(rendered.parse::<TokenStream>().unwrap().to_string(), rendered);
}

#[test]
fn template_match_and_nested_interpolation_generate_parseable_token_streams() {
    let selected = Some("2");
    let expression = template! {
        @match (selected) {
            Some(value) => { Some({{ value }} + 1) },
            None => { None },
        }
    };
    assert_eq!(expression.to_string(), "Some (2 + 1)");
    assert!(expression.to_string().parse::<TokenStream>().is_ok());
}

#[test]
fn pasted_identifiers_participate_in_generated_rust_workflows() {
    let value = GeneratedRecord { field_value: 7 };
    assert_eq!(value.field_value, 7);
    let source: TokenStream = "struct GeneratedRecord { field_value: i32 }".parse().unwrap();
    assert_eq!(source.to_string(), "struct GeneratedRecord {field_value : i32}");
}
