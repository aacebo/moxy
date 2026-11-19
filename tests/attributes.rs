use moxy::ast::{Attributed, Expr, Item, Meta};
use moxy::token::{Spanner, ToTokenStream};

#[test]
fn outer_attributes_are_attached_to_the_declaration_and_rendered() {
    let item: Item = moxy::parse!("#[repr(C)] #[derive(Clone, Debug)] pub struct Header { value: u32 }").unwrap();
    let structure = item.as_struct().unwrap();
    assert_eq!(structure.attrs.len(), 2);
    assert_eq!(structure.ident.text(), "Header");
    assert!(!structure.span().is_empty());
    debug_assert_eq!(
        moxy::fmt!(&item).unwrap(),
        "#[repr(C)]\n#[derive(Clone, Debug)]\npub struct Header {\n\tvalue: u32,\n}",
        "{item:#?}"
    );
}

#[test]
fn field_attributes_remain_with_the_field_syntax() {
    let item: Item = moxy::parse!("struct Packet { #[cfg(unix)] bytes: Vec<u8> }").unwrap();
    let structure = item.as_struct().unwrap();
    let fields = &structure.fields.as_named().unwrap().fields.inner;
    assert_eq!(fields.len(), 1);
    assert_eq!(fields[0].attrs.len(), 1);
    assert_eq!(fields[0].ident.as_ref().unwrap().text(), "bytes");
    assert_eq!(
        moxy::fmt!(&item).unwrap(),
        "struct Packet {\n\t#[cfg(unix)]\n\tbytes: Vec<u8>,\n}"
    );
}

#[test]
fn metadata_converts_builtin_scalar_values() {
    let value: Meta = moxy::parse!("value").unwrap();
    assert!(value.parse::<bool>().unwrap());

    let value: Meta = moxy::parse!("value = false").unwrap();
    assert!(!value.parse::<bool>().unwrap());

    let value: Meta = moxy::parse!("value = \"text\"").unwrap();
    assert_eq!(value.parse::<String>().unwrap(), "text");

    let value: Meta = moxy::parse!("value = 'x'").unwrap();
    assert_eq!(value.parse::<char>().unwrap(), 'x');

    let value: Meta = moxy::parse!("value = 1").unwrap();
    assert_eq!(value.parse::<u8>().unwrap(), 1);
    assert_eq!(value.parse::<u16>().unwrap(), 1);
    assert_eq!(value.parse::<u32>().unwrap(), 1);
    assert_eq!(value.parse::<u64>().unwrap(), 1);
    assert_eq!(value.parse::<u128>().unwrap(), 1);
    assert_eq!(value.parse::<usize>().unwrap(), 1);
    assert_eq!(value.parse::<i8>().unwrap(), 1);
    assert_eq!(value.parse::<i16>().unwrap(), 1);
    assert_eq!(value.parse::<i32>().unwrap(), 1);
    assert_eq!(value.parse::<i64>().unwrap(), 1);
    assert_eq!(value.parse::<i128>().unwrap(), 1);
    assert_eq!(value.parse::<isize>().unwrap(), 1);

    let value: Meta = moxy::parse!("value = -1").unwrap();
    assert_eq!(value.parse::<i8>().unwrap(), -1);
    assert_eq!(value.parse::<i16>().unwrap(), -1);
    assert_eq!(value.parse::<i32>().unwrap(), -1);
    assert_eq!(value.parse::<i64>().unwrap(), -1);
    assert_eq!(value.parse::<i128>().unwrap(), -1);
    assert_eq!(value.parse::<isize>().unwrap(), -1);

    let value: Meta = moxy::parse!("value = 1.5").unwrap();
    assert_eq!(value.parse::<f32>().unwrap(), 1.5);
    assert_eq!(value.parse::<f64>().unwrap(), 1.5);

    let value: Meta = moxy::parse!("value = -1.5").unwrap();
    assert_eq!(value.parse::<f32>().unwrap(), -1.5);
    assert_eq!(value.parse::<f64>().unwrap(), -1.5);
}

#[test]
fn metadata_rejects_scalar_overflow_and_invalid_literal_shapes() {
    let value: Meta = moxy::parse!("value = 256").unwrap();
    assert!(value.parse::<u8>().is_err());

    let value: Meta = moxy::parse!("value = -129").unwrap();
    assert!(value.parse::<i8>().is_err());

    let value: Meta = moxy::parse!("value = -1").unwrap();
    assert!(value.parse::<u8>().is_err());

    let value: Meta = moxy::parse!("value = 1").unwrap();
    assert!(value.parse::<String>().is_err());
    assert!(value.parse::<char>().is_err());
    assert!(value.parse::<f64>().is_err());

    let value: Meta = moxy::parse!("value(1)").unwrap();
    assert!(value.parse::<u8>().is_err());

    let value: Meta = moxy::parse!("value = 1 + 2").unwrap();
    assert!(value.parse::<u8>().is_err());

    let value: Meta = moxy::parse!("value = true && false").unwrap();
    assert!(value.parse::<bool>().is_err());
}

#[test]
fn expression_attributes_belong_to_the_operand_that_follows_them() {
    for (source, tokens) in [
        ("#[a] x.y()", "# [a] x . y ()"),
        ("#[a] x + y", "# [a] x + y"),
        ("#[a] -x", "# [a] - x"),
        ("#[a] x = 1", "# [a] x = 1"),
        ("#[a] x..y", "# [a] x .. y"),
        ("#[a] &x", "# [a] & x"),
        ("x + #[a] y", "x + # [a] y"),
    ] {
        let expression: Expr = moxy::parse!(source).unwrap_or_else(|error| panic!("`{source}`: {error}"));
        assert_eq!(expression.to_token_stream().to_string(), tokens);
    }

    let expression: Expr = moxy::parse!("#[a] x.y()").unwrap();
    let call = expression.as_method_call().unwrap();
    assert_eq!(call.attrs.len(), 1);
    assert!(call.receiver.attrs().unwrap().is_empty());

    let expression: Expr = moxy::parse!("#[a] x + y").unwrap();
    let binary = expression.as_binary().unwrap();
    assert!(binary.attrs.is_empty());
    assert_eq!(binary.left.attrs().unwrap().len(), 1);
    assert!(binary.right.attrs().unwrap().is_empty());

    let expression: Expr = moxy::parse!("#[a] -x").unwrap();
    assert_eq!(expression.as_unary().unwrap().attrs.len(), 1);

    let expression: Expr = moxy::parse!("x + #[a] y").unwrap();
    assert_eq!(expression.as_binary().unwrap().right.attrs().unwrap().len(), 1);
}

#[test]
fn outer_attributes_at_the_start_of_a_file_belong_to_its_first_item() {
    let file: moxy::ast::File = moxy::parse!("#![a] #[b] fn f() {} #[c] fn g() {}").unwrap();
    assert_eq!(file.attrs.len(), 1);
    assert!(file.attrs[0].style.is_inner());
    assert_eq!(file.items.len(), 2);
    assert_eq!(file.items[0].attrs().len(), 1);
    assert_eq!(file.items[0].to_token_stream().to_string(), "# [b] fn f () {}");

    let file: moxy::ast::File = moxy::parse!("#[macro_use] extern crate a;").unwrap();
    assert!(file.attrs.is_empty());
    assert_eq!(file.items[0].attrs().len(), 1);
}

#[test]
fn doc_comments_keep_their_text_as_written() {
    for (source, doc) in [
        ("///     code block\nfn f() {}", "\"     code block\""),
        ("/// trailing  \r\nfn f() {}", "\" trailing  \""),
        ("/** a /* nested */ b */ fn f() {}", "\" a /* nested */ b \""),
        ("/** multi\n * line\n */ fn f() {}", "\" multi\\n * line\\n \""),
        ("//! inner\nfn f() {}", "\" inner\""),
    ] {
        let tokens: moxy::token::TokenStream = source.parse().unwrap_or_else(|error| panic!("`{source}`: {error:?}"));
        let expected: proc_macro2::TokenStream = source.parse().unwrap();
        let text = tokens.to_string();
        assert!(text.contains(doc), "`{source}` lexed as `{text}`");
        assert_eq!(text.replace(' ', ""), expected.to_string().replace(' ', ""), "`{source}`");
    }
}

// The texts of a token stream's tokens, with each group's delimiters around
// its contents
fn token_texts(tokens: &moxy::token::TokenStream, texts: &mut Vec<String>) {
    for token in tokens.iter() {
        if let moxy::token::TokenTree::Group(group) = token {
            let (open, close) = match group.delim {
                moxy::token::Delim::Paren => ("(", ")"),
                moxy::token::Delim::Bracket => ("[", "]"),
                moxy::token::Delim::Brace => ("{", "}"),
                moxy::token::Delim::None => ("", ""),
            };
            texts.push(open.to_string());
            token_texts(&group.tokens, texts);
            texts.push(close.to_string());
        } else {
            texts.push(token.to_string());
        }
    }
}

// Every attribute of every differential corpus source formats to text that
// lexes to the attribute's own tokens
#[test]
fn formatted_attributes_lex_to_their_own_tokens() {
    use moxy::ast::{Attribute, Visit};

    #[derive(Default)]
    struct Attrs(Vec<Attribute>);

    impl<'ast> Visit<'ast> for Attrs {
        fn visit_attribute(&mut self, node: &'ast Attribute) {
            self.0.push(node.clone());
        }
    }

    let mut attrs = Attrs::default();

    for line in include_str!("differential/corpus.txt").lines() {
        let Some(source) = line.strip_prefix("f: ") else {
            continue;
        };
        let source = source.replace("\\n", "\n");

        if let Ok(file) = moxy::parse!(source as moxy::ast::File) {
            file.attrs.iter().for_each(|attr| attrs.visit_attribute(attr));
            file.items.iter().for_each(|item| attrs.visit_item(item));
        }
    }

    assert!(attrs.0.len() > 100, "{} attributes", attrs.0.len());
    let mut mismatches = Vec::new();

    for attr in &attrs.0 {
        let formatted = moxy::fmt!(attr).unwrap();
        let (mut expected, mut actual) = (Vec::new(), Vec::new());
        token_texts(&attr.to_token_stream(), &mut expected);

        if let Ok(tokens) = formatted.parse::<moxy::token::TokenStream>() {
            token_texts(&tokens, &mut actual);
        }

        if expected != actual {
            mismatches.push(format!("{formatted}: {expected:?}"));
        }
    }

    assert!(mismatches.is_empty(), "{}", mismatches.join("\n"));
}

#[test]
fn an_attribute_value_is_an_expression() {
    for source in ["#[a = 1] fn f() {}", "#[a = \"b\"] fn f() {}", "#[a(b = c)] fn f() {}"] {
        assert!(moxy::parse!(source as moxy::ast::File).is_ok(), "`{source}`");
    }

    for source in ["#[a = ] fn f() {}", "#[a = ,] fn f() {}"] {
        assert!(moxy::parse!(source as moxy::ast::File).is_err(), "`{source}`");
    }
}

#[test]
fn inner_attributes_start_a_container_before_any_outer_attribute() {
    for source in ["#![a] fn f() {}", "fn f() { #![a] x }", "mod m { #![a] #[b] fn f() {} }"] {
        assert!(moxy::parse!(source as moxy::ast::File).is_ok(), "`{source}`");
    }

    for source in [
        "#[a] #![b] fn f() {}",
        "fn f() { let x = [#![a] 1]; }",
        "fn f() { g(#![a] 1); }",
    ] {
        assert!(moxy::parse!(source as moxy::ast::File).is_err(), "`{source}`");
    }

    assert!(moxy::parse!("#![a] x" as Expr).is_err());
}
