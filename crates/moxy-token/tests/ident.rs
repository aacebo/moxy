use moxy_token::ident;

#[test]
fn from_ident() {
    let ident = ident!(abc);
    assert_eq!(ident, "abc");
}

#[test]
fn from_literal_str() {
    let ident = ident!("a_test_name");
    assert_eq!(ident, "a_test_name");
}

#[test]
fn from_expr() {
    let is_one = false;
    let ident = ident!(if is_one { "one" } else { "two" });
    assert_eq!(ident, "two");
}

#[test]
fn from_raw() {
    let ident = ident!(r#dyn);
    assert!(ident.is_raw());
    assert_eq!(ident, "r#dyn");
}

#[test]
fn from_concat() {
    let ident = ident!(this, "_", "is", "_", a, "_", test);
    assert_eq!(ident, "this_is_a_test");
}

#[test]
fn equality_compares_text_not_spans() {
    let tokens: moxy_token::TokenStream = "value value other r#value".parse().unwrap();
    let idents: Vec<moxy_token::Ident> = tokens
        .iter()
        .map(|token| match token {
            moxy_token::TokenTree::Ident(ident) => ident.clone(),
            other => panic!("expected an identifier, got {other}"),
        })
        .collect();

    assert_ne!(idents[0].span(), idents[1].span());
    assert_eq!(idents[0], idents[1]);
    assert_ne!(idents[0], idents[2]);
    assert_ne!(idents[0], idents[3]);
}

#[test]
fn punctuation_and_keywords_compare_by_text_like_identifiers() {
    let first: moxy_token::TokenStream = "fn a + b ; :: x".parse().unwrap();
    let second: moxy_token::TokenStream = "fn   a+b;   ::x".parse().unwrap();
    assert_eq!(first, second);

    let other: moxy_token::TokenStream = "fn a - b ; :: x".parse().unwrap();
    assert_ne!(first, other);

    let hash = |tokens: &moxy_token::TokenStream| {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        for token in tokens.iter() {
            if let moxy_token::TokenTree::Punct(punct) = token {
                punct.hash(&mut hasher);
            } else if let moxy_token::TokenTree::Keyword(keyword) = token {
                keyword.hash(&mut hasher);
            }
        }
        hasher.finish()
    };
    assert_eq!(hash(&first), hash(&second));
}
