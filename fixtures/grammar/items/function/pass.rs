pub unsafe extern "C" fn f<'a, T: Copy>(x: &'a T) -> T where T: 'a { *x }
