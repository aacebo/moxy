use moxy_ast::{Attribute, Attributes, Meta, MetaContent, attr::AttrStyle};
use moxy_token::{Delim, TokenStream};

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

impl Format for MetaContent {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::List(v) => {
                match v.delim {
                    Delim::Paren => f.text("(")?,
                    Delim::Brace => f.text("{")?,
                    Delim::Bracket => f.text("[")?,
                    _ => (),
                };

                v.tokens.format(f)?;

                match v.delim {
                    Delim::Paren => f.text(")"),
                    Delim::Brace => f.text("}"),
                    Delim::Bracket => f.text("]"),
                    _ => Ok(()),
                }
            }
            Self::Expr { eq: _, expr } => {
                f.text(" = ")?;
                expr.format(f)
            }
            _ => Ok(()),
        }
    }
}

impl Format for TokenStream {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        for token in self.iter() {
            f.text(token)?;

            if token.is_punct_comma() {
                f.text(" ")?;
            }
        }

        Ok(())
    }
}
