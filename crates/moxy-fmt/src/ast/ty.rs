use moxy_ast::Type;
use moxy_ast::sig::BareFnArg;
use moxy_ast::ty::*;

use crate::{Fmt, FmtError, Formatter};

impl Fmt for Type {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Never => f.text("!"),
            Self::Infer => f.text("_"),
            Self::Path(v) => v.fmt(f),
            Self::Tuple(v) => v.fmt(f),
            Self::Array(v) => v.fmt(f),
            Self::Slice(v) => v.fmt(f),
            Self::Reference(v) => v.fmt(f),
            Self::Pointer(v) => v.fmt(f),
            Self::BareFn(v) => v.fmt(f),
            Self::ImplTrait(v) => v.fmt(f),
            Self::TraitObject(v) => v.fmt(f),
            Self::Paren(v) => v.fmt(f),
            Self::Group(v) => v.elem.fmt(f),
            Self::Macro(v) => v.fmt(f),
        }
    }
}

impl Fmt for TypePath {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        if let Some(qself) = &self.qself {
            f.text("<")?;
            qself.ty.fmt(f)?;

            if qself.position > 0 {
                f.text(" as ")?;
                // emit all segments up to position as a path prefix
                for (i, pair) in self.path.segments.pairs().enumerate() {
                    if i >= qself.position {
                        break;
                    }

                    match pair {
                        moxy_ast::Pair::Punctuated(seg, _) => {
                            seg.fmt(f)?;
                            f.text("::")?;
                        }
                        moxy_ast::Pair::End(seg) => {
                            seg.fmt(f)?;
                        }
                    }
                }
            }

            f.text(">")?;
            f.text("::")?;

            // emit remaining segments
            for (i, pair) in self.path.segments.pairs().enumerate() {
                if i < qself.position {
                    continue;
                }

                match pair {
                    moxy_ast::Pair::Punctuated(seg, _) => {
                        seg.fmt(f)?;
                        f.text("::")?;
                    }
                    moxy_ast::Pair::End(seg) => {
                        seg.fmt(f)?;
                    }
                }
            }
        } else {
            self.path.fmt(f)?;
        }

        Ok(())
    }
}

impl Fmt for TypeTuple {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("(")?;
        self.elems.fmt(f)?;
        f.text(")")
    }
}

impl Fmt for TypeArray {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("[")?;
        self.elem.fmt(f)?;
        f.text("; ")?;
        self.len.fmt(f)?;
        f.text("]")
    }
}

impl Fmt for TypeSlice {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("[")?;
        self.elem.fmt(f)?;
        f.text("]")
    }
}

impl Fmt for TypeReference {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("&")?;

        if let Some(lt) = &self.lifetime {
            lt.fmt(f)?;
            f.space()?;
        }

        if matches!(self.mutability, moxy_ast::Mutability::Mutable) {
            f.text("mut ")?;
        }

        self.elem.fmt(f)
    }
}

impl Fmt for TypePointer {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("*")?;

        match self.mutability {
            moxy_ast::Mutability::Mutable => f.text("mut ")?,
            moxy_ast::Mutability::Immutable => f.text("const ")?,
        }

        self.elem.fmt(f)
    }
}

impl Fmt for TypeBareFn {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        if let Some(lifetimes) = &self.lifetimes {
            lifetimes.fmt(f)?;
            f.space()?;
        }

        self.unsafety.fmt(f)?;

        if matches!(self.unsafety, moxy_ast::Unsafety::Unsafe) {
            f.space()?;
        }

        if let Some(abi) = &self.abi {
            abi.fmt(f)?;
            f.space()?;
        }

        f.text("fn(")?;
        self.inputs.fmt(f)?;

        if self.variadic.is_some() {
            if !self.inputs.is_empty() {
                f.text(", ")?;
            }

            f.text("...")?;
        }

        f.text(")")?;
        self.output.fmt(f)
    }
}

impl Fmt for BareFnArg {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        if let Some(name) = &self.name {
            name.fmt(f)?;
            f.text(": ")?;
        }

        self.ty.fmt(f)
    }
}

impl Fmt for TypeImplTrait {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("impl ")?;
        self.bounds.fmt(f)
    }
}

impl Fmt for TypeTraitObject {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        if self.dyn_token {
            f.text("dyn ")?;
        }

        self.bounds.fmt(f)
    }
}

impl Fmt for TypeParen {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text("(")?;
        self.elem.fmt(f)?;
        f.text(")")
    }
}

impl Fmt for TypeMacro {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        self.mac.fmt(f)
    }
}
