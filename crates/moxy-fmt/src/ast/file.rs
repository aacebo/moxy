use moxy_ast::{File, FilePath, Shebang};

use crate::{FmtError, Format, Formatter};

impl Format for File {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        if let Some(shebang) = &self.shebang {
            shebang.format(f)?;

            if !self.attrs.is_empty() || !self.items.is_empty() {
                f.hard_break()?;
            }
        }

        self.attrs.format(f)?;
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

impl Format for Shebang {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.pound.format(f)?;
        self.bang.format(f)?;
        self.path.format(f)?;

        if let Some(ident) = &self.ident {
            f.text(" ")?;
            ident.format(f)?;
        }

        Ok(())
    }
}

impl Format for FilePath {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        for (slash, ident) in self.iter() {
            slash.format(f)?;
            ident.format(f)?;
        }

        Ok(())
    }
}
