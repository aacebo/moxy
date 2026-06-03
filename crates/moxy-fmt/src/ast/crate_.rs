use moxy_ast::{Crate, TypedParam};

use crate::{Fmt, FmtError, Formatter};

impl Fmt for TypedParam {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.pat.fmt(f)?;
        f.text(": ")?;
        self.ty.fmt(f)
    }
}

impl Fmt for Crate {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        let mut first = true;

        for item in &self.items {
            if !first {
                f.hard_break()?;
                f.hard_break()?;
            }

            item.fmt(f)?;
            first = false;
        }

        Ok(())
    }
}
