const MIN: core::num::NonZeroU8 = moxy_macro_tests::nonzero_u8!(1);
const MAX: core::num::NonZeroU8 = moxy_macro_tests::nonzero_u8!(255);

const _: () = {
    assert!(MIN.get() == 1);
    assert!(MAX.get() == 255);
};

fn main() {}
