macro_rules! truncated {
    ($name:ident) => {
        pub struct $name {
            pub value: usize,
        }
    };

invoke!(truncated
