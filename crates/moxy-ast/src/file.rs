use moxy_token::{Cursor as TextCursor, Span, Spanner, ToTokenStream, ToTokens, TokenStream};

use crate::*;

#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct File {
    /// The shebang line that starts the text of the file, as
    /// `#!/usr/bin/env -S cargo`, without its line break. Only
    /// [`File::from_source_file`], and so `parse_file!` and `parse_files!`,
    /// read it, since only the text says where the line ends: a token stream
    /// holds no line breaks, and parsing one never gives a shebang. The
    /// formatter writes it back on the first line, and returns an error for a
    /// value that would read back otherwise: one that does not start with
    /// `#!`, spans more than one line, or opens an inner attribute, as `#![`.
    /// Whether a line reads back as a shebang also depends on the text after
    /// it: `#!` with a `[` on the next line opens an inner attribute. Emitted
    /// tokens leave it out, as rustc's tokens of a file do.
    pub shebang: Option<String>,
    pub attrs: Attributes,
    pub items: Vec<Item>,
}

impl File {
    /// Parses the text of a source file, which may start with a byte order
    /// mark and a shebang line, as rustc reads one: the shebang line is kept in
    /// [`File::shebang`] and the rest is lexed with
    /// [`TokenStream::from_source_file`].
    pub fn from_source_file(text: String) -> Result<Self, ParseError> {
        let body = text.strip_prefix('\u{feff}').unwrap_or(&text);
        let end = TextCursor::new(body, 0).skip_source_start().offset() as usize;
        // A `#!` that opens an inner attribute is skipped by nothing
        let shebang = (end > 0 && body.starts_with("#!")).then(|| body[..end].trim_end_matches('\r').to_owned());
        let mut file: Self = crate::__parse_owned(&TokenStream::from_source_file(text)?)?;
        file.shebang = shebang;
        Ok(file)
    }
}

impl Attributed for File {
    fn attrs(&self) -> &[Attribute] {
        &self.attrs
    }
}

impl Spanner for File {
    fn span(&self) -> Span {
        self.to_token_stream().span()
    }
}

impl ToTokens for File {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.attrs.to_tokens(tokens);

        for item in &self.items {
            item.to_tokens(tokens);
        }
    }
}

impl Parse for File {
    fn peek(cursor: Cursor<'_>) -> bool {
        Attributes::peek_inner(cursor) || Item::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        // Only inner attributes belong to the file; outer ones belong to its first item
        let attrs = Attributes::parse_inner(parser)?;

        Ok(Self {
            shebang: None,
            attrs,
            items: <_ as Parse>::parse(parser)?,
        })
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        let cursor = Attributes::skip_inner(cursor)?;
        Vec::<Item>::skip(cursor)
    }
}
