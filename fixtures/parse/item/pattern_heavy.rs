pub fn classify(record: &(u8, u8), values: &[u8]) -> usize {
    match (record, values) {
        (&(0 | 1, value @ 2..=9), [head, middle @ .., tail]) => {
            usize::from(value) + usize::from(*head) + middle.len() + usize::from(*tail)
        }
        (&(_, value), [single]) if value > *single => usize::from(value),
        (&(_, _), []) => 0,
        _ => 1,
    }
}
