use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use moxy_ast::{Pair, Punctuated};
use moxy_token::punct::Comma;
use moxy_token::{Ident, ToTokenStream};

#[test]
fn punctuated_mutation_iteration_pairs_and_conversion_have_exact_ordered_output() {
    let mut values = Punctuated::<Ident, Comma>::new();
    assert!(values.is_empty());
    assert!(values.is_empty_or_trailing());
    values.push(Ident::new("alpha"));
    values.push(Ident::new("beta"));
    values.insert(1, Ident::new("middle"));
    assert_eq!(values.to_token_stream().to_string(), "alpha , middle , beta");
    assert_eq!(values.len(), 3);
    assert!(!values.is_trailing());
    assert_eq!(values.first().unwrap().text(), "alpha");
    assert_eq!(values.last().unwrap().text(), "beta");
    assert_eq!(values.get(1).unwrap().text(), "middle");
    assert!(values.get(3).is_none());

    *values.first_mut().unwrap() = Ident::new("first");
    *values.last_mut().unwrap() = Ident::new("last");
    *values.get_mut(1).unwrap() = Ident::new("center");
    values[0] = Ident::new("zero");
    assert_eq!(values[0].text(), "zero");
    assert_eq!(values.iter().map(Ident::text).collect::<Vec<_>>(), ["zero", "center", "last"]);

    for value in values.iter_mut() {
        *value = Ident::new(value.text().to_uppercase());
    }
    assert_eq!(
        values.iter().rev().map(Ident::text).collect::<Vec<_>>(),
        ["LAST", "CENTER", "ZERO"]
    );
    assert_eq!(
        values
            .clone()
            .into_iter()
            .map(|value| value.text().to_owned())
            .collect::<Vec<_>>(),
        ["ZERO", "CENTER", "LAST"]
    );
    assert_eq!(
        (&values).into_iter().map(Ident::text).collect::<Vec<_>>(),
        ["ZERO", "CENTER", "LAST"]
    );
    assert_eq!(
        values.span(),
        values.first().unwrap().span().join(values.last().unwrap().span())
    );

    let pairs = values.pairs().map(|pair| pair.into_tuple().1.is_some()).collect::<Vec<_>>();
    assert_eq!(pairs, [true, true, false]);
    for mut pair in values.pairs_mut() {
        if let Some(punct) = pair.punct_mut() {
            punct.set_span(Default::default());
        }
        let lowercase = pair.value().text().to_lowercase();
        **pair.value_mut() = Ident::new(lowercase);
    }
    assert_eq!(values.to_token_stream().to_string(), "zero , center , last");
    for value in &mut values {
        *value = Ident::new(format!("{}_mut", value.text()));
    }
    assert_eq!(values.to_token_stream().to_string(), "zero_mut , center_mut , last_mut");
    for value in &mut values {
        *value = Ident::new(value.text().trim_end_matches("_mut"));
    }

    let tuples = values.clone().into_pairs().map(Pair::into_tuple).collect::<Vec<_>>();
    assert_eq!(tuples.len(), 3);
    assert_eq!(tuples[0].0.text(), "zero");
    assert!(tuples[0].1.is_some());
    assert!(tuples[2].1.is_none());

    let mut cloned = values.clone();
    cloned.extend([Ident::new("fourth"), Ident::new("fifth")]);
    assert_eq!(cloned.to_token_stream().to_string(), "zero , center , last , fourth , fifth");
    let collected: Punctuated<Ident, Comma> = [Ident::new("a"), Ident::new("b")].into_iter().collect();
    assert_eq!(collected.to_token_stream().to_string(), "a , b");
    let from_pairs: Punctuated<Ident, Comma> = [
        Pair::new(Ident::new("x"), Some(Default::default())),
        Pair::new(Ident::new("y"), None),
    ]
    .into_iter()
    .collect();
    assert_eq!(from_pairs.to_token_stream().to_string(), "x , y");
    let mut extended_pairs: Punctuated<Ident, Comma> = [Ident::new("before")].into_iter().collect();
    extended_pairs.extend([
        Pair::Punctuated(Ident::new("x"), Default::default()),
        Pair::End(Ident::new("y")),
    ]);
    assert_eq!(extended_pairs.to_token_stream().to_string(), "before , x , y");

    let mut left_hash = DefaultHasher::new();
    values.hash(&mut left_hash);
    let mut right_hash = DefaultHasher::new();
    values.clone().hash(&mut right_hash);
    assert_eq!(left_hash.finish(), right_hash.finish());
    assert_eq!(format!("{values:?}"), format!("{:?}", values.clone()));

    assert!(matches!(cloned.pop(), Some(Pair::End(value)) if value.text() == "fifth"));
    assert_eq!(cloned.pop_punct().unwrap().to_string(), ",");
    assert!(matches!(cloned.pop(), Some(Pair::End(value)) if value.text() == "fourth"));
    assert_eq!(cloned[2].text(), "last");
    cloned.clear();
    assert!(cloned.is_empty());
}

#[test]
fn pair_accessors_and_double_ended_iterators_preserve_values_and_punctuation() {
    let mut pair: Pair<Ident, Comma> = Pair::new(Ident::new("value"), Some(Default::default()));
    assert_eq!(pair.value().text(), "value");
    assert_eq!(pair.punct().unwrap().to_string(), ",");
    *pair.value_mut() = Ident::new("changed");
    pair.punct_mut().unwrap().set_span(Default::default());
    assert_eq!(pair.clone().into_value().text(), "changed");
    let (value, punctuation) = pair.into_tuple();
    assert_eq!(value.text(), "changed");
    assert_eq!(punctuation.unwrap().to_string(), ",");

    let values: Punctuated<Ident, Comma> = [Ident::new("a"), Ident::new("b"), Ident::new("c")].into_iter().collect();
    let mut iter = values.iter();
    assert_eq!(iter.len(), 3);
    assert_eq!(iter.size_hint(), (3, Some(3)));
    assert_eq!(iter.next().unwrap().text(), "a");
    assert_eq!(iter.next_back().unwrap().text(), "c");
    assert_eq!(iter.clone().next().unwrap().text(), "b");

    let mut mutable = values.clone();
    let mut iter_mut = mutable.iter_mut();
    assert_eq!(iter_mut.len(), 3);
    assert_eq!(iter_mut.size_hint(), (3, Some(3)));
    *iter_mut.next().unwrap() = Ident::new("front");
    *iter_mut.next_back().unwrap() = Ident::new("back");
    drop(iter_mut);
    assert_eq!(mutable.to_token_stream().to_string(), "front , b , back");

    let mut pairs = values.pairs();
    assert_eq!(pairs.len(), 3);
    assert_eq!(pairs.size_hint(), (3, Some(3)));
    assert_eq!(pairs.next().unwrap().value().text(), "a");
    assert_eq!(pairs.next_back().unwrap().value().text(), "c");
    assert_eq!(pairs.clone().next().unwrap().value().text(), "b");

    let mut mutable_pairs = mutable.pairs_mut();
    assert_eq!(mutable_pairs.len(), 3);
    assert_eq!(mutable_pairs.size_hint(), (3, Some(3)));
    assert_eq!(mutable_pairs.next().unwrap().value().text(), "front");
    assert_eq!(mutable_pairs.next_back().unwrap().value().text(), "back");

    let mut owned = values.clone().into_pairs();
    assert_eq!(owned.len(), 3);
    assert_eq!(owned.size_hint(), (3, Some(3)));
    assert_eq!(owned.next().unwrap().into_value().text(), "a");
    assert_eq!(owned.next_back().unwrap().into_value().text(), "c");
    assert_eq!(owned.clone().next().unwrap().into_value().text(), "b");

    let mut values_iter = values.clone().into_iter();
    assert_eq!(values_iter.len(), 3);
    assert_eq!(values_iter.size_hint(), (3, Some(3)));
    assert_eq!(values_iter.next().unwrap().text(), "a");
    assert_eq!(values_iter.next_back().unwrap().text(), "c");
    assert_eq!(values_iter.clone().next().unwrap().text(), "b");

    let punctuated = Pair::Punctuated(Ident::new("left"), Comma::default());
    let end: Pair<Ident, Comma> = Pair::End(Ident::new("right"));
    assert_eq!(punctuated.clone().to_token_stream().to_string(), "left ,");
    assert_eq!(end.clone().to_token_stream().to_string(), "right");
}

#[cfg(feature = "serde")]
#[test]
fn punctuated_serde_output_is_an_ordered_value_array() {
    let values: Punctuated<Ident, Comma> = [Ident::new("alpha"), Ident::new("beta")].into_iter().collect();
    assert_eq!(serde_json::to_value(values).unwrap(), serde_json::json!(["alpha", "beta"]));
}
