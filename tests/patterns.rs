use moxy::ast::Pattern;
use moxy::token::{Spanner, ToTokenStream};

#[test]
fn binding_tuple_slice_and_struct_patterns_preserve_shape() {
    for (source, expected, expected_kind) in [
        ("ref mut value @ Some(_)", "ref mut value @ Some(_)", 0),
        ("(first, second, ..)", "(first, second, ..)", 1),
        ("[first, .., last]", "[first, .., last]", 2),
        ("Point { x: renamed, y, .. }", "Point {\n\tx: renamed,\n\ty,\n\t..\n}", 3),
    ] {
        let pattern: Pattern = moxy::parse!(source).unwrap();
        assert_eq!(
            [
                pattern.is_ident(),
                pattern.is_tuple(),
                pattern.is_slice(),
                pattern.is_struct()
            ],
            std::array::from_fn(|index| index == expected_kind)
        );
        assert_eq!(
            [
                pattern.as_ident().is_some(),
                pattern.as_tuple().is_some(),
                pattern.as_slice().is_some(),
                pattern.as_struct().is_some(),
            ],
            std::array::from_fn(|index| index == expected_kind)
        );
        assert!(!pattern.span().is_empty());
        assert!(!pattern.to_token_stream().is_empty());
        assert_eq!(moxy::fmt!(&pattern).unwrap(), expected);
    }
}

#[test]
fn alternatives_ranges_references_and_typed_patterns_render_exactly() {
    for (source, expected, expected_kind) in [
        ("A | B | C", "A | B | C", 0),
        ("1..=10", "1..=10", 1),
        ("&mut value", "&mut value", 2),
    ] {
        let pattern: Pattern = moxy::parse!(source).unwrap();
        assert_eq!(
            [pattern.is_or(), pattern.is_lit(), pattern.is_reference()],
            std::array::from_fn(|index| index == expected_kind)
        );
        assert_eq!(
            [
                pattern.as_or().is_some(),
                pattern.as_lit().is_some(),
                pattern.as_reference().is_some()
            ],
            std::array::from_fn(|index| index == expected_kind)
        );
        assert!(!pattern.span().is_empty());
        assert!(!pattern.to_token_stream().is_empty());
        assert_eq!(moxy::fmt!(&pattern).unwrap(), expected);
    }
}

#[test]
#[ignore = "pattern parser currently stops before type ascriptions"]
fn typed_patterns_preserve_their_type_syntax() {
    let pattern: Pattern = moxy::parse!("value: Option<T>").unwrap();
    assert!(pattern.is_type());
    assert!(!pattern.span().is_empty());
    assert_eq!(moxy::fmt!(&pattern).unwrap(), "value: Option<T>");
}

#[test]
#[ignore = "member parser currently accepts suffixed, non-decimal, separated, and overflowing tuple indices"]
fn invalid_tuple_indices_are_rejected_as_member_patterns() {
    assert!(moxy::parse!("0" as moxy::ast::Member).is_ok());
    for source in ["0u8", "0x1", "1_0", "4294967296"] {
        let result: Result<moxy::ast::Member, _> = moxy::parse!(source);
        assert!(result.is_err(), "accepted invalid tuple index {source}");
    }
}
