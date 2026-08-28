mod attr;
mod crate_;
mod expr;
mod generics;
mod item;
mod leaf;
mod lit;
mod macro_call;
mod pat;
mod path;
mod stmt;
mod ty;

impl crate::Format for moxy_ast::Declaration {
    fn format(&self, f: &mut crate::Formatter) -> Result<(), crate::FmtError> {
        match self {
            Self::Enum(v) => v.format(f),
            Self::Struct(v) => v.format(f),
            Self::Union(v) => v.format(f),
        }
    }
}
