mod config;
mod error;
mod node;
mod rule;

pub use config::*;
pub use error::*;
pub use node::*;
pub use rule::*;

pub trait Fmt {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError>;
}

pub struct Formatter<'a> {
    config: &'a FormatConfig,
    buffer: Vec<FmtNode>,
    output: String,
    indent: usize,
    column: usize,
}

impl<'a> Formatter<'a> {
    pub fn new(config: &'a FormatConfig) -> Self {
        Self {
            config,
            buffer: Vec::new(),
            output: String::new(),
            indent: 0,
            column: 0,
        }
    }

    pub fn config(&self) -> &FormatConfig {
        &self.config
    }

    pub fn indent(&self) -> usize {
        self.indent
    }

    pub fn column(&self) -> usize {
        self.column
    }
}

impl<'a> Formatter<'a> {
    pub fn write<T: Fmt>(&mut self, value: &T) -> Result<(), FormatError> {
        value.fmt(self)?;
        let nodes: Vec<_> = self.buffer.drain(0..).collect();

        for node in nodes {
            self.write_node(&node, Mode::Broken)?;
        }

        Ok(())
    }

    pub fn write_all<T: Fmt>(&mut self, iter: impl AsRef<[T]>) -> Result<(), FormatError> {
        for item in iter.as_ref() {
            self.write(item)?;
        }

        Ok(())
    }

    pub fn write_node(&mut self, node: &FmtNode, mode: Mode) -> Result<(), FormatError> {
        match node {
            FmtNode::Text(text) => {
                self.output.push_str(&text);
                self.column += text.len();
            }
            FmtNode::Line(line) => {
                self.write_line(*line, mode);
            }
            FmtNode::Concat(nodes) => {
                for node in nodes {
                    self.write_node(node, mode)?;
                }
            }
            FmtNode::Group(node) => {
                let remaining = self.config.max_width.saturating_sub(self.column);

                if node.flat_width().is_some_and(|width| width <= remaining) {
                    self.write_node(node, Mode::Flat)?;
                } else {
                    self.write_node(node, Mode::Broken)?;
                }
            }
            FmtNode::Indent(node) => {
                self.indent += self.config.indent.spaces();
                self.write_node(node, mode)?;
                self.indent -= self.config.indent.spaces();
            }
            FmtNode::IfBreak { broken, flat } => match mode {
                Mode::Flat => self.write_node(flat, mode)?,
                Mode::Broken => self.write_node(broken, mode)?,
            },
            _ => {}
        }

        Ok(())
    }

    pub fn write_line(&mut self, line: Line, mode: Mode) {
        match (line, mode) {
            (Line::Space, Mode::Flat) => {
                self.output.push(' ');
                self.column += 1;
            }
            (Line::Hard, _) => self.write_newline(),
            (Line::Space | Line::Soft, Mode::Broken) => self.write_newline(),
            _ => {}
        }
    }

    pub fn write_newline(&mut self) {
        self.output.push_str(&self.config.newline.to_string());

        for _ in 0..self.indent {
            self.output.push(' ');
        }

        self.column = self.indent;
    }
}

impl<'a> Extend<FmtNode> for Formatter<'a> {
    fn extend<T: IntoIterator<Item = FmtNode>>(&mut self, iter: T) {
        self.buffer.extend(iter);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Mode {
    Flat,
    Broken,
}

impl Mode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Flat => "flat",
            Self::Broken => "broken",
        }
    }
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
