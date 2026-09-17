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
#[derive(Debug, Clone, PartialEq, Eq)]
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

        fn parse_single(parser: &Parser) -> Result<Pattern, ParseError> {
            if parser.peek::<PatWild>() {
                return Ok(Pattern::Wild(parser.parse()?));
            }

            if parser.peek::<PatRange>() {
                return Ok(Pattern::Range(parser.parse()?));
            }

            if parser.peek::<PatRest>() {
                return Ok(Pattern::Rest(parser.parse()?));
            }

            if parser.peek::<PatBox>() {
                return Ok(Pattern::Box(parser.parse()?));
            }

            if parser.peek::<PatConst>() {
                return Ok(Pattern::Const(parser.parse()?));
            }

            if parser.peek::<PatReference>() {
                return Ok(Pattern::Reference(parser.parse()?));
            }

            if parser.peek::<PatGroup>() {
                return Ok(Pattern::Group(parser.parse()?));
            }

            if parser.peek::<PatSlice>() {
                return Ok(Pattern::Slice(parser.parse()?));
            }

            if parser.peek::<PatTuple>() {
                return Ok(Pattern::Tuple(parser.parse()?));
            }

            if parser.peek::<PatParen>() {
                return Ok(Pattern::Paren(parser.parse()?));
            }

            if parser.peek::<PatMacro>() {
                return Ok(Pattern::Macro(parser.parse()?));
            }

            if parser.peek::<PatTupleStruct>() {
                return Ok(Pattern::TupleStruct(parser.parse()?));
            }

            if parser.peek::<PatStruct>() {
                return Ok(Pattern::Struct(parser.parse()?));
            }

            if parser.peek::<PatIdent>() {
                return Ok(Pattern::Ident(parser.parse()?));
            }

            if parser.peek::<PatPath>() {
                return Ok(Pattern::Path(parser.parse()?));
            }

            if parser.peek::<PatLit>() {
                return Ok(Pattern::Lit(parser.parse()?));
            }

            parser.error("expected pattern").into()
        }

        let first = parse_single(parser)?;

        if !leading && !parser.peek::<Token![|]>() {
            return Ok(first);
        }

        let mut cases = Punctuated::new();
        cases.push_value(first);

        while parser.peek::<Token![|]>() {
            cases.push_punct(parser.parse()?);
            cases.push_value(parse_single(parser)?);
        }

        Ok(Self::Or(PatOr { attrs, cases }))
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        fn skip_single(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
            if cursor.peek::<PatWild>() {
                return cursor.skip::<PatWild>();
            }

            if cursor.peek::<PatRange>() {
                return cursor.skip::<PatRange>();
            }

            if cursor.peek::<PatRest>() {
                return cursor.skip::<PatRest>();
            }

            if cursor.peek::<PatBox>() {
                return cursor.skip::<PatBox>();
            }

            if cursor.peek::<PatConst>() {
                return cursor.skip::<PatConst>();
            }

            if cursor.peek::<PatReference>() {
                return cursor.skip::<PatReference>();
            }

            if cursor.is_delimited(moxy_token::Delim::None) {
                return cursor.skip::<PatGroup>();
            }

            if cursor.is_delimited(moxy_token::Delim::Bracket) {
                return cursor.skip::<PatSlice>();
            }

            if cursor.is_delimited(moxy_token::Delim::Paren) {
                if cursor.peek::<PatTuple>() {
                    return cursor.skip::<PatTuple>();
                }

                return cursor.skip::<PatParen>();
            }

            if cursor.peek::<MacroCall>() {
                return cursor.skip::<MacroCall>();
            }

            if cursor.peek::<PatTupleStruct>() {
                return cursor.skip::<PatTupleStruct>();
            }

            if cursor.peek::<PatStruct>() {
                return cursor.skip::<PatStruct>();
            }

            if cursor.peek::<PatIdent>() {
                return cursor.skip::<PatIdent>();
            }

            if cursor.peek::<PatPath>() {
                return cursor.skip::<PatPath>();
            }

            cursor.skip::<PatLit>()
        }

        let bare = Attributes::skip(cursor)?;

        if bare.peek::<Token![|]>() {
            cursor = bare;
            cursor = cursor.skip::<Token![|]>()?;
        }

        cursor = skip_single(cursor)?;

        while cursor.peek::<Token![|]>() {
            cursor = cursor.skip::<Token![|]>()?;
            cursor = skip_single(cursor)?;
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
