mod abi;
mod bare_fn_arg;
mod fn_param;
mod receiver;
mod signature;
mod variadic;

pub use abi::*;
pub use bare_fn_arg::*;
pub use fn_param::*;
pub use receiver::*;
pub use signature::*;
pub use variadic::*;

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use moxy_token::{ToTokenStream, TokenStream};

    use super::*;

    #[test]
    fn signature_basic() {
        let s = moxy_token::parse!("fn foo(x: u8) -> u8" as Signature).unwrap();
        assert_eq!(s.ident.text, "foo");
        assert_eq!(s.params.inner.inputs.len(), 1);
        assert!(matches!(s.output, crate::ReturnType::Type(..)));
    }

    #[test]
    fn signature_generic_where() {
        let s = moxy_token::parse!("fn f<T>(x: T) where T: Clone" as Signature).unwrap();
        assert_eq!(s.generics.params.len(), 1);
        assert!(s.generics.where_clause.is_some());
    }

    #[test]
    fn receiver_param() {
        let s = moxy_token::parse!("fn m(&self, x: u8)" as Signature).unwrap();
        assert!(matches!(s.params.inner.inputs.first().unwrap(), FnParam::Receiver(_)));
    }

    #[test]
    fn bare_fn_type() {
        use crate::Type;
        assert!(matches!(moxy_token::parse!("fn(u8) -> u8" as Type).unwrap(), Type::BareFn(_)));
        assert_eq!(
            moxy_token::parse!("fn(u8) -> u8" as Type)
                .unwrap()
                .to_token_stream()
                .to_string(),
            "fn (u8) -> u8"
        );
    }
}
