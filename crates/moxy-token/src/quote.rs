use crate::{ToTokens, TokenStream};

#[macro_export]
macro_rules! quoted {
    [&$input:expr] => { $crate::Quoted(&$input) };
    [$input:expr] => { $crate::Quoted(&$input) };
}

#[doc(hidden)]
pub struct Quoted<'a, T: quote::ToTokens>(pub &'a T);

impl<'a, T: quote::ToTokens> Quoted<'a, T> {
    pub fn new<V: AsRef<T> + 'a>(value: &'a V) -> Self {
        Self(value.as_ref())
    }
}

impl<'a, T: quote::ToTokens> From<&'a T> for Quoted<'a, T> {
    fn from(value: &'a T) -> Self {
        Self(value)
    }
}

impl<'a, T: quote::ToTokens> ToTokens for Quoted<'a, T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let out = quote::ToTokens::to_token_stream(self.0);
        let out: TokenStream = out.into();
        tokens.extend(out);
    }
}
