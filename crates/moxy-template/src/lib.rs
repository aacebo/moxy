pub mod ast;
mod template;

pub use moxy_token;
pub use template::*;

/// Parse raw token trees into a [`Template`] AST.
///
/// Accepts any sequence of raw tokens — no quotes needed. The tokens are
/// stringified and lexed into a [`moxy_token::TokenStream`], then parsed as
/// a `Template`. Returns `Result<Template, ParseError>`.
///
/// # Examples
///
/// ```
/// let tmpl = moxy_template::compile!(hello {{ name }} !).unwrap();
/// assert_eq!(tmpl.nodes.len(), 3);
/// ```
///
/// ```
/// let tmpl = moxy_template::compile!(@if (cond) { yes }).unwrap();
/// assert!(tmpl.nodes.len() == 1);
/// ```
#[macro_export]
macro_rules! compile {
    ($($tt:tt)*) => {{
        use ::std::str::FromStr;
        $crate::moxy_token::TokenStream::from_str(stringify!($($tt)*))
            .map_err($crate::moxy_token::parse::ParseError::from)
            .and_then(|ts| <$crate::Template as $crate::moxy_token::Parse>::parse(&mut ts.parse()))
    }};
}

/// Parse raw token trees and emit the result as a [`moxy_token::TokenStream`].
///
/// Accepts any sequence of raw tokens. On parse failure emits a `compile_error!`
/// token stream.
///
/// # Examples
///
/// ```
/// let ts = moxy_template::template!(hello {{ name }} !);
/// assert!(!ts.is_empty());
/// ```
#[macro_export]
macro_rules! template {
    ($($tt:tt)*) => {{
        use $crate::moxy_token::ToTokenStream;
        match $crate::compile!($($tt)*) {
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
            let tmpl = crate::compile!(hello).unwrap();
            assert_eq!(tmpl.nodes.len(), 1);
            assert!(matches!(tmpl.nodes[0], Node::Tokens(_)));
            assert_eq!(tmpl.render().to_string(), "hello");
        }

        #[test]
        fn empty_template() {
            let tmpl = crate::compile!().unwrap();
            assert!(tmpl.nodes.is_empty());
        }

        #[test]
        fn multiple_tokens() {
            let tmpl = crate::compile!(let x = 1).unwrap();
            assert_eq!(tmpl.nodes.len(), 1);
            assert!(matches!(tmpl.nodes[0], Node::Tokens(_)));
        }
    }

    mod interp {
        use crate::ast::Node;

        #[test]
        fn single_interp() {
            let tmpl = crate::compile!({ { name } }).unwrap();
            assert_eq!(tmpl.nodes.len(), 1);
            let Node::Interp(ref interp) = tmpl.nodes[0] else {
                panic!("expected Interp")
            };
            assert!(!interp.expr.is_empty());
        }

        #[test]
        fn interp_in_text() {
            let tmpl = crate::compile!(Hello {{ name }} !).unwrap();
            assert_eq!(tmpl.nodes.len(), 3);
            assert!(matches!(tmpl.nodes[0], Node::Tokens(_)));
            assert!(matches!(tmpl.nodes[1], Node::Interp(_)));
            assert!(matches!(tmpl.nodes[2], Node::Tokens(_)));
        }

        #[test]
        fn interp_expr() {
            let tmpl = crate::compile!({ { a + b } }).unwrap();
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
            let tmpl = crate::compile!(@if (cond) { yes }).unwrap();
            assert_eq!(tmpl.nodes.len(), 1);
            let Node::Keyword(TmplKeyword::If(ref n)) = tmpl.nodes[0] else {
                panic!("expected If keyword")
            };
            assert_eq!(n.branches.len(), 1);
            assert!(n.else_body.is_none());
        }

        #[test]
        fn if_else() {
            let tmpl = crate::compile!(@if (a) { b } @else { c }).unwrap();
            let Node::Keyword(TmplKeyword::If(ref n)) = tmpl.nodes[0] else {
                panic!("expected If keyword")
            };
            assert!(n.else_body.is_some());
        }

        #[test]
        fn if_else_if() {
            let tmpl = crate::compile!(@if (a) { b } @else if (c) { d } @else { e }).unwrap();
            let Node::Keyword(TmplKeyword::If(ref n)) = tmpl.nodes[0] else {
                panic!("expected If keyword")
            };
            assert_eq!(n.branches.len(), 2);
            assert!(n.else_body.is_some());
        }

        #[test]
        fn for_basic() {
            let tmpl = crate::compile!(@for (item in items) { {{ item }} }).unwrap();
            let Node::Keyword(TmplKeyword::For(ref n)) = tmpl.nodes[0] else {
                panic!("expected For keyword")
            };
            assert_eq!(n.binding.to_string(), "item");
        }

        #[test]
        fn match_basic() {
            let tmpl = crate::compile!(@match (x) { A => { a }, B => { b }, }).unwrap();
            let Node::Keyword(TmplKeyword::Match(ref n)) = tmpl.nodes[0] else {
                panic!("expected Match keyword")
            };
            assert_eq!(n.arms.len(), 2);
        }
    }

    mod macros {
        #[test]
        fn template_passthrough() {
            let ts = crate::template!(hello world);
            assert_eq!(ts.to_string(), "hello world");
        }

        #[test]
        fn template_interp_roundtrip() {
            let ts = crate::template!({ { name } });
            assert!(ts.to_string().contains("name"));
        }

        #[test]
        fn template_empty() {
            assert!(crate::template!().is_empty());
        }

        #[test]
        fn template_if_roundtrip() {
            let ts = crate::template!(@if (cond) { yes });
            let s = ts.to_string();
            assert!(s.contains("if"));
            assert!(s.contains("cond"));
            assert!(s.contains("yes"));
        }
    }
}
