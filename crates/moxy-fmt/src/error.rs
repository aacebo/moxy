use moxy_token::{Delim, Group, Ident, Lit, Punct, Span, ToTokenStream, ToTokens, Token, TokenStream, TokenTree};

#[derive(Debug)]
pub enum FmtError {
    Unsupported(String),
    Std(std::fmt::Error),
}

impl FmtError {
    pub fn unsupported(message: impl std::fmt::Display) -> Self {
        Self::Unsupported(message.to_string())
    }
}

impl FmtError {
    pub fn to_compile_error(&self) -> TokenStream {
        let ident = Ident::new("compile_error");
        let bang = <Token![!]>::new(Span::def_site());
        let lit = Lit::string(&self.to_string());
        let inner: TokenTree = lit.into();
        let group = Group::new(Delim::Paren, inner.into_token_stream());

        vec![
            TokenTree::from(ident),
            TokenTree::from(Punct::from(bang)),
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
            Self::Unsupported(v) => write!(f, "{v}"),
            Self::Std(err) => write!(f, "{err}"),
        }
    }
}

impl std::error::Error for FmtError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Std(err) => Some(err),
            _ => None,
        }
    }
}

impl ToTokens for FmtError {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.to_compile_error().to_tokens(tokens);
    }
}
