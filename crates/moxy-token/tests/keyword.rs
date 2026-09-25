use moxy_token::{Keyword, TokenStream};

#[test]
fn accept() {
    macro_rules! scan {
        ($($check:ident => $source:literal),+ $(,)?) => {
            $(
                let tokens: TokenStream = $source.parse().expect(concat!("keyword ", $source, " could not be scanned"));
                let mut iter = tokens.into_iter();
                let token = iter.next().expect(concat!("no tokens scanned from ", $source));
                let token = token.as_keyword().expect("expected keyword");
                assert!(matches!(token, Keyword::$check(_)));
            )+
        };
    }

    scan! {
        As => "as", Async => "async", Auto => "auto", Await => "await",
        Become => "become", Box => "box", Break => "break", Const => "const",
        Continue => "continue", Crate => "crate", Default => "default", Do => "do",
        Dyn => "dyn", Else => "else", Enum => "enum", Extern => "extern",
        Final => "final", Fn => "fn", For => "for", If => "if", Impl => "impl",
        In => "in", Let => "let", Loop => "loop", Macro => "macro",
        MacroRules => "macro_rules", Match => "match", Mod => "mod", Move => "move",
        Mut => "mut", Override => "override", Priv => "priv", Pub => "pub", Raw => "raw",
        Ref => "ref", Return => "return", SelfType => "Self", SelfValue => "self",
        Static => "static", Struct => "struct", Super => "super", Trait => "trait",
        Try => "try", Type => "type", Typeof => "typeof", Union => "union",
        Unsafe => "unsafe", Unsized => "unsized", Use => "use", Virtual => "virtual",
        Where => "where", While => "while", Yield => "yield",
    }
}

#[test]
fn reject() {
    let cases = ["1", "1.5", ",", "|"];

    for case in cases {
        let tokens: TokenStream = case.parse().expect("invalid");
        let mut iter = tokens.into_iter();
        let token = iter.next().expect("no tokens scanned");
        assert!(!token.is_keyword());
    }
}
