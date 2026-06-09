use moxy_template::paste;

paste! {
    fn {{ get_ value }}() -> u32 {
        7
    }
}

#[test]
fn mints_fn_name() {
    assert_eq!(get_value(), 7);
}

paste! {
    struct {{ My Struct }} {
        {{ the_ field }}: u32,
    }
}

#[test]
fn mints_struct_and_field_names() {
    let s = MyStruct { the_field: 42 };
    assert_eq!(s.the_field, 42);
}

paste! {
    fn {{ from_ "str" }}() -> &'static str {
        "ok"
    }
}

#[test]
fn string_literal_segment_is_unquoted() {
    assert_eq!(from_str(), "ok");
}

paste! {
    fn passthrough<T: Clone>(x: T) -> (T, T) {
        (x.clone(), x)
    }
}

#[test]
fn passes_through_generics_and_groups() {
    assert_eq!(passthrough(3), (3, 3));
}
