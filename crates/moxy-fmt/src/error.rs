use moxy_ast::Ident;
use moxy_token::{Delim, Group, Lit, Not, Punctuation, Span, ToTokenStream, ToTokens, TokenStream, TokenTree};

#[derive(Debug)]
pub enum FmtError {
    Std(std::fmt::Error),
}

impl FmtError {
    pub fn to_compile_error(&self) -> TokenStream {
        let ident = Ident::new("compile_error");
        let bang = Not::new(Span::def_site());
        let lit = Lit::string(&self.to_string());
        let inner: TokenTree = lit.into();
        let group = Group::new(Delim::Paren, inner.into_token_stream());

        vec![
            TokenTree::from(ident),
            TokenTree::from(Punctuation::from(bang)),
            TokenTree::from(group),
        ]
        .into()
    }
}

impl From<std::fmt::Error> for FmtError {
    fn from(value: std::fmt::Error) -> Self {
        Self::Std(value)
    }
}

impl std::fmt::Display for FmtError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Std(err) => write!(f, "{err}"),
        }
    }
}

impl std::error::Error for FmtError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Std(err) => Some(err),
        }
    }
}

impl ToTokens for FmtError {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.to_compile_error().to_tokens(tokens);
    }
}
