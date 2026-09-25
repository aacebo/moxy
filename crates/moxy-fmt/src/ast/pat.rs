use moxy_ast::*;

use crate::{FmtError, Format, Formatter};

impl Format for Pattern {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Wild(v) => v.format(f),
            Self::Rest(v) => v.format(f),
            Self::Ident(v) => v.format(f),
            Self::Path(v) => v.format(f),
            Self::Tuple(v) => v.format(f),
            Self::TupleStruct(v) => v.format(f),
            Self::Struct(v) => v.format(f),
            Self::Slice(v) => v.format(f),
            Self::Reference(v) => v.format(f),
            Self::Or(v) => v.format(f),
            Self::Lit(v) => v.format(f),
            Self::Range(v) => v.format(f),
            Self::Macro(v) => v.format(f),
            Self::Type(v) => v.format(f),
            Self::Group(v) => v.format(f),
            Self::Paren(v) => v.format(f),
            Self::Box(v) => v.format(f),
            Self::Const(v) => v.format(f),
            _ => Err(FmtError::unsupported("unsupported pattern variant")),
        }
    }
}

impl Format for PatIdent {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.attrs.format(f)?;

        if self.by_ref.is_some() {
            f.text("ref ")?;
        }

        self.mutability.format(f)?;

        if self.mutability.is_some() {
            f.text(" ")?;
        }

        self.ident.format(f)?;

        if let Some((_, subpat)) = &self.subpat {
            f.text(" @ ")?;
            subpat.format(f)?;
        }

        Ok(())
    }
}

impl Format for PatPath {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.attrs.format(f)?;
        self.path.format(f)
    }
}

impl Format for PatTuple {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.attrs.format(f)?;
        f.text("(")?;
        self.elems.inner.format(f)?;
        f.text(")")
    }
}

impl Format for PatTupleStruct {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.attrs.format(f)?;
        self.path.format(f)?;
        f.text("(")?;
        self.elems.inner.format(f)?;
        f.text(")")
    }
}

impl Format for PatStruct {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.attrs.format(f)?;
        self.path.format(f)?;
        f.text(" {")?;
        f.indent(|f| {
            for pair in self.body.inner.fields.pairs() {
                f.hard_break()?;

                match pair {
                    moxy_ast::Pair::Punctuated(field, _) => {
                        field.format(f)?;
                        f.text(",")?;
                    }
                    moxy_ast::Pair::End(field) => {
                        field.format(f)?;
                        f.text(",")?;
                    }
                }
            }

            if self.body.inner.dotdot.is_some() {
                f.hard_break()?;
                f.text("..")?;
            }

            Ok(())
        })?;

        if !self.body.inner.fields.is_empty() || self.body.inner.dotdot.is_some() {
            f.hard_break()?;
        }

        f.text("}")
    }
}

impl Format for PatField {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.attrs.format(f)?;

        if self.is_shorthand() {
            self.pat.format(f)
        } else {
            self.member.format(f)?;
            f.text(": ")?;
            self.pat.format(f)
        }
    }
}

impl Format for PatSlice {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.attrs.format(f)?;
        f.text("[")?;
        self.elems.inner.format(f)?;
        f.text("]")
    }
}

impl Format for PatReference {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.attrs.format(f)?;
        f.text("&")?;
        self.mutability.format(f)?;

        if self.mutability.is_some() {
            f.text(" ")?;
        }

        self.pat.format(f)
    }
}

impl Format for PatOr {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.attrs.format(f)?;

        for pair in self.cases.pairs() {
            match pair {
                moxy_ast::Pair::Punctuated(pat, _) => {
                    pat.format(f)?;
                    f.text(" | ")?;
                }
                moxy_ast::Pair::End(pat) => {
                    pat.format(f)?;
                }
            }
        }

        Ok(())
    }
}

impl Format for PatLit {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.attrs.format(f)?;
        self.lit.format(f)
    }
}

impl Format for PatRange {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.attrs.format(f)?;

        if let Some(start) = &self.start {
            start.format(f)?;
        }

        self.limits.format(f)?;

        if let Some(end) = &self.end {
            end.format(f)?;
        }

        Ok(())
    }
}

impl Format for PatType {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.attrs.format(f)?;
        self.pat.format(f)?;
        f.text(": ")?;
        self.ty.format(f)
    }
}

impl Format for PatGroup {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.attrs.format(f)?;
        self.pat.format(f)
    }
}

impl Format for PatParen {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.attrs.format(f)?;
        f.text("(")?;
        self.content.inner.format(f)?;
        f.text(")")
    }
}

impl Format for PatBox {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.attrs.format(f)?;
        self.keyword.format(f)?;
        f.space()?;
        self.pattern.format(f)
    }
}

impl Format for PatConst {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.attrs.format(f)?;
        self.keyword.format(f)?;
        f.space()?;
        self.block.format(f)
    }
}

impl Format for PatWild {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.attrs.format(f)?;
        self.token.format(f)
    }
}

impl Format for PatRest {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.attrs.format(f)?;
        self.token.format(f)
    }
}

impl Format for PatMacro {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.attrs.format(f)?;
        self.call.format(f)
    }
}
