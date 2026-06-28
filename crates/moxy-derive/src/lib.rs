use moxy_ast::Declaration;
use moxy_diagnostic::SpanExt;
use moxy_fmt::fmt;
use moxy_token::{Spanner, TokenStream};

#[proc_macro_derive(ToTokens, attributes(template))]
pub fn derive_to_tokens(target: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let target = TokenStream::from(target);
    let declr: Declaration = match target.parse().parse() {
        Err(err) => {
            return err.span().error(target).emit().into();
        }
        Ok(v) => v,
    };

    let formatted = match fmt!(&declr) {
        Err(err) => panic!("{}", err),
        Ok(v) => v,
    };

    declr.span().note(formatted).emit();
    Default::default()
}
