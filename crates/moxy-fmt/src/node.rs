pub fn nil() -> FmtNode {
    FmtNode::Nil
}

pub fn text(value: impl std::fmt::Display) -> FmtNode {
    FmtNode::Text(value.to_string())
}

pub fn line(value: impl Into<Line>) -> FmtNode {
    FmtNode::Line(value.into())
}

pub fn concat(value: impl IntoIterator<Item = FmtNode>) -> FmtNode {
    let nodes = value
        .into_iter()
        .filter(|node| !matches!(node, FmtNode::Nil))
        .collect::<Vec<_>>();

    match nodes.len() {
        0 => FmtNode::Nil,
        1 => nodes.into_iter().next().unwrap(),
        _ => FmtNode::Concat(nodes),
    }
}

pub fn group(value: FmtNode) -> FmtNode {
    FmtNode::Group(Box::new(value))
}

pub fn indent(value: FmtNode) -> FmtNode {
    FmtNode::Indent(Box::new(value))
}

pub fn if_break(broken: FmtNode, flat: FmtNode) -> FmtNode {
    FmtNode::IfBreak {
        broken: Box::new(broken),
        flat: Box::new(flat),
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FmtNode {
    #[default]
    Nil,
    Text(String),
    Line(Line),
    Concat(Vec<Self>),
    Group(Box<Self>),
    Indent(Box<Self>),
    IfBreak {
        broken: Box<Self>,
        flat: Box<Self>,
    },
}

impl FmtNode {
    pub fn flat_width(&self) -> Option<usize> {
        match self {
            Self::Nil => Some(0),
            Self::Text(v) => Some(v.len()),
            Self::Line(Line::Space) => Some(1),
            Self::Line(Line::Soft) => Some(0),
            Self::Line(Line::Hard) => None,
            Self::Group(v) => v.flat_width(),
            Self::Indent(v) => v.flat_width(),
            Self::IfBreak { broken: _, flat } => flat.flat_width(),
            Self::Concat(v) => {
                let mut width = 0;

                for item in v {
                    width += item.flat_width()?;
                }

                Some(width)
            }
        }
    }
}

impl super::Format for FmtNode {
    fn format(&self, f: &mut crate::Formatter) -> Result<(), crate::FmtError> {
        f.write_node(self, super::Mode::Broken)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "lowercase")
)]
pub enum Line {
    /// Space when flat, newline when broken.
    Space,

    /// Empty when flat, newline when broken.
    Soft,

    /// Always renders as newline.
    Hard,
}

impl Line {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Space => "space",
            Self::Soft => "soft",
            Self::Hard => "hard",
        }
    }
}

impl std::fmt::Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
