use moxy_ast::fields::Fields;
use moxy_ast::generics::{GenericParam, TypeBound, WherePredicate};
use moxy_ast::member::{ForeignItem, ImplItem, TraitItem};
use moxy_ast::use_tree::UseTree;
use moxy_ast::{Declaration, Visibility};
use moxy_token::{Span, Spanner, ToTokenStream};

#[test]
fn visibility_variants_expose_exact_kind_span_and_tokens() {
    for (source, output, expected) in [
        ("", "", 0),
        ("pub", "pub", 1),
        ("pub(crate)", "pub (crate)", 2),
        ("pub(self)", "pub (self)", 3),
        ("pub(super)", "pub (super)", 4),
        ("pub(in module::nested)", "pub (in module :: nested)", 5),
    ] {
        let visibility: Visibility = moxy_token::parse!(source).unwrap();
        assert_eq!(
            [
                visibility.is_inherited(),
                visibility.is_public(),
                visibility.is_crate(),
                visibility.is_self_value(),
                visibility.is_super(),
                visibility.is_restricted(),
            ],
            std::array::from_fn(|index| index == expected)
        );
        assert_eq!(visibility.to_token_stream().to_string(), output);
        if expected == 0 {
            assert_eq!(visibility.span(), Span::call_site());
        } else {
            assert!(!visibility.span().is_empty());
        }
    }
}

#[test]
fn declarations_expose_each_concrete_shape_metadata_span_conversion_and_tokens() {
    let enumeration: Declaration = moxy_token::parse!("#[repr(u8)] pub enum Choice<T> { Value(T) }").unwrap();
    assert_eq!(
        [enumeration.is_enum(), enumeration.is_struct(), enumeration.is_union()],
        [true, false, false]
    );
    assert_eq!(enumeration.attrs().len(), 1);
    assert!(enumeration.vis().is_public());
    assert_eq!(enumeration.ident().text(), "Choice");
    assert_eq!(enumeration.generics().params.len(), 1);
    assert_eq!(
        enumeration.to_token_stream().to_string(),
        "# [repr (u8)] pub enum Choice < T > {Value (T)}"
    );
    assert_eq!(enumeration.span(), enumeration.as_enum().unwrap().span());
    assert!(enumeration.as_struct().is_none() && enumeration.as_union().is_none());

    let item_struct = moxy_token::parse!("pub(crate) struct Record<T> { value: T }" as moxy_ast::item::ItemStruct).unwrap();
    let structure = Declaration::from(item_struct.clone());
    assert_eq!(
        [structure.is_enum(), structure.is_struct(), structure.is_union()],
        [false, true, false]
    );
    assert_eq!(structure.ident().text(), "Record");
    assert_eq!(structure.generics().params.len(), 1);
    assert_eq!(structure.to_token_stream(), item_struct.to_token_stream());
    assert_eq!(structure.span(), structure.as_struct().unwrap().span());
    assert!(structure.as_enum().is_none() && structure.as_union().is_none());

    let item_union = moxy_token::parse!("union Storage<T> { value: T, empty: () }" as moxy_ast::item::ItemUnion).unwrap();
    let union = Declaration::from(item_union.clone());
    assert_eq!([union.is_enum(), union.is_struct(), union.is_union()], [false, false, true]);
    assert!(union.vis().is_inherited());
    assert_eq!(union.ident().text(), "Storage");
    assert_eq!(union.to_token_stream(), item_union.to_token_stream());
    assert_eq!(union.span(), union.as_union().unwrap().span());
    assert!(union.as_enum().is_none() && union.as_struct().is_none());

    let error = moxy_token::parse!("fn unrelated() {}" as Declaration).unwrap_err();
    assert_eq!(error.message(), "expected a user defined type declaration");
}

#[test]
fn generic_parameters_bounds_and_predicates_preserve_public_variants_and_output() {
    let lifetime: GenericParam = moxy_token::parse!("'a: 'b").unwrap();
    let ty: GenericParam = moxy_token::parse!("T: Clone = String").unwrap();
    let constant: GenericParam = moxy_token::parse!("const N: usize = 4").unwrap();
    assert_eq!(
        [lifetime.is_lifetime(), lifetime.is_type(), lifetime.is_const()],
        [true, false, false]
    );
    assert_eq!([ty.is_lifetime(), ty.is_type(), ty.is_const()], [false, true, false]);
    assert_eq!(
        [constant.is_lifetime(), constant.is_type(), constant.is_const()],
        [false, false, true]
    );
    assert_eq!(lifetime.as_lifetime().unwrap().to_token_stream().to_string(), "'a : 'b");
    assert_eq!(ty.as_type().unwrap().to_token_stream().to_string(), "T : Clone = String");
    assert_eq!(
        constant.as_const().unwrap().to_token_stream().to_string(),
        "const N : usize = 4"
    );
    assert!(lifetime.as_type().is_none() && ty.as_const().is_none() && constant.as_lifetime().is_none());
    assert_eq!(
        GenericParam::from(lifetime.as_lifetime().unwrap().clone()).to_token_stream(),
        lifetime.to_token_stream()
    );
    assert_eq!(
        GenericParam::from(ty.as_type().unwrap().clone()).to_token_stream(),
        ty.to_token_stream()
    );
    assert_eq!(
        GenericParam::from(constant.as_const().unwrap().clone()).to_token_stream(),
        constant.to_token_stream()
    );
    assert!(!lifetime.span().is_empty() && !ty.span().is_empty() && !constant.span().is_empty());

    let trait_bound: TypeBound = moxy_token::parse!("?Clone").unwrap();
    let lifetime_bound: TypeBound = moxy_token::parse!("'a").unwrap();
    let use_bound: TypeBound = moxy_token::parse!("use<'a, 'b>").unwrap();
    assert_eq!(
        [trait_bound.is_trait(), trait_bound.is_lifetime(), trait_bound.is_use()],
        [true, false, false]
    );
    assert_eq!(
        [
            lifetime_bound.is_trait(),
            lifetime_bound.is_lifetime(),
            lifetime_bound.is_use()
        ],
        [false, true, false]
    );
    assert_eq!(
        [use_bound.is_trait(), use_bound.is_lifetime(), use_bound.is_use()],
        [false, false, true]
    );
    assert_eq!(trait_bound.as_trait().unwrap().to_token_stream().to_string(), "? Clone");
    assert_eq!(lifetime_bound.as_lifetime().unwrap().to_token_stream().to_string(), "'a");
    assert_eq!(use_bound.as_use().unwrap().to_token_stream().to_string(), "use < 'a , 'b >");
    assert_eq!(
        TypeBound::from(trait_bound.as_trait().unwrap().clone()).to_token_stream(),
        trait_bound.to_token_stream()
    );
    assert_eq!(
        TypeBound::from(use_bound.as_use().unwrap().clone()).to_token_stream(),
        use_bound.to_token_stream()
    );
    assert!(trait_bound.as_use().is_none() && lifetime_bound.as_trait().is_none() && use_bound.as_lifetime().is_none());

    let lifetime_predicate: WherePredicate = moxy_token::parse!("'a: 'b + 'c").unwrap();
    let type_predicate: WherePredicate = moxy_token::parse!("for<'a> &'a T: Clone + 'a").unwrap();
    assert!(matches!(lifetime_predicate, WherePredicate::Lifetime(_)));
    assert!(matches!(type_predicate, WherePredicate::Type(_)));
    assert_eq!(lifetime_predicate.to_token_stream().to_string(), "'a : 'b + 'c");
    assert_eq!(type_predicate.to_token_stream().to_string(), "for < 'a > & 'a T : Clone + 'a");
    assert!(!lifetime_predicate.span().is_empty() && !type_predicate.span().is_empty());
}

#[test]
fn use_trees_and_fields_report_real_shapes_and_exact_tokens() {
    for (source, expected) in [
        ("root::child", 0),
        ("name", 1),
        ("old as new", 2),
        ("*", 3),
        ("{one, two as second, nested::*}", 4),
    ] {
        let tree: UseTree = moxy_token::parse!(source).unwrap();
        assert_eq!(
            [
                tree.is_path(),
                tree.is_name(),
                tree.is_rename(),
                tree.is_glob(),
                tree.is_group()
            ],
            std::array::from_fn(|index| index == expected)
        );
        assert_eq!(
            [
                tree.as_path().is_some(),
                tree.as_name().is_some(),
                tree.as_rename().is_some(),
                tree.as_glob().is_some(),
                tree.as_group().is_some(),
            ],
            std::array::from_fn(|index| index == expected)
        );
        assert!(!tree.span().is_empty());
        let tokens = tree.to_token_stream();
        let mut stream = tokens.parse();
        assert_eq!(stream.parse::<UseTree>().unwrap(), tree);
        assert!(stream.is_empty());
    }

    let named: Fields = moxy_token::parse!("{ pub name: String, count: usize }").unwrap();
    let unnamed: Fields = moxy_token::parse!("(pub String, usize)").unwrap();
    let unit: Fields = moxy_token::parse!("").unwrap();
    assert_eq!([named.is_named(), named.is_unnamed(), named.is_unit()], [true, false, false]);
    assert_eq!(
        [unnamed.is_named(), unnamed.is_unnamed(), unnamed.is_unit()],
        [false, true, false]
    );
    assert_eq!([unit.is_named(), unit.is_unnamed(), unit.is_unit()], [false, false, true]);
    assert_eq!(
        named.as_named().unwrap().to_token_stream().to_string(),
        "{pub name : String , count : usize}"
    );
    assert_eq!(
        unnamed.as_unnamed().unwrap().to_token_stream().to_string(),
        "(pub String , usize)"
    );
    assert_eq!(unit.to_token_stream().to_string(), "");
    assert_eq!(Fields::from(named.as_named().unwrap().clone()), named);
    assert_eq!(Fields::from(unnamed.as_unnamed().unwrap().clone()), unnamed);
    assert!(named.as_unnamed().is_none() && unnamed.as_named().is_none());
    assert!(!named.span().is_empty() && !unnamed.span().is_empty());
    assert_eq!(unit.span(), Span::call_site());
}

#[test]
fn impl_trait_and_foreign_members_expose_variants_conversions_spans_and_tokens() {
    for (source, expected) in [
        ("fn run() {}", 0),
        ("const VALUE: usize = 1;", 1),
        ("type Output = usize;", 2),
        ("call!();", 3),
    ] {
        let item: ImplItem = moxy_token::parse!(source).unwrap();
        assert_eq!(
            [item.is_fn(), item.is_const(), item.is_type(), item.is_macro()],
            std::array::from_fn(|index| index == expected)
        );
        assert_eq!(
            [
                item.as_fn().is_some(),
                item.as_const().is_some(),
                item.as_type().is_some(),
                item.as_macro().is_some()
            ],
            std::array::from_fn(|index| index == expected)
        );
        let converted = match expected {
            0 => ImplItem::from(item.as_fn().unwrap().clone()),
            1 => ImplItem::from(item.as_const().unwrap().clone()),
            2 => ImplItem::from(item.as_type().unwrap().clone()),
            3 => ImplItem::from(item.as_macro().unwrap().clone()),
            _ => unreachable!(),
        };
        assert_eq!(converted.to_token_stream(), item.to_token_stream());
        assert!(!item.span().is_empty());
    }

    for (source, expected) in [
        ("fn run();", 0),
        ("const VALUE: usize;", 1),
        ("type Output;", 2),
        ("call!();", 3),
    ] {
        let item: TraitItem = moxy_token::parse!(source).unwrap();
        assert_eq!(
            [item.is_fn(), item.is_const(), item.is_type(), item.is_macro()],
            std::array::from_fn(|index| index == expected)
        );
        assert_eq!(
            [
                item.as_fn().is_some(),
                item.as_const().is_some(),
                item.as_type().is_some(),
                item.as_macro().is_some()
            ],
            std::array::from_fn(|index| index == expected)
        );
        let converted = match expected {
            0 => TraitItem::from(item.as_fn().unwrap().clone()),
            1 => TraitItem::from(item.as_const().unwrap().clone()),
            2 => TraitItem::from(item.as_type().unwrap().clone()),
            3 => TraitItem::from(item.as_macro().unwrap().clone()),
            _ => unreachable!(),
        };
        assert_eq!(converted.to_token_stream(), item.to_token_stream());
        assert!(!item.span().is_empty());
    }

    for (source, expected) in [
        ("fn run();", 0),
        ("static VALUE: usize;", 1),
        ("type Output;", 2),
        ("call!();", 3),
    ] {
        let item: ForeignItem = moxy_token::parse!(source).unwrap();
        assert_eq!(
            [item.is_fn(), item.is_static(), item.is_type(), item.is_macro()],
            std::array::from_fn(|index| index == expected)
        );
        assert_eq!(
            [
                item.as_fn().is_some(),
                item.as_static().is_some(),
                item.as_type().is_some(),
                item.as_macro().is_some()
            ],
            std::array::from_fn(|index| index == expected)
        );
        let converted = match expected {
            0 => ForeignItem::from(item.as_fn().unwrap().clone()),
            1 => ForeignItem::from(item.as_static().unwrap().clone()),
            2 => ForeignItem::from(item.as_type().unwrap().clone()),
            3 => ForeignItem::from(item.as_macro().unwrap().clone()),
            _ => unreachable!(),
        };
        assert_eq!(converted.to_token_stream(), item.to_token_stream());
        assert!(!item.span().is_empty());
    }
}
