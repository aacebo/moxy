use moxy::ast::{Expr, Item};
use moxy::token::{Spanner, ToTokenStream, TokenStream};

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
    let name = moxy::ident!("Generated");
    let fields = [moxy::ident!("first"), moxy::ident!("second")];
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
    for (value, expected) in [
        (Some("1".parse::<TokenStream>().unwrap()), "const VALUE: usize = 1;"),
        (None, "const VALUE: usize = 0;"),
    ] {
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
    let fields = [
        (moxy::ident!("first"), moxy::ident!("String")),
        (moxy::ident!("second"), moxy::ident!("usize")),
    ];
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
fn template_interpolates_string_slices_as_literals() {
    let name: &str = "hello world";
    let apostrophe: &str = "it's done";
    let unlexable: &str = "say \"hi";
    let owned = String::from("hello world");

    let tokens = moxy::template! {
        {{ name }} {{ apostrophe }} {{ unlexable }} {{ owned }}
    };

    assert_eq!(tokens.to_string(), format!("{name:?} {apostrophe:?} {unlexable:?} {owned:?}"));
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

#[test]
fn template_for_loops_emit_a_separator_between_iterations() {
    let args = [moxy::ident!("first"), moxy::ident!("second"), moxy::ident!("third")];
    let empty: [moxy::token::Ident; 0] = [];
    let rows = [vec!["a", "b"], vec![], vec!["c"]];
    let tokens = moxy::template! {
        call(@for arg in &args { {{ arg }} } @sep { , });
        none(@for arg in &empty { {{ arg }} } @sep { , });
        nested(@for row in &rows { [@for value in row { {{ *value }} } @sep { + }] } @sep { ; });
    };

    assert_eq!(
        tokens.to_string(),
        "call (first , second , third) ; none () ; nested ([\"a\" + \"b\"] ; [] ; [\"c\"]) ;"
    );
}

#[test]
fn template_separators_interpolate_and_skip_empty_iterations() {
    let joiner = moxy::ident!("then");
    let values = [None, Some("a"), None, None, Some("c"), None];
    let flags = [true, false, true];
    let numbers = [(1, "one"), (2, "two"), (3, "three"), (4, "four")];
    let tokens = moxy::template! {
        chain(@for value in &values { @if let Some(value) = value { {{ *value }} } } @sep { {{ joiner }} });
        flags(@for f in &flags { @if *f { x } } @sep { , });
        odd(@for (n, name) in &numbers {
            @match (n % 2) {
                0 => {},
                _ => { {{ *name }} },
            }
        } @sep { , });
        none(@for f in &flags { @if !*f { x } @else {} } @sep { , });
    };

    assert_eq!(
        tokens.to_string(),
        "chain (\"a\" then \"c\") ; flags (x , x) ; odd (\"one\" , \"three\") ; none (x) ;"
    );
}

#[test]
fn template_separator_state_is_hidden_from_user_bindings() {
    let emitted = ["a", "b"];
    let iteration = [false, true];
    let __moxy_first = ["c", "d"];
    let tokens = moxy::template! {
        a(@for iteration in &emitted { {{ *iteration }} } @sep { , });
        b(@for emitted in &iteration { @if *emitted { y } } @sep { , });
        c(@for __moxy_first in &__moxy_first { {{ *__moxy_first }} } @sep { , });
    };

    assert_eq!(tokens.to_string(), "a (\"a\" , \"b\") ; b (y) ; c (\"c\" , \"d\") ;");
}

#[test]
fn malformed_template_separator_is_rejected() {
    let cases = trybuild::TestCases::new();
    cases.compile_fail("fixtures/trybuild/template/fail/malformed_separator.rs");
}

macro_rules! scaled {
    ($e:expr) => {
        moxy::template! { let x = $e * 2; }
    };
}

macro_rules! referenced {
    ($t:ty) => {
        moxy::template! { let x: &$t = y; }
    };
}

macro_rules! crate_call {
    () => {
        moxy::template! { $crate::foo() }
    };
}

#[test]
fn macro_rules_fragments_keep_their_invisible_groups() {
    let tokens = scaled!(1 + 2);
    let statement: moxy::ast::Stmt = moxy::parse!(tokens).unwrap();
    let local = statement.as_local().unwrap();
    let product = local.init.as_ref().unwrap().expr.as_binary_chain().unwrap();
    assert!(product.operands[0].is_group());
    assert_eq!(product.operands[0].to_token_stream().to_string(), "1 + 2");

    let tokens = referenced!(dyn A + B);
    let groups = |tokens: &TokenStream| {
        tokens
            .iter()
            .filter(|token| matches!(token, moxy::token::TokenTree::Group(group) if group.delim == moxy::token::Delim::None))
            .count()
    };
    assert_eq!(groups(&tokens), 1);

    let tokens = crate_call!();
    let first = tokens.iter().next().unwrap();
    assert!(matches!(first, moxy::token::TokenTree::Ident(ident) if ident.text() == "$crate"));
}

#[test]
fn template_literals_the_compiler_accepts_build_at_run_time() {
    let tokens = moxy::template! { 1.0f16 1f128 10px 340282366920938463463374607431768211456 '\u{00_41}' };
    assert_eq!(
        tokens.to_string(),
        "1.0f16 1f128 10px 340282366920938463463374607431768211456 '\\u{00_41}'"
    );
}

#[test]
fn template_conditions_may_start_with_parentheses_and_patterns_may_bind_with_at() {
    let (a, b, c) = (false, true, true);
    let tokens = moxy::template! { @if (a || b) && c { yes } @else { no } };
    assert_eq!(tokens.to_string(), "yes");

    let value = moxy::ident!("x");
    let tokens = moxy::template! { match {{ value }} { n @ 1..=9 => n, _ => 0 } };
    assert_eq!(tokens.to_string(), "match x {n @ 1 ..= 9 => n , _ => 0}");
}

#[test]
fn template_directives_follow_any_token_and_bindings_stay_tokens() {
    let a = true;
    let rows = vec![moxy::ident!("p"), moxy::ident!("q")];
    let tokens = moxy::template! { x @if a { yes } };
    assert_eq!(tokens.to_string(), "x yes");

    let tokens = moxy::template! { f @for r in &rows { {{ r }} } };
    assert_eq!(tokens.to_string(), "f p q");

    let tokens = moxy::template! { let x = y @match (a) { true => { t }, false => { f } } };
    assert_eq!(tokens.to_string(), "let x = y t");

    let tokens = moxy::template! { struct S @if a { <T> } @else { <U> }; };
    assert_eq!(tokens.to_string(), "struct S < T > ;");

    let tokens = moxy::template! { f() @if a { yes } };
    assert_eq!(tokens.to_string(), "f () yes");

    let tokens = moxy::template! { Vec<T> @if !a { no } @else { yes } };
    assert_eq!(tokens.to_string(), "Vec < T > yes");

    // An interpolation makes the arms a template, which reads every `@`
    let one = moxy::ident!("one");
    let tokens = moxy::template! { match v { x @ sep => {{ one }} } };
    assert_eq!(tokens.to_string(), "match v {x @ sep => one}");

    let tokens = moxy::template! { @for r in &rows { {{ r }} } @sep { , } };
    assert_eq!(tokens.to_string(), "p , q");

    let tokens = moxy::template! { match v { n @ 1..=9 => n, all @ Some(_) => all, w @ _ => w } };
    assert_eq!(
        tokens.to_string(),
        "match v {n @ 1 ..= 9 => n , all @ Some (_) => all , w @ _ => w}"
    );
}

#[test]
fn template_literal_tokens_are_lexed_once_per_thread() {
    use moxy::token::source::SourceMap;

    let expand = || moxy::template! { pub struct Widget { a: u8 } };
    let first = expand();
    let sources = SourceMap::with(|map| map.len());
    let second = expand();

    assert_eq!(SourceMap::with(|map| map.len()), sources);
    assert_eq!(first.to_string(), second.to_string());
    assert_eq!(first.to_string(), "pub struct Widget {a : u8}");
}
