#[cfg(feature = "ast")]
#[doc(inline)]
pub use moxy_ast as ast;
#[cfg(feature = "diagnostic")]
#[doc(inline)]
pub use moxy_diagnostic as diagnostic;
#[cfg(feature = "diagnostic")]
#[doc(inline)]
pub use moxy_diagnostic::{error, help, note, warn};
#[cfg(feature = "fmt")]
#[doc(inline)]
pub use moxy_fmt as fmt;
#[cfg(feature = "macros")]
#[doc(inline)]
pub use moxy_macros as macros;
#[cfg(feature = "template")]
#[doc(inline)]
pub use moxy_template as template;
#[cfg(feature = "template")]
#[doc(inline)]
pub use moxy_template::template;
#[cfg(feature = "test")]
#[doc(inline)]
pub use moxy_test as test;
#[cfg(feature = "token")]
#[doc(inline)]
pub use moxy_token as token;
#[cfg(feature = "token")]
#[doc(inline)]
pub use moxy_token::{Token, parse};
