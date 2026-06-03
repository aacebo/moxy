use moxy_ast::generics::{
    ConstParam, GenericParam, LifetimeParam, LifetimePredicate, TraitBound, TypeBound, TypeParam, TypePredicate, UseBound,
};
use moxy_ast::{BoundLifetimes, Generics, TraitRef, WhereClause, WherePredicate};

use crate::{Fmt, FmtError, Formatter};

impl Fmt for BoundLifetimes {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("for<")?;
        self.params.fmt(f)?;
        f.text(">")
    }
}

impl Fmt for Generics {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        if self.params.is_empty() {
            return Ok(());
        }

        f.text("<")?;
        self.params.fmt(f)?;
        f.text(">")?;

        if let Some(where_clause) = &self.where_clause {
            where_clause.fmt(f)?;
        }

        Ok(())
    }
}

impl Fmt for WhereClause {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.hard_break()?;
        f.text("where")?;

        for pair in self.predicates.pairs() {
            f.hard_break()?;
            f.indent(|f| match pair {
                moxy_ast::Pair::Punctuated(pred, _) => {
                    pred.fmt(f)?;
                    f.text(",")
                }
                moxy_ast::Pair::End(pred) => pred.fmt(f),
            })?;
        }

        Ok(())
    }
}

impl Fmt for WherePredicate {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Lifetime(v) => v.fmt(f),
            Self::Type(v) => v.fmt(f),
        }
    }
}

impl Fmt for LifetimePredicate {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.lifetime.fmt(f)?;
        f.text(": ")?;
        self.bounds.fmt(f)
    }
}

impl Fmt for TypePredicate {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        if let Some(lifetimes) = &self.lifetimes {
            lifetimes.fmt(f)?;
            f.space()?;
        }

        self.bounded_ty.fmt(f)?;
        f.text(": ")?;
        self.bounds.fmt(f)
    }
}

impl Fmt for GenericParam {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Lifetime(v) => v.fmt(f),
            Self::Type(v) => v.fmt(f),
            Self::Const(v) => v.fmt(f),
        }
    }
}

impl Fmt for LifetimeParam {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.lifetime.fmt(f)?;

        if !self.bounds.is_empty() {
            f.text(": ")?;
            self.bounds.fmt(f)?;
        }

        Ok(())
    }
}

impl Fmt for TypeParam {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.ident.fmt(f)?;

        if !self.bounds.is_empty() {
            f.text(": ")?;
            self.bounds.fmt(f)?;
        }

        if let Some(default) = &self.default {
            f.text(" = ")?;
            default.fmt(f)?;
        }

        Ok(())
    }
}

impl Fmt for ConstParam {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("const ")?;
        self.ident.fmt(f)?;
        f.text(": ")?;
        self.ty.fmt(f)?;

        if let Some(default) = &self.default {
            f.text(" = ")?;
            default.fmt(f)?;
        }

        Ok(())
    }
}

impl Fmt for TypeBound {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Trait(v) => v.fmt(f),
            Self::Lifetime(v) => v.fmt(f),
            Self::Use(v) => v.fmt(f),
        }
    }
}

impl Fmt for TraitBound {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.polarity.fmt(f)?;

        if let Some(lifetimes) = &self.lifetimes {
            lifetimes.fmt(f)?;
            f.space()?;
        }

        self.modifier.fmt(f)?;
        self.path.fmt(f)
    }
}

impl Fmt for TraitRef {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.polarity.fmt(f)?;
        self.path.fmt(f)
    }
}

impl Fmt for UseBound {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("use<")?;
        self.lifetimes.fmt(f)?;
        f.text(">")
    }
}
