use std::collections::{BTreeMap, HashMap};
use std::fmt::{Debug, Display};

extern crate core as rust_core;

pub type SharedMap<K, V> = HashMap<K, Vec<Option<V>>>;
pub const DEFAULT_LIMIT: usize = 64;
pub static mut GLOBAL_LIMIT: usize = 1024;

pub union Number {
    pub integer: u64,
    pub floating: f64,
}

pub trait Summarize<T>: Send + Sync {
    type Output: Clone;
    const LIMIT: usize = DEFAULT_LIMIT;
    fn summarize(&self, value: T) -> Self::Output;
}

pub struct Summary {
    pub id: usize,
    pub labels: Vec<String>,
}

pub enum TopLevel {
    Empty,
    Value { id: usize, label: String },
    Pair(usize, usize),
}

impl Summary {
    pub fn count(&self) -> usize {
        self.labels.len()
    }
}

extern "C" {
    fn fixture_callback(value: usize) -> usize;
    static mut FIXTURE_STATE: usize;
    type FixtureHandle;
    fixture_macro!();
}

macro_rules! top_level {
    ($value:expr) => { $value };
}

pub mod domain_01 {
    use super::{BTreeMap, Debug, Display};

    pub struct Record01 {
        pub id: usize,
        pub name: String,
        pub enabled: bool,
        pub retries: u16,
        pub tags: Vec<String>,
        pub mapping: BTreeMap<String, Vec<u8>>,
        pub payload: Option<Result<Vec<u8>, String>>,
        pub parent: Option<Box<Record01>>,
        pub children: Vec<Record01>,
        pub marker: core::marker::PhantomData<String>,
        pub created_at: u64,
        pub updated_at: u64,
    }

    pub enum Event01 {
        Empty,
        Created { id: usize, label: String },
        Updated(usize, Vec<u8>),
        Failed { reason: String, retries: u16 },
    }

    pub trait Service01: Send + Sync {
        type Output: Clone + Debug;
        const LIMIT: usize = 8;
        fn process(&self, value: usize) -> Self::Output;
    }

    impl Record01 {
        pub fn identifier(&self) -> usize {
            self.id
        }

        pub fn compute(&self, value: usize) -> usize {
            if value > self.id {
                value + self.retries as usize
            } else {
                self.id + self.retries as usize
            }
        }

        pub fn collect(&self) -> usize {
            let count = self.tags.len() + self.children.len();
            count + self.mapping.len()
        }
    }

    impl Service01 for Record01 {
        type Output = usize;

        fn process(&self, value: usize) -> Self::Output {
            self.compute(value)
        }
    }

    pub fn render01<T>(value: T, record: &Record01) -> usize
    where
        T: Display + Debug,
    {
        let measured = record.compute(record.id);
        if measured > 0 {
            measured
        } else {
            record.collect()
        }
    }

    pub const DOMAIN_LIMIT: usize = 16;
    pub static mut DOMAIN_STATE: usize = 1;
}

pub mod domain_02 {
    use super::{BTreeMap, Debug, Display};

    pub struct Record02 {
        pub id: usize,
        pub name: String,
        pub enabled: bool,
        pub retries: u16,
        pub tags: Vec<String>,
        pub mapping: BTreeMap<String, Vec<u8>>,
        pub payload: Option<Result<Vec<u8>, String>>,
        pub parent: Option<Box<Record02>>,
        pub children: Vec<Record02>,
        pub marker: core::marker::PhantomData<String>,
        pub created_at: u64,
        pub updated_at: u64,
    }

    pub enum Event02 {
        Empty,
        Created { id: usize, label: String },
        Updated(usize, Vec<u8>),
        Failed { reason: String, retries: u16 },
    }

    pub trait Service02: Send + Sync {
        type Output: Clone + Debug;
        const LIMIT: usize = 16;
        fn process(&self, value: usize) -> Self::Output;
    }

    impl Record02 {
        pub fn identifier(&self) -> usize {
            self.id
        }

        pub fn compute(&self, value: usize) -> usize {
            if value > self.id {
                value + self.retries as usize
            } else {
                self.id + self.retries as usize
            }
        }

        pub fn collect(&self) -> usize {
            let count = self.tags.len() + self.children.len();
            count + self.mapping.len()
        }
    }

    impl Service02 for Record02 {
        type Output = usize;

        fn process(&self, value: usize) -> Self::Output {
            self.compute(value)
        }
    }

    pub fn render02<T>(value: T, record: &Record02) -> usize
    where
        T: Display + Debug,
    {
        let measured = record.compute(record.id);
        if measured > 0 {
            measured
        } else {
            record.collect()
        }
    }

    pub const DOMAIN_LIMIT: usize = 32;
    pub static mut DOMAIN_STATE: usize = 2;
}

pub mod domain_03 {
    use super::{BTreeMap, Debug, Display};

    pub struct Record03 {
        pub id: usize,
        pub name: String,
        pub enabled: bool,
        pub retries: u16,
        pub tags: Vec<String>,
        pub mapping: BTreeMap<String, Vec<u8>>,
        pub payload: Option<Result<Vec<u8>, String>>,
        pub parent: Option<Box<Record03>>,
        pub children: Vec<Record03>,
        pub marker: core::marker::PhantomData<String>,
        pub created_at: u64,
        pub updated_at: u64,
    }

    pub enum Event03 {
        Empty,
        Created { id: usize, label: String },
        Updated(usize, Vec<u8>),
        Failed { reason: String, retries: u16 },
    }

    pub trait Service03: Send + Sync {
        type Output: Clone + Debug;
        const LIMIT: usize = 24;
        fn process(&self, value: usize) -> Self::Output;
    }

    impl Record03 {
        pub fn identifier(&self) -> usize {
            self.id
        }

        pub fn compute(&self, value: usize) -> usize {
            if value > self.id {
                value + self.retries as usize
            } else {
                self.id + self.retries as usize
            }
        }

        pub fn collect(&self) -> usize {
            let count = self.tags.len() + self.children.len();
            count + self.mapping.len()
        }
    }

    impl Service03 for Record03 {
        type Output = usize;

        fn process(&self, value: usize) -> Self::Output {
            self.compute(value)
        }
    }

    pub fn render03<T>(value: T, record: &Record03) -> usize
    where
        T: Display + Debug,
    {
        let measured = record.compute(record.id);
        if measured > 0 {
            measured
        } else {
            record.collect()
        }
    }

    pub const DOMAIN_LIMIT: usize = 48;
    pub static mut DOMAIN_STATE: usize = 3;
}

pub mod domain_04 {
    use super::{BTreeMap, Debug, Display};

    pub struct Record04 {
        pub id: usize,
        pub name: String,
        pub enabled: bool,
        pub retries: u16,
        pub tags: Vec<String>,
        pub mapping: BTreeMap<String, Vec<u8>>,
        pub payload: Option<Result<Vec<u8>, String>>,
        pub parent: Option<Box<Record04>>,
        pub children: Vec<Record04>,
        pub marker: core::marker::PhantomData<String>,
        pub created_at: u64,
        pub updated_at: u64,
    }

    pub enum Event04 {
        Empty,
        Created { id: usize, label: String },
        Updated(usize, Vec<u8>),
        Failed { reason: String, retries: u16 },
    }

    pub trait Service04: Send + Sync {
        type Output: Clone + Debug;
        const LIMIT: usize = 32;
        fn process(&self, value: usize) -> Self::Output;
    }

    impl Record04 {
        pub fn identifier(&self) -> usize {
            self.id
        }

        pub fn compute(&self, value: usize) -> usize {
            if value > self.id {
                value + self.retries as usize
            } else {
                self.id + self.retries as usize
            }
        }

        pub fn collect(&self) -> usize {
            let count = self.tags.len() + self.children.len();
            count + self.mapping.len()
        }
    }

    impl Service04 for Record04 {
        type Output = usize;

        fn process(&self, value: usize) -> Self::Output {
            self.compute(value)
        }
    }

    pub fn render04<T>(value: T, record: &Record04) -> usize
    where
        T: Display + Debug,
    {
        let measured = record.compute(record.id);
        if measured > 0 {
            measured
        } else {
            record.collect()
        }
    }

    pub const DOMAIN_LIMIT: usize = 64;
    pub static mut DOMAIN_STATE: usize = 4;
}

pub mod domain_05 {
    use super::{BTreeMap, Debug, Display};

    pub struct Record05 {
        pub id: usize,
        pub name: String,
        pub enabled: bool,
        pub retries: u16,
        pub tags: Vec<String>,
        pub mapping: BTreeMap<String, Vec<u8>>,
        pub payload: Option<Result<Vec<u8>, String>>,
        pub parent: Option<Box<Record05>>,
        pub children: Vec<Record05>,
        pub marker: core::marker::PhantomData<String>,
        pub created_at: u64,
        pub updated_at: u64,
    }

    pub enum Event05 {
        Empty,
        Created { id: usize, label: String },
        Updated(usize, Vec<u8>),
        Failed { reason: String, retries: u16 },
    }

    pub trait Service05: Send + Sync {
        type Output: Clone + Debug;
        const LIMIT: usize = 40;
        fn process(&self, value: usize) -> Self::Output;
    }

    impl Record05 {
        pub fn identifier(&self) -> usize {
            self.id
        }

        pub fn compute(&self, value: usize) -> usize {
            if value > self.id {
                value + self.retries as usize
            } else {
                self.id + self.retries as usize
            }
        }

        pub fn collect(&self) -> usize {
            let count = self.tags.len() + self.children.len();
            count + self.mapping.len()
        }
    }

    impl Service05 for Record05 {
        type Output = usize;

        fn process(&self, value: usize) -> Self::Output {
            self.compute(value)
        }
    }

    pub fn render05<T>(value: T, record: &Record05) -> usize
    where
        T: Display + Debug,
    {
        let measured = record.compute(record.id);
        if measured > 0 {
            measured
        } else {
            record.collect()
        }
    }

    pub const DOMAIN_LIMIT: usize = 80;
    pub static mut DOMAIN_STATE: usize = 5;
}

pub mod domain_06 {
    use super::{BTreeMap, Debug, Display};

    pub struct Record06 {
        pub id: usize,
        pub name: String,
        pub enabled: bool,
        pub retries: u16,
        pub tags: Vec<String>,
        pub mapping: BTreeMap<String, Vec<u8>>,
        pub payload: Option<Result<Vec<u8>, String>>,
        pub parent: Option<Box<Record06>>,
        pub children: Vec<Record06>,
        pub marker: core::marker::PhantomData<String>,
        pub created_at: u64,
        pub updated_at: u64,
    }

    pub enum Event06 {
        Empty,
        Created { id: usize, label: String },
        Updated(usize, Vec<u8>),
        Failed { reason: String, retries: u16 },
    }

    pub trait Service06: Send + Sync {
        type Output: Clone + Debug;
        const LIMIT: usize = 48;
        fn process(&self, value: usize) -> Self::Output;
    }

    impl Record06 {
        pub fn identifier(&self) -> usize {
            self.id
        }

        pub fn compute(&self, value: usize) -> usize {
            if value > self.id {
                value + self.retries as usize
            } else {
                self.id + self.retries as usize
            }
        }

        pub fn collect(&self) -> usize {
            let count = self.tags.len() + self.children.len();
            count + self.mapping.len()
        }
    }

    impl Service06 for Record06 {
        type Output = usize;

        fn process(&self, value: usize) -> Self::Output {
            self.compute(value)
        }
    }

    pub fn render06<T>(value: T, record: &Record06) -> usize
    where
        T: Display + Debug,
    {
        let measured = record.compute(record.id);
        if measured > 0 {
            measured
        } else {
            record.collect()
        }
    }

    pub const DOMAIN_LIMIT: usize = 96;
    pub static mut DOMAIN_STATE: usize = 6;
}

pub mod domain_07 {
    use super::{BTreeMap, Debug, Display};

    pub struct Record07 {
        pub id: usize,
        pub name: String,
        pub enabled: bool,
        pub retries: u16,
        pub tags: Vec<String>,
        pub mapping: BTreeMap<String, Vec<u8>>,
        pub payload: Option<Result<Vec<u8>, String>>,
        pub parent: Option<Box<Record07>>,
        pub children: Vec<Record07>,
        pub marker: core::marker::PhantomData<String>,
        pub created_at: u64,
        pub updated_at: u64,
    }

    pub enum Event07 {
        Empty,
        Created { id: usize, label: String },
        Updated(usize, Vec<u8>),
        Failed { reason: String, retries: u16 },
    }

    pub trait Service07: Send + Sync {
        type Output: Clone + Debug;
        const LIMIT: usize = 56;
        fn process(&self, value: usize) -> Self::Output;
    }

    impl Record07 {
        pub fn identifier(&self) -> usize {
            self.id
        }

        pub fn compute(&self, value: usize) -> usize {
            if value > self.id {
                value + self.retries as usize
            } else {
                self.id + self.retries as usize
            }
        }

        pub fn collect(&self) -> usize {
            let count = self.tags.len() + self.children.len();
            count + self.mapping.len()
        }
    }

    impl Service07 for Record07 {
        type Output = usize;

        fn process(&self, value: usize) -> Self::Output {
            self.compute(value)
        }
    }

    pub fn render07<T>(value: T, record: &Record07) -> usize
    where
        T: Display + Debug,
    {
        let measured = record.compute(record.id);
        if measured > 0 {
            measured
        } else {
            record.collect()
        }
    }

    pub const DOMAIN_LIMIT: usize = 112;
    pub static mut DOMAIN_STATE: usize = 7;
}

pub mod domain_08 {
    use super::{BTreeMap, Debug, Display};

    pub struct Record08 {
        pub id: usize,
        pub name: String,
        pub enabled: bool,
        pub retries: u16,
        pub tags: Vec<String>,
        pub mapping: BTreeMap<String, Vec<u8>>,
        pub payload: Option<Result<Vec<u8>, String>>,
        pub parent: Option<Box<Record08>>,
        pub children: Vec<Record08>,
        pub marker: core::marker::PhantomData<String>,
        pub created_at: u64,
        pub updated_at: u64,
    }

    pub enum Event08 {
        Empty,
        Created { id: usize, label: String },
        Updated(usize, Vec<u8>),
        Failed { reason: String, retries: u16 },
    }

    pub trait Service08: Send + Sync {
        type Output: Clone + Debug;
        const LIMIT: usize = 64;
        fn process(&self, value: usize) -> Self::Output;
    }

    impl Record08 {
        pub fn identifier(&self) -> usize {
            self.id
        }

        pub fn compute(&self, value: usize) -> usize {
            if value > self.id {
                value + self.retries as usize
            } else {
                self.id + self.retries as usize
            }
        }

        pub fn collect(&self) -> usize {
            let count = self.tags.len() + self.children.len();
            count + self.mapping.len()
        }
    }

    impl Service08 for Record08 {
        type Output = usize;

        fn process(&self, value: usize) -> Self::Output {
            self.compute(value)
        }
    }

    pub fn render08<T>(value: T, record: &Record08) -> usize
    where
        T: Display + Debug,
    {
        let measured = record.compute(record.id);
        if measured > 0 {
            measured
        } else {
            record.collect()
        }
    }

    pub const DOMAIN_LIMIT: usize = 128;
    pub static mut DOMAIN_STATE: usize = 8;
}

pub mod domain_09 {
    use super::{BTreeMap, Debug, Display};

    pub struct Record09 {
        pub id: usize,
        pub name: String,
        pub enabled: bool,
        pub retries: u16,
        pub tags: Vec<String>,
        pub mapping: BTreeMap<String, Vec<u8>>,
        pub payload: Option<Result<Vec<u8>, String>>,
        pub parent: Option<Box<Record09>>,
        pub children: Vec<Record09>,
        pub marker: core::marker::PhantomData<String>,
        pub created_at: u64,
        pub updated_at: u64,
    }

    pub enum Event09 {
        Empty,
        Created { id: usize, label: String },
        Updated(usize, Vec<u8>),
        Failed { reason: String, retries: u16 },
    }

    pub trait Service09: Send + Sync {
        type Output: Clone + Debug;
        const LIMIT: usize = 72;
        fn process(&self, value: usize) -> Self::Output;
    }

    impl Record09 {
        pub fn identifier(&self) -> usize {
            self.id
        }

        pub fn compute(&self, value: usize) -> usize {
            if value > self.id {
                value + self.retries as usize
            } else {
                self.id + self.retries as usize
            }
        }

        pub fn collect(&self) -> usize {
            let count = self.tags.len() + self.children.len();
            count + self.mapping.len()
        }
    }

    impl Service09 for Record09 {
        type Output = usize;

        fn process(&self, value: usize) -> Self::Output {
            self.compute(value)
        }
    }

    pub fn render09<T>(value: T, record: &Record09) -> usize
    where
        T: Display + Debug,
    {
        let measured = record.compute(record.id);
        if measured > 0 {
            measured
        } else {
            record.collect()
        }
    }

    pub const DOMAIN_LIMIT: usize = 144;
    pub static mut DOMAIN_STATE: usize = 9;
}

pub mod domain_10 {
    use super::{BTreeMap, Debug, Display};

    pub struct Record10 {
        pub id: usize,
        pub name: String,
        pub enabled: bool,
        pub retries: u16,
        pub tags: Vec<String>,
        pub mapping: BTreeMap<String, Vec<u8>>,
        pub payload: Option<Result<Vec<u8>, String>>,
        pub parent: Option<Box<Record10>>,
        pub children: Vec<Record10>,
        pub marker: core::marker::PhantomData<String>,
        pub created_at: u64,
        pub updated_at: u64,
    }

    pub enum Event10 {
        Empty,
        Created { id: usize, label: String },
        Updated(usize, Vec<u8>),
        Failed { reason: String, retries: u16 },
    }

    pub trait Service10: Send + Sync {
        type Output: Clone + Debug;
        const LIMIT: usize = 80;
        fn process(&self, value: usize) -> Self::Output;
    }

    impl Record10 {
        pub fn identifier(&self) -> usize {
            self.id
        }

        pub fn compute(&self, value: usize) -> usize {
            if value > self.id {
                value + self.retries as usize
            } else {
                self.id + self.retries as usize
            }
        }

        pub fn collect(&self) -> usize {
            let count = self.tags.len() + self.children.len();
            count + self.mapping.len()
        }
    }

    impl Service10 for Record10 {
        type Output = usize;

        fn process(&self, value: usize) -> Self::Output {
            self.compute(value)
        }
    }

    pub fn render10<T>(value: T, record: &Record10) -> usize
    where
        T: Display + Debug,
    {
        let measured = record.compute(record.id);
        if measured > 0 {
            measured
        } else {
            record.collect()
        }
    }

    pub const DOMAIN_LIMIT: usize = 160;
    pub static mut DOMAIN_STATE: usize = 10;
}

pub mod domain_11 {
    use super::{BTreeMap, Debug, Display};

    pub struct Record11 {
        pub id: usize,
        pub name: String,
        pub enabled: bool,
        pub retries: u16,
        pub tags: Vec<String>,
        pub mapping: BTreeMap<String, Vec<u8>>,
        pub payload: Option<Result<Vec<u8>, String>>,
        pub parent: Option<Box<Record11>>,
        pub children: Vec<Record11>,
        pub marker: core::marker::PhantomData<String>,
        pub created_at: u64,
        pub updated_at: u64,
    }

    pub enum Event11 {
        Empty,
        Created { id: usize, label: String },
        Updated(usize, Vec<u8>),
        Failed { reason: String, retries: u16 },
    }

    pub trait Service11: Send + Sync {
        type Output: Clone + Debug;
        const LIMIT: usize = 88;
        fn process(&self, value: usize) -> Self::Output;
    }

    impl Record11 {
        pub fn identifier(&self) -> usize {
            self.id
        }

        pub fn compute(&self, value: usize) -> usize {
            if value > self.id {
                value + self.retries as usize
            } else {
                self.id + self.retries as usize
            }
        }

        pub fn collect(&self) -> usize {
            let count = self.tags.len() + self.children.len();
            count + self.mapping.len()
        }
    }

    impl Service11 for Record11 {
        type Output = usize;

        fn process(&self, value: usize) -> Self::Output {
            self.compute(value)
        }
    }

    pub fn render11<T>(value: T, record: &Record11) -> usize
    where
        T: Display + Debug,
    {
        let measured = record.compute(record.id);
        if measured > 0 {
            measured
        } else {
            record.collect()
        }
    }

    pub const DOMAIN_LIMIT: usize = 176;
    pub static mut DOMAIN_STATE: usize = 11;
}

pub mod domain_12 {
    use super::{BTreeMap, Debug, Display};

    pub struct Record12 {
        pub id: usize,
        pub name: String,
        pub enabled: bool,
        pub retries: u16,
        pub tags: Vec<String>,
        pub mapping: BTreeMap<String, Vec<u8>>,
        pub payload: Option<Result<Vec<u8>, String>>,
        pub parent: Option<Box<Record12>>,
        pub children: Vec<Record12>,
        pub marker: core::marker::PhantomData<String>,
        pub created_at: u64,
        pub updated_at: u64,
    }

    pub enum Event12 {
        Empty,
        Created { id: usize, label: String },
        Updated(usize, Vec<u8>),
        Failed { reason: String, retries: u16 },
    }

    pub trait Service12: Send + Sync {
        type Output: Clone + Debug;
        const LIMIT: usize = 96;
        fn process(&self, value: usize) -> Self::Output;
    }

    impl Record12 {
        pub fn identifier(&self) -> usize {
            self.id
        }

        pub fn compute(&self, value: usize) -> usize {
            if value > self.id {
                value + self.retries as usize
            } else {
                self.id + self.retries as usize
            }
        }

        pub fn collect(&self) -> usize {
            let count = self.tags.len() + self.children.len();
            count + self.mapping.len()
        }
    }

    impl Service12 for Record12 {
        type Output = usize;

        fn process(&self, value: usize) -> Self::Output {
            self.compute(value)
        }
    }

    pub fn render12<T>(value: T, record: &Record12) -> usize
    where
        T: Display + Debug,
    {
        let measured = record.compute(record.id);
        if measured > 0 {
            measured
        } else {
            record.collect()
        }
    }

    pub const DOMAIN_LIMIT: usize = 192;
    pub static mut DOMAIN_STATE: usize = 12;
}

