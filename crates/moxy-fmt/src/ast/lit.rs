use moxy_ast::Lit;
use moxy_ast::lit::*;

use crate::{Fmt, FmtError, Formatter};

impl Fmt for Lit {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Str(v) => v.fmt(f),
            Self::ByteStr(v) => v.fmt(f),
            Self::CStr(v) => v.fmt(f),
            Self::Byte(v) => v.fmt(f),
            Self::Char(v) => v.fmt(f),
            Self::Int(v) => v.fmt(f),
            Self::Float(v) => v.fmt(f),
            Self::Bool(v) => v.fmt(f),
            Self::Verbatim(v) => f.text(v),
        }
    }
}

impl Fmt for LitStr {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text(&self.repr)
    }
}

impl Fmt for LitByteStr {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text(&self.repr)
    }
}

impl Fmt for LitCStr {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text(&self.repr)
    }
}

impl Fmt for LitByte {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text(&self.repr)
    }
}

impl Fmt for LitChar {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text(&self.repr)
    }
}

impl Fmt for LitInt {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text(&self.repr)
    }
}

impl Fmt for LitFloat {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text(&self.repr)
    }
}

impl Fmt for LitBool {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text(self.value)
    }
}
