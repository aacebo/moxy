mod ast;
mod config;
mod error;
mod node;

use std::fmt::Write;

#[doc(inline)]
pub use config::*;
#[doc(inline)]
pub use error::*;
#[doc(inline)]
pub use node::*;

/// Format a value that implements [`Fmt`] into a `String`.
///
/// Accepts an optional [`FmtConfig`] as a second argument. When omitted,
/// [`FmtConfig::default`] is used.
///
/// Returns `Result<String, FmtError>`.
///
/// The intended pipeline is `template!` → `parse!` → `fmt!`: generate tokens,
/// parse them into an AST node, then format it.
///
/// # Examples
///
/// ```ignore
/// use moxy_ast::Item;
///
/// let name = "Greeter";
/// let tokens = moxy::template! {
///     pub struct {{ name }} {
///         message: String,
///     }
/// };
///
/// let item = moxy::parse!(tokens as Item)?;
/// let output = moxy::fmt!(&item)?;
///
/// assert_eq!(output, "pub struct Greeter {\n    message: String,\n}");
/// ```
#[macro_export]
macro_rules! fmt {
    ($value:expr, $config:expr) => {{
        let mut f = $crate::Formatter::new($config);

        match f.write($value) {
            Err(err) => Err(err),
            Ok(_) => Ok(f.done()),
        }
    }};
    ($value:expr) => {{
        let mut f = $crate::Formatter::new($crate::FmtConfig::default());

        match f.write($value) {
            Err(err) => Err(err),
            Ok(_) => Ok(f.done()),
        }
    }};
}

pub trait Fmt {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError>;
}

pub struct Formatter {
    config: FmtConfig,
    buffer: Vec<FmtNode>,
    output: String,
    depth: usize,
    column: usize,
}

impl Formatter {
    pub fn new(config: FmtConfig) -> Self {
        Self {
            config,
            buffer: Vec::new(),
            output: String::new(),
            depth: 0,
            column: 0,
        }
    }

    pub fn config(&self) -> &FmtConfig {
        &self.config
    }

    pub fn depth(&self) -> usize {
        self.depth
    }

    pub fn column(&self) -> usize {
        self.column
    }

    pub fn done(self) -> String {
        self.output
    }
}

impl Formatter {
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

impl Formatter {
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
                        let sep = p.to_string();
                        if sep.trim() == "+" {
                            f.text(" ")?;
                            f.text(sep)?;
                        } else {
                            f.text(sep)?;
                        }
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

impl Extend<FmtNode> for Formatter {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn run(node: FmtNode) -> String {
        let config = FmtConfig::default().with_newline(NewlineStyle::Unix);
        let node_clone = node.clone();
        fmt!(&node_clone, config).unwrap()
    }

    fn run_with(node: FmtNode, config: FmtConfig) -> String {
        let node_clone = node.clone();
        fmt!(&node_clone, config.with_newline(NewlineStyle::Unix)).unwrap()
    }

    #[test]
    fn text_node() {
        assert_eq!(run(text("hello")), "hello");
    }

    #[test]
    fn group_fits_flat() {
        let node = group(concat([text("a"), line(Line::Space), text("b")]));
        assert_eq!(run(node), "a b");
    }

    #[test]
    fn group_exceeds_width_breaks() {
        let node = group(concat([text("hello"), line(Line::Space), text("world")]));
        let config = FmtConfig::default().with_max_width(8);
        assert_eq!(run_with(node, config), "hello\nworld");
    }

    #[test]
    fn hard_break_always_breaks_in_group() {
        let node = group(concat([text("a"), line(Line::Hard), text("b")]));
        assert_eq!(run(node), "a\nb");
    }

    #[test]
    fn soft_break_nothing_when_flat() {
        let node = group(concat([text("a"), line(Line::Soft), text("b")]));
        assert_eq!(run(node), "ab");
    }

    #[test]
    fn soft_break_newline_when_broken() {
        let node = group(concat([text("looooooong"), line(Line::Soft), text("word")]));
        let config = FmtConfig::default().with_max_width(8);
        assert_eq!(run_with(node, config), "looooooong\nword");
    }

    #[test]
    fn indent_emits_tab() {
        let node = concat([
            text("{"),
            indent(concat([line(Line::Hard), text("x")])),
            line(Line::Hard),
            text("}"),
        ]);
        let config = FmtConfig::default().with_indent(Indent::Tab(1));
        assert_eq!(run_with(node, config), "{\n\tx\n}");
    }

    #[test]
    fn indent_emits_spaces() {
        let node = concat([
            text("{"),
            indent(concat([line(Line::Hard), text("x")])),
            line(Line::Hard),
            text("}"),
        ]);
        let config = FmtConfig::default().with_indent(Indent::Space(4));
        assert_eq!(run_with(node, config), "{\n    x\n}");
    }

    #[test]
    fn if_break_flat_branch_in_fitting_group() {
        let node = group(concat([text("a"), if_break(text("broken"), text("flat"))]));
        assert_eq!(run(node), "aflat");
    }

    #[test]
    fn if_break_broken_branch_when_group_breaks() {
        let node = group(concat([
            text("loooooong"),
            line(Line::Space),
            if_break(text("broken"), text("flat")),
        ]));
        let config = FmtConfig::default().with_max_width(8);
        assert_eq!(run_with(node, config), "loooooong\nbroken");
    }

    #[test]
    fn nested_indent() {
        let node = concat([
            text("{"),
            indent(concat([
                line(Line::Hard),
                text("{"),
                indent(concat([line(Line::Hard), text("x")])),
                line(Line::Hard),
                text("}"),
            ])),
            line(Line::Hard),
            text("}"),
        ]);
        let config = FmtConfig::default().with_indent(Indent::Tab(1));
        assert_eq!(run_with(node, config), "{\n\t{\n\t\tx\n\t}\n}");
    }

    #[test]
    fn fmt_fn_returns_string() {
        let node = text("hello");
        assert_eq!(fmt!(&node).unwrap(), "hello");
    }
}
