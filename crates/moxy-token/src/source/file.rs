use std::path::PathBuf;

use super::Location;
use crate::span::fallback::Span;

/// Source text with a stable identifier for span resolution.
///
/// Positions count bytes, as the lexer's cursor does, from the start of the
/// source map's position space.
#[derive(Debug)]
pub struct Source {
    /// the filepath if this source belongs to a file
    /// on disk.
    path: Option<PathBuf>,

    /// raw source text
    text: String,

    /// the positions the text takes in the source map
    span: Span,

    /// line start offsets, in bytes
    lines: Vec<usize>,
}

impl Source {
    pub(crate) fn new(generation: std::num::NonZeroU32, start: usize, src: impl Into<String>) -> Self {
        let text = src.into();
        let lines = std::iter::once(0)
            .chain(text.match_indices('\n').map(|(i, _)| i + 1))
            .collect();

        Self {
            path: None,
            span: Span::in_generation(generation, start as u32, (start + text.len()) as u32),
            text,
            lines,
        }
    }

    pub fn from_file(path: impl Into<PathBuf>) -> std::io::Result<Self> {
        let path = path.into();
        let src = std::fs::read_to_string(&path)?;
        let mut src = Self::new(Span::NO_SOURCE, 0, src);

        src.path = Some(path);
        Ok(src)
    }

    pub fn path(&self) -> Option<&PathBuf> {
        self.path.as_ref()
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn span(&self) -> Span {
        self.span
    }

    /// Resolves the given span into a byte index range.
    pub fn range(&self, span: Span) -> std::ops::Range<usize> {
        let r = span.byte_range();
        self.byte(r.start)..self.byte(r.end)
    }

    /// Gets a sub span of source text from the file.
    pub fn slice(&self, span: Span) -> String {
        self.text[self.range(span)].to_owned()
    }

    /// Resolves a position within this file into a 0-based `Location`: the
    /// byte index in the text, the line, and the column in characters.
    pub fn location(&self, i: usize) -> Location {
        let index = self.byte(i);
        let line = match self.lines.binary_search(&index) {
            Err(next) => next - 1,
            Ok(line) => line,
        };
        let line_start = self.lines[line];
        let column = self.text.get(line_start..index).map_or(index - line_start, |text| {
            // A byte order mark starts the text and takes no column
            let text = if line == 0 {
                text.strip_prefix('\u{feff}').unwrap_or(text)
            } else {
                text
            };
            text.chars().count()
        });

        Location::new(index, line, column)
    }

    /// Returns the byte index in the text of a position within this file.
    pub fn byte(&self, i: usize) -> usize {
        i - self.span.byte_range().start
    }
}

impl Default for Source {
    fn default() -> Self {
        Self {
            path: None,
            text: String::default(),
            span: Span::default(),
            lines: vec![0],
        }
    }
}
