use moxy_ast::args::{AngleArgs, AssocConstArg, AssocTypeArg, ConstraintArg, GenericArgument};
use moxy_ast::path::{ParenthesizedArgs, PathArguments};
use moxy_ast::{Lifetime, Path, PathSegment, ReturnType};

use crate::{Fmt, FmtError, Formatter};

impl Fmt for Lifetime {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("'")?;
        f.text(&self.ident.text)
    }
}

impl Fmt for Path {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        if self.leading_colon {
            f.text("::")?;
        }

        for (i, pair) in self.segments.pairs().enumerate() {
            match pair {
                moxy_ast::Pair::Punctuated(seg, _) => {
                    seg.fmt(f)?;
                    f.text("::")?;
                }
                moxy_ast::Pair::End(seg) => {
                    seg.fmt(f)?;
                }
            }

            let _ = i;
        }

        Ok(())
    }
}

impl Fmt for PathSegment {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.ident.fmt(f)?;
        self.args.fmt(f)
    }
}

impl Fmt for PathArguments {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::None => Ok(()),
            Self::AngleBracketed(args) => args.fmt(f),
            Self::Parenthesized(args) => args.fmt(f),
        }
    }
}

impl Fmt for AngleArgs {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("<")?;
        self.args.fmt(f)?;
        f.text(">")
    }
}

impl Fmt for ParenthesizedArgs {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("(")?;
        self.inputs.fmt(f)?;
        f.text(")")?;
        self.output.fmt(f)
    }
}

impl Fmt for ReturnType {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Default => Ok(()),
            Self::Type(ty) => {
                f.space()?;
                f.text("->")?;
                f.space()?;
                ty.fmt(f)
            }
        }
    }
}

impl Fmt for GenericArgument {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Lifetime(v) => v.fmt(f),
            Self::Type(v) => v.fmt(f),
            Self::Const(v) => v.fmt(f),
            Self::AssocType(v) => v.fmt(f),
            Self::AssocConst(v) => v.fmt(f),
            Self::Constraint(v) => v.fmt(f),
        }
    }
}

impl Fmt for AssocTypeArg {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.ident.fmt(f)?;

        if let Some(generics) = &self.generics {
            generics.fmt(f)?;
        }

        f.text(" = ")?;
        self.ty.fmt(f)
    }
}

impl Fmt for AssocConstArg {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.ident.fmt(f)?;

        if let Some(generics) = &self.generics {
            generics.fmt(f)?;
        }

        f.text(" = ")?;
        self.expr.fmt(f)
    }
}

impl Fmt for ConstraintArg {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.ident.fmt(f)?;

        if let Some(generics) = &self.generics {
            generics.fmt(f)?;
        }

        f.text(": ")?;
        self.bounds.fmt(f)
    }
}
