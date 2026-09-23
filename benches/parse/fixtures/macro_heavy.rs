macro_rules! define_record {
    ($name:ident, $field:ident: $ty:ty) => {
        pub struct $name {
            pub $field: $ty,
        }
    };
}

macro_rules! call_many {
    ($value:expr) => {{
        trace!($value);
        audit!($value);
        $value
    }};
}

macro_rules! define_module {
    ($name:ident) => {
        pub mod $name {
            invoke! { $name }
        }
    };
}

define_record!(Packet, bytes: Vec<u8>);
define_record!(Response, status: u16);
define_module!(generated);

#[derive(Clone, Debug)]
#[register_component(name = "macro_fixture", features(trace, metrics))]
pub struct MacroFixture {
    pub handler: handler!(Request, Response),
    pub marker: marker!(MacroFixture),
}

pub type GeneratedType = generated_type!(Request, Response, Error);

pub fn process(value: usize) -> Result<usize, Error> {
    let parsed = parse_value!(value, mode = strict);
    let checked = validate!(parsed, Rule::Default)?;

    if enabled!(feature = "trace") {
        log::debug!("processed = {:?}", checked);
    }

    Ok(call_many!(checked))
}

pub fn patterns(value: Option<usize>) -> usize {
    match value {
        Some(value @ capture!(number)) => transform!(value),
        None => fallback!(),
    }
}

extern "C" {
    foreign_macro!();
}

macro_rules! nested {
    ($($item:item)*) => { $($item)* };
}

nested! {
    pub const GENERATED: usize = calculate!(1, 2, 3);
    pub static mut STATE: usize = state!(initial = 0);
}
