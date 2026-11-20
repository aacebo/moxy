use moxy_ast::MacroCall;
use moxy_token::Delim;

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
        f.check_tokens(self.tokens())?;
        f.text(self.tokens())?;
        f.text(close)
    }
}
