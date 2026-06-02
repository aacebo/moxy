pub mod ast;
mod template;

pub use moxy_token;
pub use template::*;

/// Lex and parse a template source string into a [`Template`] AST.
///
/// Accepts either a string literal or a runtime `&str` expression.
/// Returns `Result<Template, ParseError>` — `Ok` when the input is valid,
/// `Err` with a span-annotated [`moxy_token::parse::ParseError`] otherwise.
///
/// # Examples
///
/// ```
/// let tmpl = moxy_template::compile!("Hello {{ name }}!").unwrap();
/// assert_eq!(tmpl.nodes.len(), 3);
/// ```
///
/// ```
/// let src = String::from("{{ value }}");
/// let result = moxy_template::compile!(src);
/// assert!(result.is_ok());
/// ```
#[macro_export]
macro_rules! compile {
    ($src:literal) => {{
        use ::std::str::FromStr;
        $crate::moxy_token::TokenStream::from_str($src)
            .map_err($crate::moxy_token::parse::ParseError::from)
            .and_then(|ts| <$crate::Template as $crate::moxy_token::Parse>::parse(&mut ts.parse()))
    }};
    ($src:expr) => {{
        use ::std::str::FromStr;
        $crate::moxy_token::TokenStream::from_str(::std::convert::AsRef::<str>::as_ref(&$src))
            .map_err($crate::moxy_token::parse::ParseError::from)
            .and_then(|ts| <$crate::Template as $crate::moxy_token::Parse>::parse(&mut ts.parse()))
    }};
}

/// Lex, parse, and emit a template source string as a [`moxy_token::TokenStream`].
///
/// Accepts either a string literal or a runtime `&str` expression.
/// On success emits the template's token representation; on parse failure emits
/// a `compile_error!` token stream carrying the error message and span.
///
/// # Examples
///
/// ```
/// let ts = moxy_template::template!("Hello {{ name }}!");
/// assert!(!ts.is_empty());
/// ```
///
/// ```
/// // Invalid input — the returned stream is a compile_error! invocation
/// let ts = moxy_template::template!("@bad");
/// assert!(ts.to_string().contains("compile_error"));
/// ```
#[macro_export]
macro_rules! template {
    ($src:literal) => {{
        use $crate::moxy_token::ToTokenStream;
        match $crate::compile!($src) {
            ::std::result::Result::Ok(tmpl) => tmpl.into_token_stream(),
            ::std::result::Result::Err(e) => e.into_token_stream(),
        }
    }};
    ($src:expr) => {{
        use $crate::moxy_token::ToTokenStream;
        match $crate::compile!($src) {
            ::std::result::Result::Ok(tmpl) => tmpl.into_token_stream(),
            ::std::result::Result::Err(e) => e.into_token_stream(),
        }
    }};
}

#[cfg(test)]
mod tests {
    mod tokens {
        use crate::ast::Node;

        #[test]
        fn plain_ident() {
            let tmpl = crate::compile!("hello").unwrap();
            assert_eq!(tmpl.nodes.len(), 1);
            assert!(matches!(tmpl.nodes[0], Node::Tokens(_)));
            assert_eq!(tmpl.render().to_string(), "hello");
        }

        #[test]
        fn empty_template() {
            let tmpl = crate::compile!("").unwrap();
            assert!(tmpl.nodes.is_empty());
        }

        #[test]
        fn multiple_tokens() {
            let tmpl = crate::compile!("let x = 1").unwrap();
            assert_eq!(tmpl.nodes.len(), 1);
            assert!(matches!(tmpl.nodes[0], Node::Tokens(_)));
            assert_eq!(tmpl.render().to_string(), "let x = 1");
        }
    }

    mod interp {
        use crate::ast::Node;

        #[test]
        fn single_interp() {
            let tmpl = crate::compile!("{{ name }}").unwrap();
            assert_eq!(tmpl.nodes.len(), 1);
            let Node::Interp(ref interp) = tmpl.nodes[0] else {
                panic!("expected Interp")
            };
            assert!(!interp.expr.is_empty());
        }

        #[test]
        fn interp_in_text() {
            let tmpl = crate::compile!("Hello {{ name }}!").unwrap();
            assert_eq!(tmpl.nodes.len(), 3);
            assert!(matches!(tmpl.nodes[0], Node::Tokens(_)));
            assert!(matches!(tmpl.nodes[1], Node::Interp(_)));
            assert!(matches!(tmpl.nodes[2], Node::Tokens(_)));
        }

        #[test]
        fn interp_expr() {
            let tmpl = crate::compile!("{{ a + b }}").unwrap();
            let Node::Interp(ref interp) = tmpl.nodes[0] else {
                panic!("expected Interp")
            };
            assert_eq!(interp.expr.to_string(), "a + b");
        }
    }

    mod keywords {
        use crate::ast::{Node, TmplKeyword};

        #[test]
        fn if_basic() {
            let tmpl = crate::compile!("@if (cond) { yes }").unwrap();
            assert_eq!(tmpl.nodes.len(), 1);
            let Node::Keyword(TmplKeyword::If(ref if_node)) = tmpl.nodes[0] else {
                panic!("expected If keyword")
            };
            assert_eq!(if_node.branches.len(), 1);
            assert!(if_node.else_body.is_none());
        }

        #[test]
        fn if_else() {
            let tmpl = crate::compile!("@if (a) { b } @else { c }").unwrap();
            let Node::Keyword(TmplKeyword::If(ref if_node)) = tmpl.nodes[0] else {
                panic!("expected If keyword")
            };
            assert!(if_node.else_body.is_some());
        }

        #[test]
        fn if_else_if() {
            let tmpl = crate::compile!("@if (a) { b } @else if (c) { d } @else { e }").unwrap();
            let Node::Keyword(TmplKeyword::If(ref if_node)) = tmpl.nodes[0] else {
                panic!("expected If keyword")
            };
            assert_eq!(if_node.branches.len(), 2);
            assert!(if_node.else_body.is_some());
        }

        #[test]
        fn for_basic() {
            let tmpl = crate::compile!("@for (item in items) { {{ item }} }").unwrap();
            assert_eq!(tmpl.nodes.len(), 1);
            let Node::Keyword(TmplKeyword::For(ref for_node)) = tmpl.nodes[0] else {
                panic!("expected For keyword")
            };
            assert_eq!(for_node.binding.to_string(), "item");
        }

        #[test]
        fn match_basic() {
            let tmpl = crate::compile!("@match (x) { A => { a }, B => { b }, }").unwrap();
            assert_eq!(tmpl.nodes.len(), 1);
            let Node::Keyword(TmplKeyword::Match(ref match_node)) = tmpl.nodes[0] else {
                panic!("expected Match keyword")
            };
            assert_eq!(match_node.arms.len(), 2);
        }
    }

    mod macros {
        #[test]
        fn compile_ok() {
            let src = "hello {{ world }}";
            assert!(crate::compile!(src).is_ok());
        }

        #[test]
        fn template_ok() {
            let src = "hello {{ world }}";
            assert!(!crate::template!(src).is_empty());
        }

        #[test]
        fn compile_literal_arm() {
            assert!(crate::compile!("hello").is_ok());
        }

        #[test]
        fn template_literal_arm() {
            assert!(!crate::template!("hello").is_empty());
        }

        #[test]
        fn template_error_emits_compile_error() {
            // An unclosed group fails the lexer, which is the only way to get a real
            let src = "{{ unclosed";
            assert!(crate::template!(src).to_string().contains("compile_error"));
        }
    }
}
