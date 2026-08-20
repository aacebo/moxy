use moxy_token::Lit;

use crate::{FmtError, Format, Formatter};

impl Format for Lit {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text(self.repr())
    }
}
