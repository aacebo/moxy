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

// The attributes of a parameter, a generic parameter, a tuple field, an
// expression, or a pattern, which stay on its line with a space after each, as
// in `fn f(#[a] x: u8)`
pub(crate) fn inline_attrs(attrs: &Attributes, f: &mut Formatter) -> Result<(), FmtError> {
    if std::mem::take(&mut f.stmt_expr_attrs) {
        return attrs.format(f);
    }

    for attr in attrs.iter() {
        attr.format(f)?;
        f.text(" ")?;
    }

    Ok(())
}

// The inner attributes that open a body, each on its own line
pub(crate) fn inner_attrs(attrs: &Attributes, f: &mut Formatter) -> Result<(), FmtError> {
    for attr in attrs.iter() {
        f.hard_break()?;
        attr.format(f)?;
    }

    Ok(())
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

                f.check_tokens(&v.tokens)?;
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
        f.check_tokens(self)?;
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
// tokens, as `pub b` would as `pubb`, `/ /` as a comment, and `b 'a` as a
// byte literal. A lifetime's quote joins its name and takes a space only after
// a word. A token that does not lex back to itself, as `$crate` from a
// `macro_rules!` fragment, takes a space only where one changes how the two
// lex.
fn joins(left: &str, right: &str) -> bool {
    fn texts(source: &str) -> Option<Vec<String>> {
        let tokens = source.parse::<TokenStream>().ok()?;
        Some(tokens.iter().map(ToString::to_string).collect())
    }

    if left == "'" {
        return false;
    }

    if right == "'" {
        return left.ends_with(|ch: char| ch.is_alphanumeric() || ch == '_');
    }

    let together = texts(&format!("{left}{right}"));

    if texts(left).as_deref() != Some(&[left.to_string()]) || texts(right).as_deref() != Some(&[right.to_string()]) {
        return together != texts(&format!("{left} {right}"));
    }

    together.as_deref() != Some(&[left.to_string(), right.to_string()])
}
