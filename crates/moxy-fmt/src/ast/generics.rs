use moxy_ast::generics::{
    ConstParam, GenericParam, LifetimeParam, LifetimePredicate, TraitBound, TypeBound, TypeParam, TypePredicate, UseBound,
};
use moxy_ast::{BoundLifetimes, Generics, TraitRef, WhereClause, WherePredicate};

use crate::{FmtError, Format, Formatter};

impl Format for BoundLifetimes {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("for<")?;
        self.params.format(f)?;
        f.text(">")
    }
}

impl Format for Generics {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        if self.params.is_empty() {
            return Ok(());
        }

        f.text("<")?;
        self.params.format(f)?;
        f.text(">")?;

        if let Some(where_clause) = &self.where_clause {
            where_clause.format(f)?;
        }

        Ok(())
    }
}

impl Format for WhereClause {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.hard_break()?;
        f.text("where")?;

        for pair in self.predicates.pairs() {
            f.hard_break()?;
            f.indent(|f| match pair {
                moxy_ast::Pair::Punctuated(pred, _) => {
                    pred.format(f)?;
                    f.text(",")
                }
                moxy_ast::Pair::End(pred) => pred.format(f),
            })?;
        }

        Ok(())
    }
}

impl Format for WherePredicate {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Lifetime(v) => v.format(f),
            Self::Type(v) => v.format(f),
        }
    }
}

impl Format for LifetimePredicate {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.lifetime.format(f)?;
        f.text(": ")?;
        self.bounds.format(f)
    }
}

impl Format for TypePredicate {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        if let Some(lifetimes) = &self.lifetimes {
            lifetimes.format(f)?;
            f.text(" ")?;
        }

        self.bounded_ty.format(f)?;
        f.text(": ")?;
        self.bounds.format(f)
    }
}

impl Format for GenericParam {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Lifetime(v) => v.format(f),
            Self::Type(v) => v.format(f),
            Self::Const(v) => v.format(f),
        }
    }
}

impl Format for LifetimeParam {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.lifetime.format(f)?;

        if !self.bounds.is_empty() {
            f.text(": ")?;
            self.bounds.format(f)?;
        }

        Ok(())
    }
}

impl Format for TypeParam {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.ident.format(f)?;

        if !self.bounds.is_empty() {
            f.text(": ")?;
            self.bounds.format(f)?;
        }

        if let Some(default) = &self.default {
            f.text(" = ")?;
            default.format(f)?;
        }

        Ok(())
    }
}

impl Format for ConstParam {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("const ")?;
        self.ident.format(f)?;
        f.text(": ")?;
        self.ty.format(f)?;

        if let Some(default) = &self.default {
            f.text(" = ")?;
            default.format(f)?;
        }

        Ok(())
    }
}

impl Format for TypeBound {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Trait(v) => v.format(f),
            Self::Lifetime(v) => v.format(f),
            Self::Use(v) => v.format(f),
        }
    }
}

impl Format for TraitBound {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.polarity.format(f)?;

        if let Some(lifetimes) = &self.lifetimes {
            lifetimes.format(f)?;
            f.text(" ")?;
        }

        self.modifier.format(f)?;
        self.path.format(f)
    }
}

impl Format for TraitRef {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.polarity.format(f)?;
        self.path.format(f)
    }
}

impl Format for UseBound {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("use<")?;
        self.lifetimes.format(f)?;
        f.text(">")
    }
}
