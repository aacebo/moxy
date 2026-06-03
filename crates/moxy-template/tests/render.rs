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
