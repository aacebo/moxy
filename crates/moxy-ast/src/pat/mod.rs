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
#[non_exhaustive]
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

impl Attributed for Pattern {
    fn attrs(&self) -> &[Attribute] {
        match self {
            Self::Wild(value) => &value.attrs,
            Self::Rest(value) => &value.attrs,
            Self::Ident(value) => &value.attrs,
            Self::Path(value) => &value.attrs,
            Self::Tuple(value) => &value.attrs,
            Self::TupleStruct(value) => &value.attrs,
            Self::Struct(value) => &value.attrs,
            Self::Slice(value) => &value.attrs,
            Self::Reference(value) => &value.attrs,
            Self::Or(value) => &value.attrs,
            Self::Lit(value) => &value.attrs,
            Self::Range(value) => &value.attrs,
            Self::Macro(value) => &value.attrs,
            Self::Type(value) => &value.attrs,
            Self::Group(value) => &value.attrs,
            Self::Paren(value) => &value.attrs,
            Self::Box(value) => &value.attrs,
            Self::Const(value) => &value.attrs,
        }
    }
}

impl Pattern {
    // The attributes of a closure parameter, which rustc allows on no other
    // pattern, go on its pattern
    pub(crate) fn attrs_mut(&mut self) -> &mut Attributes {
        match self {
            Self::Wild(value) => &mut value.attrs,
            Self::Rest(value) => &mut value.attrs,
            Self::Ident(value) => &mut value.attrs,
            Self::Path(value) => &mut value.attrs,
            Self::Tuple(value) => &mut value.attrs,
            Self::TupleStruct(value) => &mut value.attrs,
            Self::Struct(value) => &mut value.attrs,
            Self::Slice(value) => &mut value.attrs,
            Self::Reference(value) => &mut value.attrs,
            Self::Or(value) => &mut value.attrs,
            Self::Lit(value) => &mut value.attrs,
            Self::Range(value) => &mut value.attrs,
            Self::Macro(value) => &mut value.attrs,
            Self::Type(value) => &mut value.attrs,
            Self::Group(value) => &mut value.attrs,
            Self::Paren(value) => &mut value.attrs,
            Self::Box(value) => &mut value.attrs,
            Self::Const(value) => &mut value.attrs,
        }
    }

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
    // A pattern starts here when its first tokens start one of its forms; no
    // form is read to the end
    fn peek(cursor: Cursor<'_>) -> bool {
        <Token![|]>::peek(cursor) || !matches!(Form::of_start(cursor), Form::None)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let leading = <Token![|]>::peek(parser.cursor());
        let attrs = Attributes::default();

        let leading_vert = if leading { Some(<_ as Parse>::parse(parser)?) } else { None };

        let first = parse::single(parser)?;

        if !leading && !<Token![|]>::peek(parser.cursor()) {
            return Ok(first);
        }

        let mut cases = List::new();
        cases.push_value(first);

        while <Token![|]>::peek(parser.cursor()) {
            cases.push_punct(<_ as Parse>::parse(parser)?);
            cases.push_value(parse::single(parser)?);
        }

        Ok(Self::Or(PatOr {
            attrs,
            leading_vert,
            cases,
        }))
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        if <Token![|]>::peek(cursor) {
            cursor = <Token![|]>::skip(cursor)?;
        }

        cursor = skip::single(cursor)?;

        while <Token![|]>::peek(cursor) {
            cursor = <Token![|]>::skip(cursor)?;
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

// The form of a single pattern, told from its first tokens, or for a pattern
// that starts with a path, from the token after the path
#[derive(Clone, Copy)]
pub(crate) enum Form {
    Wild,
    RangeWithoutStart,
    Rest,
    Box,
    Const,
    Reference,
    Group,
    Slice,
    ParenOrTuple,
    Ident,
    Macro,
    TupleStruct,
    Struct,
    Path,
    Lit,
    None,
}

impl Form {
    pub(crate) fn of(cursor: Cursor<'_>) -> Self {
        match Self::of_start(cursor) {
            Self::Path => match Self::path_end(cursor) {
                Some(after) if after.is_delimited(moxy_token::Delim::Paren) => Self::TupleStruct,
                Some(after) if after.is_delimited(moxy_token::Delim::Brace) => Self::Struct,
                _ => Self::Path,
            },
            form => form,
        }
    }

    // The form as far as the first tokens tell it; a path is `Path` here
    pub(crate) fn of_start(cursor: Cursor<'_>) -> Self {
        if PatWild::peek(cursor) {
            Self::Wild
        } else if PatRange::peek_without_start(cursor) {
            Self::RangeWithoutStart
        } else if PatRest::peek(cursor) {
            Self::Rest
        } else if PatBox::peek(cursor) {
            Self::Box
        } else if PatConst::peek(cursor) {
            Self::Const
        } else if PatReference::peek(cursor) {
            Self::Reference
        } else if cursor.is_delimited(moxy_token::Delim::None) {
            Self::Group
        } else if cursor.is_delimited(moxy_token::Delim::Bracket) {
            Self::Slice
        } else if cursor.is_delimited(moxy_token::Delim::Paren) {
            Self::ParenOrTuple
        } else if MacroCall::peek(cursor) {
            Self::Macro
        } else if PatIdent::peek(cursor) {
            Self::Ident
        } else if <Token![<]>::peek(cursor) || Path::peek(cursor) {
            Self::Path
        } else if PatLit::peek(cursor) {
            Self::Lit
        } else {
            Self::None
        }
    }

    pub(crate) fn path_end(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        if <Token![<]>::peek(cursor) {
            TypePath::skip_expr(cursor)
        } else {
            Path::skip_expr(cursor)
        }
    }
}
