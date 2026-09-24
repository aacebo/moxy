struct S<'a, T: Clone + 'a, const N: usize>(&'a [T; N]);
