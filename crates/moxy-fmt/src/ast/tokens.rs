use moxy_ast::*;
use moxy_token::*;

use crate::*;

macro_rules! token_format_impl {
    ($($ident:ident,)* $(,)?) => {
        $(
            impl Format for $ident {
                fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
                    f.text(self.to_string())
                }
            }
        )*
    };
}

token_format_impl! {
    Box,
    Const,
    Mut,
    Dot,
    DotDot,
    Underscore,
    Async,
    Default,
    Not,
    Question,
    Static,
    Unsafe,
    Pound,
    Slash,
}
