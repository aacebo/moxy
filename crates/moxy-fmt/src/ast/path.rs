use moxy_ast::args::{
    AngleArguments, AssocConstArgument, AssocTypeArgument, ConstraintArgument, GenericArgument, ParenArguments,
};
use moxy_ast::path::PathArguments;
use moxy_ast::{Lifetime, Path, PathSegment, ReturnType};

use crate::{FmtError, Format, Formatter};

impl Format for Lifetime {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("'")?;
        f.text(&self.ident.text)
    }
}

impl Format for Path {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        if let Some(colon) = self.leading_colon {
            f.text(colon)?;
        }

        for (i, pair) in self.segments.pairs().enumerate() {
            match pair {
                moxy_ast::Pair::Punctuated(seg, sep) => {
                    seg.format(f)?;
                    f.text(sep)?;
                }
                moxy_ast::Pair::End(seg) => {
                    seg.format(f)?;
                }
            }

            let _ = i;
        }

        Ok(())
    }
}

impl Format for PathSegment {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.ident.format(f)?;
        self.args.format(f)
    }
}

impl Format for PathArguments {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::None => Ok(()),
            Self::AngleBracketed(args) => args.format(f),
            Self::Parenthesized(args) => args.format(f),
        }
    }
}

impl Format for AngleArguments {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("<")?;
        self.args.format(f)?;
        f.text(">")
    }
}

impl Format for ParenArguments {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("(")?;
        self.params.format(f)?;
        f.text(")")?;
        self.output.format(f)
    }
}

impl Format for ReturnType {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Default => Ok(()),
            Self::Type(_, ty) => {
                f.text(" ")?;
                f.text("->")?;
                f.text(" ")?;
                ty.format(f)
            }
        }
    }
}

impl Format for GenericArgument {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Lifetime(v) => v.format(f),
            Self::Type(v) => v.format(f),
            Self::Const(v) => v.format(f),
            Self::AssocType(v) => v.format(f),
            Self::AssocConst(v) => v.format(f),
            Self::Constraint(v) => v.format(f),
        }
    }
}

impl Format for AssocTypeArgument {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.ident.format(f)?;

        if let Some(generics) = &self.generics {
            generics.format(f)?;
        }

        f.text(" = ")?;
        self.ty.format(f)
    }
}

impl Format for AssocConstArgument {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.ident.format(f)?;

        if let Some(generics) = &self.generics {
            generics.format(f)?;
        }

        f.text(" = ")?;
        self.expr.format(f)
    }
}

impl Format for ConstraintArgument {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.ident.format(f)?;

        if let Some(generics) = &self.generics {
            generics.format(f)?;
        }

        f.text(": ")?;
        self.bounds.format(f)
    }
}
