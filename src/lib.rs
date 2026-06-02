#[cfg(feature = "ast")]
pub mod ast {
    pub use moxy_ast::*;
}

#[cfg(feature = "fmt")]
pub mod fmt {
    pub use moxy_fmt::*;
}

#[cfg(feature = "diagnostic")]
pub mod diagnostic {
    pub use moxy_diagnostic::*;
}

#[cfg(feature = "macros")]
pub mod macros {
    pub use moxy_macros::*;
}

#[cfg(feature = "token")]
pub mod token {
    pub use moxy_token::*;
}

#[cfg(feature = "template")]
pub mod template {
    pub use moxy_template::*;
}
