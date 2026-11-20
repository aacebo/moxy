pub struct Unit<const N: usize = 0>;

struct Tuple<P, const N: usize = 0>(P);

struct Named<'a, T: 'a, const N: usize = 0, const ENABLED: bool = true> {
    value: &'a T,
    bytes: [u8; N],
}

struct Multiple<const A: u8 = 0, const B: u8 = { 1 + 1 }>;
