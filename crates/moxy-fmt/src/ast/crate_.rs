use moxy_ast::{Crate, TypedParam};

use crate::{FmtError, Format, Formatter};

impl Format for TypedParam {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.pat.format(f)?;
        f.text(": ")?;
        self.ty.format(f)
    }
}

impl Format for Crate {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        let mut first = true;

        for item in &self.items {
            if !first {
                f.hard_break()?;
                f.hard_break()?;
            }

            item.format(f)?;
            first = false;
        }

        Ok(())
    }
}
