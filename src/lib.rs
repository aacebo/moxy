#[cfg(feature = "ast")]
#[doc(inline)]
pub use moxy_ast as ast;

#[cfg(feature = "build")]
#[doc(inline)]
pub use moxy_build as build;

#[cfg(feature = "derive")]
#[doc(inline)]
pub use moxy_derive::*;

#[cfg(feature = "diagnostic")]
#[doc(inline)]
pub use moxy_diagnostic as diagnostic;

#[cfg(feature = "diagnostic")]
#[doc(inline)]
pub use moxy_diagnostic::{error, help, note, warn};

#[cfg(feature = "fmt")]
#[doc(inline)]
pub use moxy_fmt as fmt;

#[cfg(feature = "fmt")]
#[doc(inline)]
pub use moxy_fmt::fmt;

#[cfg(feature = "template")]
#[doc(inline)]
pub use moxy_template as template;

#[cfg(feature = "template")]
#[doc(inline)]
pub use moxy_template::*;

#[cfg(feature = "token")]
#[doc(inline)]
pub use moxy_token as token;

#[cfg(all(feature = "token", not(feature = "ast")))]
#[doc(inline)]
pub use moxy_token::Token;

#[cfg(feature = "ast")]
#[doc(inline)]
pub use moxy_ast::{Token, parse, parse_files};
