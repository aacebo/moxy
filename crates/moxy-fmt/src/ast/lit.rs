use moxy_ast::Lit;
use moxy_ast::lit::*;

use crate::{FmtError, Format, Formatter};

impl Format for Lit {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        match self {
            Self::Str(v) => v.format(f),
            Self::ByteStr(v) => v.format(f),
            Self::CStr(v) => v.format(f),
            Self::Byte(v) => v.format(f),
            Self::Char(v) => v.format(f),
            Self::Int(v) => v.format(f),
            Self::Float(v) => v.format(f),
            Self::Bool(v) => v.format(f),
            Self::Verbatim(v) => f.text(v),
        }
    }
}

impl Format for LitStr {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text(&self.repr)
    }
}

impl Format for LitByteStr {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text(&self.repr)
    }
}

impl Format for LitCStr {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text(&self.repr)
    }
}

impl Format for LitByte {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text(&self.repr)
    }
}

impl Format for LitChar {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text(&self.repr)
    }
}

impl Format for LitInt {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text(&self.repr)
    }
}

impl Format for LitFloat {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text(&self.repr)
    }
}

impl Format for LitBool {
    fn format(&self, f: &mut Formatter) -> Result<(), FmtError> {
        f.text(self.value)
    }
}
