use moxy_ast::Pattern;
use moxy_ast::pat::*;

use crate::{Fmt, FmtError, Formatter};

impl Fmt for Pattern {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Wild => f.text("_"),
            Self::Rest => f.text(".."),
            Self::Ident(v) => v.fmt(f),
            Self::Path(v) => v.fmt(f),
            Self::Tuple(v) => v.fmt(f),
            Self::TupleStruct(v) => v.fmt(f),
            Self::Struct(v) => v.fmt(f),
            Self::Slice(v) => v.fmt(f),
            Self::Reference(v) => v.fmt(f),
            Self::Or(v) => v.fmt(f),
            Self::Lit(v) => v.fmt(f),
            Self::Range(v) => v.fmt(f),
            Self::Macro(v) => v.fmt(f),
            Self::Type(v) => v.fmt(f),
            Self::Group(v) => v.fmt(f),
            Self::Paren(v) => v.fmt(f),
            Self::Box(v) => {
                f.text("box ")?;
                v.fmt(f)
            }
            Self::Const(v) => {
                f.text("const ")?;
                v.fmt(f)
            }
        }
    }
}

impl Fmt for PatIdent {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        if self.by_ref.is_some() {
            f.text("ref ")?;
        }

        self.mutability.fmt(f)?;

        if matches!(self.mutability, moxy_ast::Mutability::Mutable(_)) {
            f.text(" ")?;
        }

        self.ident.fmt(f)?;

        if let Some((_, subpat)) = &self.subpat {
            f.text(" @ ")?;
            subpat.fmt(f)?;
        }

        Ok(())
    }
}

impl Fmt for PatPath {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.path.fmt(f)
    }
}

impl Fmt for PatTuple {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("(")?;
        self.elems.fmt(f)?;
        f.text(")")
    }
}

impl Fmt for PatTupleStruct {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.path.fmt(f)?;
        f.text("(")?;
        self.elems.fmt(f)?;
        f.text(")")
    }
}

impl Fmt for PatStruct {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.path.fmt(f)?;
        f.text(" {")?;
        f.indent(|f| {
            for pair in self.fields.pairs() {
                f.hard_break()?;

                match pair {
                    moxy_ast::Pair::Punctuated(field, _) => {
                        field.fmt(f)?;
                        f.text(",")?;
                    }
                    moxy_ast::Pair::End(field) => {
                        field.fmt(f)?;
                        f.text(",")?;
                    }
                }
            }

            if self.rest.is_some() {
                f.hard_break()?;
                f.text("..")?;
            }

            Ok(())
        })?;

        if !self.fields.is_empty() || self.rest.is_some() {
            f.hard_break()?;
        }

        f.text("}")
    }
}

impl Fmt for PatField {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        if self.shorthand {
            self.pat.fmt(f)
        } else {
            self.member.fmt(f)?;
            f.text(": ")?;
            self.pat.fmt(f)
        }
    }
}

impl Fmt for PatSlice {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("[")?;
        self.elems.fmt(f)?;
        f.text("]")
    }
}

impl Fmt for PatReference {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("&")?;
        self.mutability.fmt(f)?;

        if matches!(self.mutability, moxy_ast::Mutability::Mutable(_)) {
            f.text(" ")?;
        }

        self.pat.fmt(f)
    }
}

impl Fmt for PatOr {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        for pair in self.cases.pairs() {
            match pair {
                moxy_ast::Pair::Punctuated(pat, _) => {
                    pat.fmt(f)?;
                    f.text(" | ")?;
                }
                moxy_ast::Pair::End(pat) => {
                    pat.fmt(f)?;
                }
            }
        }

        Ok(())
    }
}

impl Fmt for PatLit {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.expr.fmt(f)
    }
}

impl Fmt for PatRange {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        if let Some(start) = &self.start {
            start.fmt(f)?;
        }

        self.limits.fmt(f)?;

        if let Some(end) = &self.end {
            end.fmt(f)?;
        }

        Ok(())
    }
}

impl Fmt for PatType {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.pat.fmt(f)?;
        f.text(": ")?;
        self.ty.fmt(f)
    }
}

impl Fmt for PatGroup {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.pat.fmt(f)
    }
}

impl Fmt for PatParen {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("(")?;
        self.pat.fmt(f)?;
        f.text(")")
    }
}
