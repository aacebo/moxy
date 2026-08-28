use moxy_ast::{
    Attribute, Attributes, Meta,
    attr::{
        AttrStyle,
        meta::{MetaArgument, MetaLayout, MetaValue},
    },
};
use moxy_token::Delim;

use crate::{FmtError, Format, Formatter};

impl Format for Attributes {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        for attr in self.iter() {
            attr.format(f)?;
            f.soft_break()?;
        }

        Ok(())
    }
}

impl Format for Attribute {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.style.format(f)?;
        f.text("[")?;
        self.meta.format(f)?;
        f.text("]")
    }
}

impl Format for AttrStyle {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Inner(_, _) => f.text("#!"),
            Self::Outer(_) => f.text("#"),
        }
    }
}

impl Format for Meta {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.path.format(f)?;
        self.content.format(f)
    }
}

impl Format for MetaLayout {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Value(v) => v.format(f),
            Self::List { items } => {
                match items.style {
                    Delim::Paren => f.text("(")?,
                    Delim::Brace => f.text("{")?,
                    Delim::Bracket => f.text("[")?,
                    _ => (),
                };

                items.format(f)?;

                match items.style {
                    Delim::Paren => f.text(")"),
                    Delim::Brace => f.text("}"),
                    Delim::Bracket => f.text("]"),
                    _ => Ok(()),
                }
            }
            Self::Alias { eq: _, value } => {
                f.text(" = ")?;
                value.format(f)
            }
            _ => Ok(()),
        }
    }
}

impl Format for MetaArgument {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Meta(v) => v.format(f),
            Self::Value(v) => v.format(f),
        }
    }
}

impl Format for MetaValue {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Literal(v) => v.format(f),
            Self::Verbatim(v) => {
                for token in v.iter() {
                    f.text(token)?;
                }

                Ok(())
            }
        }
    }
}
