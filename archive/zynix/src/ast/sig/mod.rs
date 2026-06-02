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

    use super::*;
    use crate::TokenStream;
    use crate::token::ToTokenStream;

    fn parse<T: crate::Parse>(src: &str) -> T {
        let ts = TokenStream::from_str(src).unwrap();
        ts.parse().parse::<T>().unwrap()
    }

    #[test]
    fn signature_basic() {
        let s: Signature = parse("fn foo(x: u8) -> u8");
        assert_eq!(s.ident.text, "foo");
        assert_eq!(s.inputs.len(), 1);
        assert!(matches!(s.output, crate::ast::ReturnType::Type(_)));
    }

    #[test]
    fn signature_generic_where() {
        let s: Signature = parse("fn f<T>(x: T) where T: Clone");
        assert_eq!(s.generics.params.len(), 1);
        assert!(s.generics.where_clause.is_some());
    }

    #[test]
    fn receiver_param() {
        let s: Signature = parse("fn m(&self, x: u8)");
        assert!(matches!(s.inputs.first().unwrap(), FnParam::Receiver(_)));
    }

    #[test]
    fn bare_fn_type() {
        use crate::ast::Type;
        assert!(matches!(parse::<Type>("fn(u8) -> u8"), Type::BareFn(_)));
        assert_eq!(parse::<Type>("fn(u8) -> u8").to_token_stream().to_string(), "fn (u8) -> u8");
    }
}
