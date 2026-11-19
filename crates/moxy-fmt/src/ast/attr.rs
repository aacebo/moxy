use moxy_ast::{AttrStyle, Attribute, Attributes, Meta, MetaContent};
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
        let mut previous: Option<String> = None;

        for token in self.iter() {
            let text = token.to_string();

            if let Some(previous) = &previous
                && joins(previous, &text)
            {
                f.text(" ")?;
            }

            f.text(&text)?;

            if token.is_punct_comma() {
                f.text(" ")?;
                previous = None;
            } else {
                previous = Some(text);
            }
        }

        Ok(())
    }
}

// Whether two tokens written without a space between them lex as other
// tokens, as `pub b` would as `pubb`, `/ /` as a comment, and `b 'a` as no
// valid tokens
fn joins(left: &str, right: &str) -> bool {
    fn texts(source: &str) -> Option<Vec<String>> {
        let tokens = source.parse::<TokenStream>().ok()?;
        Some(tokens.iter().map(ToString::to_string).collect())
    }

    texts(&format!("{left}{right}")) != texts(&format!("{left} {right}"))
}
