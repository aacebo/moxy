use moxy_ast::MacroCall;
use moxy_token::{Delim, ToTokenStream};

use crate::{Fmt, FmtError, Formatter};

impl Fmt for MacroCall {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.path.fmt(f)?;
        f.text("!")?;

        let (open, close) = match self.delim() {
            Delim::Paren => ("(", ")"),
            Delim::Bracket => ("[", "]"),
            Delim::Brace => ("{", "}"),
            Delim::None => ("", ""),
        };

        f.text(open)?;
        f.text(self.tokens().to_token_stream())?;
        f.text(close)
    }
}
