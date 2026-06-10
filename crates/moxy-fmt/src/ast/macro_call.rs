use moxy_ast::MacroCall;
use moxy_token::{Delim, ToTokenStream};

use crate::{FmtError, Format, Formatter};

impl Format for MacroCall {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.path.format(f)?;
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
