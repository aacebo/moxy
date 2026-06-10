use moxy_template::template;

mod tokens {
    use super::template;

    #[test]
    fn plain_ident() {
        let tokens = template! { hello };
        assert_eq!(tokens.to_string(), "hello");
    }

    #[test]
    fn empty_template() {
        let tokens = template! {};
        assert!(tokens.is_empty());
    }

    #[test]
    fn multiple_tokens() {
        let tokens = template! { let x = 1 };
        assert_eq!(tokens.to_string(), "let x = 1");
    }
}

mod interp {
    use super::template;

    #[test]
    fn single_interp() {
        let name = "world";
        let tokens = template! { {{ name }} };
        assert_eq!(tokens.to_string(), "world");
    }

    #[test]
    fn interp_in_text() {
        let name = "moxy";
        let tokens = template! { Hello {{ name }} ! };
        assert_eq!(tokens.to_string(), "Hello moxy !");
    }

    #[test]
    fn interp_expr() {
        let pair = ("a", "b");
        let tokens = template! { {{ pair.0 }} {{ pair.1 }} };
        assert_eq!(tokens.to_string(), "a b");
    }

    #[test]
    fn interp_fuses_trailing_ident() {
        let name = "user";
        let tokens = template! { {{ name }}_test };
        assert_eq!(tokens.to_string(), "user_test");
    }

    #[test]
    fn interp_does_not_fuse_leading_ident() {
        let name = "world";
        let tokens = template! { Hello {{ name }} };
        assert_eq!(tokens.to_string(), "Hello world");
    }

    #[test]
    fn interp_does_not_fuse_trailing_punct() {
        let name = "moxy";
        let tokens = template! { {{ name }}() };
        assert_eq!(tokens.to_string(), "moxy ()");
    }

    #[test]
    fn interp_fuse_keeps_trailing_tokens() {
        let name = "user";
        let tokens = template! { fn {{ name }}_test() };
        assert_eq!(tokens.to_string(), "fn user_test ()");
    }

    #[test]
    fn interp_fuse_preserves_span() {
        use std::str::FromStr;

        use moxy_token::{Ident, TokenStream};

        let seg = TokenStream::from_str("user").unwrap().parse().parse::<Ident>().unwrap();
        let tokens = template! { {{ seg }}_test };
        let fused = tokens.iter().next().unwrap();
        assert_eq!(fused.span(), seg.span());
    }

    #[test]
    fn interp_inside_group() {
        let name = "user";
        let tokens = template! { impl Foo { fn {{ name }}() {} } };
        assert_eq!(tokens.to_string(), "impl Foo {fn user () {}}");
    }

    #[test]
    fn interp_fuse_inside_nested_groups() {
        let name = "user";
        let tokens = template! { impl Foo { fn {{ name }}_test(&self) -> bool { true } } };
        assert_eq!(tokens.to_string(), "impl Foo {fn user_test (& self) -> bool {true}}");
    }

    #[test]
    fn group_without_interp_unchanged() {
        let tokens = template! { fn f() { let x = (1 + 2); } };
        assert_eq!(tokens.to_string(), "fn f () {let x = (1 + 2) ;}");
    }
}

mod keywords {
    use super::template;

    #[test]
    fn if_basic() {
        let cond = true;
        let tokens = template! { @if (cond) { yes } };
        assert_eq!(tokens.to_string(), "yes");
    }

    #[test]
    fn if_else_true() {
        let tokens = template! { @if (true) { b } @else { c } };
        assert_eq!(tokens.to_string(), "b");
    }

    #[test]
    fn if_else_false() {
        let tokens = template! { @if (false) { b } @else { c } };
        assert_eq!(tokens.to_string(), "c");
    }

    #[test]
    fn if_else_if() {
        let tokens = template! { @if (false) { b } @else if (true) { d } @else { e } };
        assert_eq!(tokens.to_string(), "d");
    }

    #[test]
    fn for_basic() {
        let items = vec!["a", "b", "c"];
        let tokens = template! { @for (item in items) { {{ item }} } };
        assert_eq!(tokens.to_string(), "a b c");
    }

    #[test]
    fn match_basic() {
        let x = 2;
        let tokens = template! { @match (x) { 1 => { a }, 2 => { b }, _ => { c }, } };
        assert_eq!(tokens.to_string(), "b");
    }
}
