use moxy_ast::File;
use moxy_token::Cursor as TextCursor;

use crate::{FmtError, Format, Formatter};

impl Format for File {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        if let Some(shebang) = &self.shebang {
            let line = TextCursor::new(shebang, 0);

            // Only a line that the text entry reads back as a shebang
            if shebang.contains('\n') || !shebang.starts_with("#!") || line.skip_source_start().offset() == 0 {
                return Err(FmtError::unsupported(format!("`{shebang}` is not a shebang line")));
            }

            f.text(shebang)?;

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
