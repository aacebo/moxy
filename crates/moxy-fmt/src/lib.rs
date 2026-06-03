pub mod ast;
mod config;
mod error;
mod node;

use std::fmt::Write;

pub use config::*;
pub use error::*;
pub use node::*;

pub fn fmt<T: Fmt>(value: &T, config: &FmtConfig) -> Result<String, FmtError> {
    let mut f = Formatter::new(config);
    f.write(value)?;
    Ok(f.output)
}

pub trait Fmt {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError>;
}

pub struct Formatter<'a> {
    config: &'a FmtConfig,
    buffer: Vec<FmtNode>,
    output: String,
    depth: usize,
    column: usize,
}

impl<'a> Formatter<'a> {
    pub fn new(config: &'a FmtConfig) -> Self {
        Self {
            config,
            buffer: Vec::new(),
            output: String::new(),
            depth: 0,
            column: 0,
        }
    }

    pub fn config(&self) -> &FmtConfig {
        self.config
    }

    pub fn depth(&self) -> usize {
        self.depth
    }

    pub fn column(&self) -> usize {
        self.column
    }
}

impl<'a> Formatter<'a> {
    pub fn text(&mut self, value: impl std::fmt::Display) -> Result<(), FmtError> {
        self.buffer.push(text(value));
        Ok(())
    }

    pub fn space(&mut self) -> Result<(), FmtError> {
        self.buffer.push(line(Line::Space));
        Ok(())
    }

    pub fn soft_break(&mut self) -> Result<(), FmtError> {
        self.buffer.push(line(Line::Soft));
        Ok(())
    }

    pub fn hard_break(&mut self) -> Result<(), FmtError> {
        self.buffer.push(line(Line::Hard));
        Ok(())
    }

    pub fn group(&mut self, f: impl FnOnce(&mut Self) -> Result<(), FmtError>) -> Result<(), FmtError> {
        let start = self.buffer.len();
        f(self)?;
        let nodes = self.buffer.drain(start..).collect::<Vec<_>>();
        self.buffer.push(group(concat(nodes)));
        Ok(())
    }

    pub fn indent(&mut self, f: impl FnOnce(&mut Self) -> Result<(), FmtError>) -> Result<(), FmtError> {
        let start = self.buffer.len();
        f(self)?;
        let nodes = self.buffer.drain(start..).collect::<Vec<_>>();
        self.buffer.push(indent(concat(nodes)));
        Ok(())
    }
}

impl<'a> Formatter<'a> {
    pub fn write<T: Fmt>(&mut self, value: &T) -> Result<(), FmtError> {
        value.fmt(self)?;
        let nodes: Vec<_> = self.buffer.drain(0..).collect();

        for node in nodes {
            self.write_node(&node, Mode::Broken)?;
        }

        Ok(())
    }

    pub fn write_all<T: Fmt>(&mut self, iter: impl AsRef<[T]>) -> Result<(), FmtError> {
        for item in iter.as_ref() {
            self.write(item)?;
        }

        Ok(())
    }

    pub fn write_node(&mut self, node: &FmtNode, mode: Mode) -> Result<(), FmtError> {
        match node {
            FmtNode::Text(text) => {
                self.output.push_str(text);
                self.column += text.len();
            }
            FmtNode::Line(line) => {
                self.write_line(*line, mode)?;
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
                self.depth += 1;
                self.write_node(node, mode)?;
                self.depth -= 1;
            }
            FmtNode::IfBreak { broken, flat } => match mode {
                Mode::Flat => self.write_node(flat, mode)?,
                Mode::Broken => self.write_node(broken, mode)?,
            },
            _ => {}
        }

        Ok(())
    }

    pub fn write_line(&mut self, line: Line, mode: Mode) -> Result<(), FmtError> {
        match (line, mode) {
            (Line::Space, Mode::Flat) => {
                self.output.push(' ');
                self.column += 1;
            }
            (Line::Hard, _) => self.write_newline()?,
            (Line::Space | Line::Soft, Mode::Broken) => self.write_newline()?,
            _ => {}
        };

        Ok(())
    }

    pub fn write_newline(&mut self) -> Result<(), FmtError> {
        write!(
            &mut self.output,
            "{}{}",
            self.config.newline,
            self.config.indent.to_string().repeat(self.depth)
        )?;

        self.column = self.depth * self.config.indent.spaces();
        Ok(())
    }
}

impl<T: Fmt, P: std::fmt::Display> Fmt for moxy_ast::Punctuated<T, P> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.group(|f| {
            for pair in self.pairs() {
                match pair {
                    moxy_ast::Pair::Punctuated(t, p) => {
                        t.fmt(f)?;
                        f.text(p)?;
                        f.space()?;
                    }
                    moxy_ast::Pair::End(t) => {
                        t.fmt(f)?;
                    }
                }
            }

            Ok(())
        })
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
