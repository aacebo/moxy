use moxy::ast::{Expr, Item};
use moxy::token::Spanner;

#[test]
fn macro_invocations_preserve_paths_delimiters_and_body_tokens() {
    for (source, expected, item_position) in [
        ("vec![1, 2, 3]", "vec![1 , 2 , 3]", false),
        ("module::call!(name, value);", "module::call!(name , value);", true),
    ] {
        if item_position {
            let item: Item = moxy::parse!(source).unwrap();
            assert!(item.is_macro());
            assert!(!item.span().is_empty());
            assert_eq!(moxy::fmt!(&item).unwrap(), expected);
        } else {
            let expression: Expr = moxy::parse!(source).unwrap();
            assert!(expression.is_macro());
            assert!(!expression.span().is_empty());
            assert_eq!(moxy::fmt!(&expression).unwrap(), expected);
        }
    }
}

#[test]
fn templates_generate_a_real_struct_syntax_pipeline() {
    let name = "Generated";
    let fields = ["first", "second"];
    let tokens = moxy::template! {
        pub struct {{ name }} {
            @for (field in fields) { {{ field }}: String, }
        }
    };
    let item: Item = moxy::parse!(tokens).unwrap();
    let structure = item.as_struct().unwrap();
    assert_eq!(structure.ident.text(), "Generated");
    assert_eq!(structure.fields.as_named().unwrap().fields.inner.len(), 2);
    assert!(!structure.span().is_empty());
    assert_eq!(
        moxy::fmt!(&item).unwrap(),
        "pub struct Generated {\n\tfirst: String,\n\tsecond: String,\n}"
    );
}

#[test]
fn template_conditionals_generate_real_struct_syntax() {
    for (enabled, expected) in [
        (true, "struct State {\n\tenabled: bool,\n}"),
        (false, "struct State {\n\tdisabled: bool,\n}"),
    ] {
        std::hint::black_box(enabled);
        let tokens = moxy::template! {
            struct State {
                @if (enabled) { enabled: bool, } @else { disabled: bool, }
            }
        };
        let rendered_tokens = tokens.to_string();
        let item: Item = moxy::parse!(tokens).unwrap_or_else(|error| panic!("failed to parse {rendered_tokens}: {error}"));
        let structure = item.as_struct().unwrap();
        assert_eq!(structure.ident.text(), "State");
        assert_eq!(structure.fields.as_named().unwrap().fields.inner.len(), 1);
        assert_eq!(moxy::fmt!(&item).unwrap(), expected);
    }
}

#[test]
fn template_matches_generate_real_constant_syntax() {
    for (value, expected) in [(Some("1"), "const VALUE: usize = 1;"), (None, "const VALUE: usize = 0;")] {
        let tokens = moxy::template! {
            @match (value) {
                Some(value) => { const VALUE: usize = {{ value }}; },
                None => { const VALUE: usize = 0; },
            }
        };
        let item: Item = moxy::parse!(tokens).unwrap();
        let constant = item.as_const().unwrap();
        assert_eq!(constant.ident.text(), "VALUE");
        assert!(!constant.span().is_empty());
        assert_eq!(moxy::fmt!(&item).unwrap(), expected);
    }
}

#[test]
fn template_directives_support_rust_patterns_and_optional_headers() {
    let value = Some((1, "matched"));
    let fields = [("first", "String"), ("second", "usize")];
    let enabled = true;
    let tokens = moxy::template! {
        @if let Some((1, text)) = value {
            const IF_VALUE: &str = {{ text }};
        } @else {
            const IF_VALUE: &str = "missing";
        }

        @if enabled {
            const ENABLED: bool = true;
        }

        struct Fields {
            @for ((name, ty) in &fields) { {{ name }}: {{ ty }}, }
        }

        @for value in ["one", "two"].into_iter().map(|value| value.to_string()) {
            const _: &str = {{ value }};
        }

        @match (value) {
            Some((1, text)) if text == "matched" => { const MATCH_VALUE: &str = {{ text }}; },
            None | Some(_) => { const MATCH_VALUE: &str = "other"; },
        }
    };
    let rendered = tokens.to_string();
    assert!(rendered.contains("IF_VALUE"));
    assert!(rendered.contains("struct Fields"));
    assert!(rendered.contains("MATCH_VALUE"));
}

#[test]
#[allow(clippy::let_and_return)]
fn template_interpolation_blocks_return_the_tail_value() {
    let values = ["first", "second"];
    let tokens = moxy::template! {
        @for value in values {
            {{
                let rendered = value.to_uppercase();
                rendered
            }}
        }
    };

    assert_eq!(tokens.to_string(), "\"FIRST\" \"SECOND\"");
}

moxy::paste! {
    struct {{ Pasted Record }} {
        {{ field_ value }}: u32,
    }
}

#[test]
fn pasted_identifiers_create_parseable_struct_syntax() {
    let value = PastedRecord { field_value: 7 };
    assert_eq!(value.field_value, 7);
    let item: Item = moxy::parse!("struct PastedRecord { field_value: u32 }").unwrap();
    let structure = item.as_struct().unwrap();
    assert_eq!(structure.ident.text(), "PastedRecord");
    assert_eq!(
        structure.fields.as_named().unwrap().fields.inner[0]
            .ident
            .as_ref()
            .unwrap()
            .text(),
        "field_value"
    );
    assert_eq!(moxy::fmt!(&item).unwrap(), "struct PastedRecord {\n\tfield_value: u32,\n}");
}

#[test]
fn macro_rules_items_render_exact_valid_syntax() {
    let item: Item = moxy::parse!("macro_rules! generated { ($tokens:tt) => { $tokens }; }").unwrap();
    assert!(item.is_macro2());
    assert!(!item.span().is_empty());
    assert_eq!(
        moxy::fmt!(&item).unwrap(),
        "macro_rules! generated { ($ tokens : tt) => {$ tokens} ; }"
    );
}

#[test]
fn template_and_paste_compiler_contracts_are_stable() {
    let cases = trybuild::TestCases::new();
    cases.pass("fixtures/trybuild/template/pass/*.rs");
    cases.compile_fail("fixtures/trybuild/template/fail/invalid_paste.rs");

    let files = moxy::parse_files!("fixtures/trybuild/template/pass/*.rs");
    assert_eq!(files.len(), 2);
    assert!(files.iter().all(|file| !file.items.is_empty()));
    assert_eq!(files.iter().map(|file| file.items.len()).sum::<usize>(), 5);
}

#[test]
fn malformed_template_compiler_contract_is_rejected() {
    let cases = trybuild::TestCases::new();
    cases.compile_fail("fixtures/trybuild/template/fail/malformed_template.rs");
}
