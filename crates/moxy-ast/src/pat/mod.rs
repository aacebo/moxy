mod pat_box;
mod pat_const;
mod pat_field;
mod pat_group;
mod pat_ident;
mod pat_lit;
mod pat_macro;
mod pat_or;
mod pat_paren;
mod pat_path;
mod pat_range;
mod pat_reference;
mod pat_rest;
mod pat_slice;
mod pat_struct;
mod pat_tuple;
mod pat_tuple_struct;
mod pat_type;
mod pat_wild;

pub(crate) mod parse;
pub(crate) mod skip;

pub use pat_box::*;
pub use pat_const::*;
pub use pat_field::*;
pub use pat_group::*;
pub use pat_ident::*;
pub use pat_lit::*;
pub use pat_macro::*;
pub use pat_or::*;
pub use pat_paren::*;
pub use pat_path::*;
pub use pat_range::*;
pub use pat_reference::*;
pub use pat_rest::*;
pub use pat_slice::*;
pub use pat_struct::*;
pub use pat_tuple::*;
pub use pat_tuple_struct::*;
pub use pat_type::*;
pub use pat_wild::*;

use moxy_token::{Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// A Rust pattern (in `let`, `match`, function params, etc.).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Pattern {
    Wild(PatWild),
    Rest(PatRest),
    Ident(PatIdent),
    Path(PatPath),
    Tuple(PatTuple),
    TupleStruct(PatTupleStruct),
    Struct(PatStruct),
    Slice(PatSlice),
    Reference(PatReference),
    Or(PatOr),
    Lit(PatLit),
    Range(Box<PatRange>),
    Macro(PatMacro),
    Type(PatType),
    Group(PatGroup),
    Paren(PatParen),
    Box(PatBox),
    Const(PatConst),
}

impl Pattern {
    pub fn is_wild(&self) -> bool {
        matches!(self, Self::Wild(_))
    }

    pub fn is_rest(&self) -> bool {
        matches!(self, Self::Rest(_))
    }

    pub fn is_ident(&self) -> bool {
        matches!(self, Self::Ident(_))
    }

    pub fn is_path(&self) -> bool {
        matches!(self, Self::Path(_))
    }

    pub fn is_tuple(&self) -> bool {
        matches!(self, Self::Tuple(_))
    }

    pub fn is_tuple_struct(&self) -> bool {
        matches!(self, Self::TupleStruct(_))
    }

    pub fn is_struct(&self) -> bool {
        matches!(self, Self::Struct(_))
    }

    pub fn is_slice(&self) -> bool {
        matches!(self, Self::Slice(_))
    }

    pub fn is_reference(&self) -> bool {
        matches!(self, Self::Reference(_))
    }

    pub fn is_or(&self) -> bool {
        matches!(self, Self::Or(_))
    }

    pub fn is_lit(&self) -> bool {
        matches!(self, Self::Lit(_))
    }

    pub fn is_range(&self) -> bool {
        matches!(self, Self::Range(_))
    }

    pub fn is_macro(&self) -> bool {
        matches!(self, Self::Macro(_))
    }

    pub fn is_type(&self) -> bool {
        matches!(self, Self::Type(_))
    }

    pub fn is_group(&self) -> bool {
        matches!(self, Self::Group(_))
    }

    pub fn is_paren(&self) -> bool {
        matches!(self, Self::Paren(_))
    }

    pub fn is_box(&self) -> bool {
        matches!(self, Self::Box(_))
    }

    pub fn is_const(&self) -> bool {
        matches!(self, Self::Const(_))
    }
}

impl From<PatIdent> for Pattern {
    fn from(value: PatIdent) -> Self {
        Self::Ident(value)
    }
}

impl From<PatPath> for Pattern {
    fn from(value: PatPath) -> Self {
        Self::Path(value)
    }
}

impl From<PatTuple> for Pattern {
    fn from(value: PatTuple) -> Self {
        Self::Tuple(value)
    }
}

impl From<PatTupleStruct> for Pattern {
    fn from(value: PatTupleStruct) -> Self {
        Self::TupleStruct(value)
    }
}

impl From<PatStruct> for Pattern {
    fn from(value: PatStruct) -> Self {
        Self::Struct(value)
    }
}

impl From<PatSlice> for Pattern {
    fn from(value: PatSlice) -> Self {
        Self::Slice(value)
    }
}

impl From<PatReference> for Pattern {
    fn from(value: PatReference) -> Self {
        Self::Reference(value)
    }
}

impl From<PatOr> for Pattern {
    fn from(value: PatOr) -> Self {
        Self::Or(value)
    }
}

impl From<PatLit> for Pattern {
    fn from(value: PatLit) -> Self {
        Self::Lit(value)
    }
}

impl From<PatRange> for Pattern {
    fn from(value: PatRange) -> Self {
        Self::Range(Box::new(value))
    }
}

impl From<PatType> for Pattern {
    fn from(value: PatType) -> Self {
        Self::Type(value)
    }
}

impl From<PatGroup> for Pattern {
    fn from(value: PatGroup) -> Self {
        Self::Group(value)
    }
}

impl From<PatParen> for Pattern {
    fn from(value: PatParen) -> Self {
        Self::Paren(value)
    }
}

impl From<PatConst> for Pattern {
    fn from(value: PatConst) -> Self {
        Self::Const(value)
    }
}

impl From<PatBox> for Pattern {
    fn from(value: PatBox) -> Self {
        Self::Box(value)
    }
}

impl From<PatMacro> for Pattern {
    fn from(value: PatMacro) -> Self {
        Self::Macro(value)
    }
}

impl From<PatWild> for Pattern {
    fn from(value: PatWild) -> Self {
        Self::Wild(value)
    }
}

impl From<PatRest> for Pattern {
    fn from(value: PatRest) -> Self {
        Self::Rest(value)
    }
}

impl Spanner for Pattern {
    fn span(&self) -> Span {
        match self {
            Self::Wild(v) => v.span(),
            Self::Rest(v) => v.span(),
            Self::Ident(v) => v.span(),
            Self::Path(v) => v.span(),
            Self::Tuple(v) => v.span(),
            Self::TupleStruct(v) => v.span(),
            Self::Struct(v) => v.span(),
            Self::Slice(v) => v.span(),
            Self::Reference(v) => v.span(),
            Self::Or(v) => v.span(),
            Self::Lit(v) => v.span(),
            Self::Range(v) => v.span(),
            Self::Macro(v) => v.span(),
            Self::Type(v) => v.span(),
            Self::Group(v) => v.span(),
            Self::Paren(v) => v.span(),
            Self::Box(v) => v.span(),
            Self::Const(v) => v.span(),
        }
    }
}

impl Parse for Pattern {
    fn peek(cursor: Cursor<'_>) -> bool {
        let cursor = Attributes::skip(cursor).unwrap_or(cursor);

        cursor.peek::<PatOr>()
            || cursor.peek::<PatWild>()
            || cursor.peek::<PatRange>()
            || cursor.peek::<PatRest>()
            || cursor.peek::<PatBox>()
            || cursor.peek::<PatConst>()
            || cursor.peek::<PatReference>()
            || cursor.peek::<PatGroup>()
            || cursor.peek::<PatSlice>()
            || cursor.is_delimited(moxy_token::Delim::Paren)
            || cursor.peek::<MacroCall>()
            || cursor.peek::<PatTupleStruct>()
            || cursor.peek::<PatStruct>()
            || cursor.peek::<PatIdent>()
            || cursor.peek::<PatPath>()
            || cursor.peek::<PatLit>()
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let bare = Attributes::skip(parser.cursor()).unwrap_or(parser.cursor());
        let leading = bare.peek::<Token![|]>();
        let attrs = if leading { parser.parse()? } else { Attributes::default() };

        if leading {
            let _: Token![|] = parser.parse()?;
        }

        let first = parse::single(parser)?;

        if !leading && !parser.peek::<Token![|]>() {
            return Ok(first);
        }

        let mut cases = Punctuated::new();
        cases.push_value(first);

        while parser.peek::<Token![|]>() {
            cases.push_punct(parser.parse()?);
            cases.push_value(parse::single(parser)?);
        }

        Ok(Self::Or(PatOr { attrs, cases }))
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        let bare = Attributes::skip(cursor)?;

        if bare.peek::<Token![|]>() {
            cursor = bare;
            cursor = cursor.skip::<Token![|]>()?;
        }

        cursor = skip::single(cursor)?;

        while cursor.peek::<Token![|]>() {
            cursor = cursor.skip::<Token![|]>()?;
            cursor = skip::single(cursor)?;
        }

        Some(cursor)
    }
}

impl ToTokens for Pattern {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            Self::Wild(v) => v.to_tokens(t),
            Self::Rest(v) => v.to_tokens(t),
            Self::Ident(v) => v.to_tokens(t),
            Self::Path(v) => v.to_tokens(t),
            Self::Tuple(v) => v.to_tokens(t),
            Self::TupleStruct(v) => v.to_tokens(t),
            Self::Struct(v) => v.to_tokens(t),
            Self::Slice(v) => v.to_tokens(t),
            Self::Reference(v) => v.to_tokens(t),
            Self::Or(v) => v.to_tokens(t),
            Self::Lit(v) => v.to_tokens(t),
            Self::Range(v) => v.to_tokens(t),
            Self::Macro(v) => v.to_tokens(t),
            Self::Type(v) => v.to_tokens(t),
            Self::Group(v) => v.to_tokens(t),
            Self::Paren(v) => v.to_tokens(t),
            Self::Box(v) => v.to_tokens(t),
            Self::Const(v) => v.to_tokens(t),
        }
    }
}
