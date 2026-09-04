use crate::{Parse, ParseError, Parser};
use moxy_token::Token;
use moxy_token::{Delim, LexError, Punct, Span, Spanner, ToTokens, TokenStream, TokenTree};

use crate::{Attributes, Delimited, Expr, Ident, Member, Mutability, Path, Punctuated};

mod pat_field;
mod pat_group;
mod pat_ident;
mod pat_lit;
mod pat_or;
mod pat_paren;
mod pat_path;
mod pat_range;
mod pat_reference;
mod pat_slice;
mod pat_struct;
mod pat_tuple;
mod pat_tuple_struct;
mod pat_type;

pub use pat_field::*;
pub use pat_group::*;
pub use pat_ident::*;
pub use pat_lit::*;
pub use pat_or::*;
pub use pat_paren::*;
pub use pat_path::*;
pub use pat_range::*;
pub use pat_reference::*;
pub use pat_slice::*;
pub use pat_struct::*;
pub use pat_tuple::*;
pub use pat_tuple_struct::*;
pub use pat_type::*;

/// A Rust pattern (in `let`, `match`, function params, etc.).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Pattern {
    Wild,
    Rest,
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
    Macro(crate::MacroCall),
    Type(PatType),
    Group(PatGroup),
    Paren(PatParen),
    Box(Box<Self>),
    Const(crate::StmtBlock),
}

impl Pattern {
    pub fn is_wild(&self) -> bool {
        matches!(self, Self::Wild)
    }

    pub fn is_rest(&self) -> bool {
        matches!(self, Self::Rest)
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

    pub fn as_ident(&self) -> Option<&PatIdent> {
        if let Self::Ident(v) = self { Some(v) } else { None }
    }

    pub fn as_path(&self) -> Option<&PatPath> {
        if let Self::Path(v) = self { Some(v) } else { None }
    }

    pub fn as_tuple(&self) -> Option<&PatTuple> {
        if let Self::Tuple(v) = self { Some(v) } else { None }
    }

    pub fn as_tuple_struct(&self) -> Option<&PatTupleStruct> {
        if let Self::TupleStruct(v) = self { Some(v) } else { None }
    }

    pub fn as_struct(&self) -> Option<&PatStruct> {
        if let Self::Struct(v) = self { Some(v) } else { None }
    }

    pub fn as_slice(&self) -> Option<&PatSlice> {
        if let Self::Slice(v) = self { Some(v) } else { None }
    }

    pub fn as_reference(&self) -> Option<&PatReference> {
        if let Self::Reference(v) = self { Some(v) } else { None }
    }

    pub fn as_or(&self) -> Option<&PatOr> {
        if let Self::Or(v) = self { Some(v) } else { None }
    }

    pub fn as_lit(&self) -> Option<&PatLit> {
        if let Self::Lit(v) = self { Some(v) } else { None }
    }

    pub fn as_range(&self) -> Option<&PatRange> {
        if let Self::Range(v) = self { Some(v.as_ref()) } else { None }
    }

    pub fn as_macro(&self) -> Option<&crate::MacroCall> {
        if let Self::Macro(v) = self { Some(v) } else { None }
    }

    pub fn as_type(&self) -> Option<&PatType> {
        if let Self::Type(v) = self { Some(v) } else { None }
    }

    pub fn as_group(&self) -> Option<&PatGroup> {
        if let Self::Group(v) = self { Some(v) } else { None }
    }

    pub fn as_paren(&self) -> Option<&PatParen> {
        if let Self::Paren(v) = self { Some(v) } else { None }
    }

    pub fn as_box(&self) -> Option<&Self> {
        if let Self::Box(v) = self { Some(v.as_ref()) } else { None }
    }

    pub fn as_const(&self) -> Option<&crate::StmtBlock> {
        if let Self::Const(v) = self { Some(v) } else { None }
    }
}

impl Spanner for Pattern {
    fn span(&self) -> Span {
        match self {
            Self::Wild | Self::Rest => Span::call_site(),
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
            Self::Box(p) => p.span(),
            Self::Const(b) => b.span(),
        }
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

impl Parse for Pattern {
    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        // Optional leading `|`, then one-or-more `|`-separated alternatives.
        let leading = parser.peek::<Token![|]>();

        if leading {
            let _ = parser.parse::<Token![|]>()?;
        }

        let first = parse_single(parser)?;

        if !leading && !parser.peek::<Token![|]>() {
            return Ok(first);
        }

        let mut cases = Punctuated::new();
        cases.push_value(first);

        while parser.peek::<Token![|]>() {
            cases.push_punct(parser.parse::<Token![|]>()?);
            cases.push_value(parse_single(parser)?);
        }

        Ok(Self::Or(PatOr {
            attrs: Attributes::default(),
            cases,
        }))
    }
}

impl Pattern {
    /// Parse a single pattern alternative (no top-level `|` or-collection).
    /// Used where `|` is a delimiter (closure params), not an or-pattern.
    pub fn parse_single(parser: &Parser) -> Result<Self, ParseError> {
        parse_single(parser)
    }
}

impl ToTokens for Pattern {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            Self::Wild => {
                moxy_token::Ident::new("_").to_tokens(t);
            }
            Self::Rest => <Token![..]>::default().to_tokens(t),
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
            Self::Box(p) => {
                <Token![box]>::default().to_tokens(t);
                p.to_tokens(t);
            }
            Self::Const(b) => {
                <Token![const]>::default().to_tokens(t);
                b.to_tokens(t);
            }
        }
    }
}

impl PatIdent {
    pub fn parse_from(parser: &Parser, attrs: Attributes) -> Result<Self, ParseError> {
        let by_ref = parser.parse_if::<Token![ref]>();
        let mutability = parser.parse::<Mutability>()?;
        let ident = parser.parse::<Ident>()?;
        let subpat = if parser.peek::<Token![@]>() {
            let at = parser.parse::<Token![@]>()?;
            Some((at, Box::new(parser.parse::<Pattern>()?)))
        } else {
            None
        };

        Ok(Self {
            attrs,
            by_ref,
            mutability,
            ident,
            subpat,
        })
    }
}

impl PatStruct {
    pub fn parse_body(parser: &Parser) -> Result<(Punctuated<PatField, Token![,]>, Option<Token![..]>), ParseError> {
        let mut fields = Punctuated::new();
        let mut rest = None;

        while !parser.is_empty() {
            if parser.peek::<Token![..]>() {
                rest = Some(parser.parse::<Token![..]>()?);
                break;
            }

            let field_attrs = parser.parse::<Attributes>()?;
            let member = parser.parse::<Member>()?;
            let (colon, pat, shorthand) = if parser.peek::<Token![:]>() {
                let colon = parser.parse::<Token![:]>()?;
                (Some(colon), parser.parse::<Pattern>()?, false)
            } else {
                // shorthand `{ field }`
                let ident = match &member {
                    Member::Named(id) => id.clone(),
                    Member::Unnamed(_) => {
                        return Err(LexError::new(parser.span()).message("tuple index needs a pattern").into());
                    }
                };
                (
                    None,
                    Pattern::Ident(PatIdent {
                        attrs: Attributes::default(),
                        by_ref: None,
                        mutability: Mutability::Immutable,
                        ident,
                        subpat: None,
                    }),
                    true,
                )
            };

            fields.push_value(PatField {
                attrs: field_attrs,
                member,
                colon,
                pat,
                shorthand,
            });

            if parser.peek::<Token![,]>() {
                fields.push_punct(parser.parse::<Token![,]>()?);
            } else {
                break;
            }
        }

        Ok((fields, rest))
    }
}

fn parse_single(parser: &Parser) -> Result<Pattern, ParseError> {
    let at = parser.span();
    let attrs = parser.parse::<Attributes>()?;

    // Wildcard `_`
    if matches!(parser.curr(), Some(tt) if tt.text() == Some("_")) {
        parser.advance();
        return Ok(Pattern::Wild);
    }

    // Rest `..`
    if parser.peek::<Token![..]>() {
        let _ = parser.parse::<Token![..]>()?;
        return Ok(Pattern::Rest);
    }

    // `box pat`
    if matches!(parser.curr(), Some(tt) if tt.text() == Some("box")) {
        parser.advance();
        return Ok(Pattern::Box(Box::new(parse_single(parser)?)));
    }

    // `const { ... }` block pattern
    if matches!(parser.curr(), Some(tt) if tt.text() == Some("const"))
        && matches!(parser.nth(1), Some(moxy_token::TokenTree::Group(g)) if g.delim() == Delim::Brace)
    {
        parser.advance();
        return Ok(Pattern::Const(parser.parse::<crate::StmtBlock>()?));
    }

    // Reference `&`/`&mut`
    if parser.peek::<Token![&]>() {
        let and = parser.parse::<Token![&]>()?;
        let mutability = parser.parse::<Mutability>()?;
        let pat = Box::new(parser.parse::<Pattern>()?);
        return Ok(Pattern::Reference(PatReference {
            attrs,
            and,
            mutability,
            pat,
        }));
    }

    // Tuple/paren `(...)`
    if matches!(parser.curr(), Some(tt) if tt.delim() == Some(Delim::Paren)) {
        let elems = Delimited::parse_paren_with(parser, Punctuated::parse_terminated)?;
        return Ok(Pattern::Tuple(PatTuple { attrs, elems }));
    }

    // Slice `[...]`
    if matches!(parser.curr(), Some(tt) if tt.delim() == Some(Delim::Bracket)) {
        let elems = Delimited::parse_bracket_with(parser, Punctuated::parse_terminated)?;
        return Ok(Pattern::Slice(PatSlice { attrs, elems }));
    }

    // `ref`/`mut`-led binding
    if parser.peek::<Token![ref]>() || parser.peek::<Token![mut]>() {
        return Ok(Pattern::Ident(PatIdent::parse_from(parser, attrs)?));
    }

    // Literal pattern
    if matches!(parser.curr(), Some(tt) if matches!(tt, TokenTree::Literal(_))) {
        let expr = parser.parse::<Expr>()?;
        return Ok(Pattern::Lit(PatLit { attrs, expr }));
    }

    // Path-led: ident binding, path, tuple-struct, or struct pattern.
    if matches!(
        parser.curr(),
        Some(TokenTree::Ident(_) | TokenTree::Keyword(_) | TokenTree::Punct(Punct::PathSep(_)))
    ) {
        // Single bare ident with no `::`/`(`/`{` → binding.
        let path = parser.parse::<Path>()?;

        if matches!(parser.curr(), Some(tt) if tt.delim() == Some(Delim::Paren)) {
            let elems = Delimited::parse_paren_with(parser, Punctuated::parse_terminated)?;

            return Ok(Pattern::TupleStruct(PatTupleStruct {
                attrs,
                qself: None,
                path,
                elems,
            }));
        }

        if matches!(parser.curr(), Some(tt) if tt.delim() == Some(Delim::Brace)) {
            let body = Delimited::parse_brace_with(parser, |inner| {
                let (fields, rest) = PatStruct::parse_body(inner)?;
                Ok(PatStructBody { fields, rest })
            })?;

            return Ok(Pattern::Struct(PatStruct {
                attrs,
                qself: None,
                path,
                body,
            }));
        }

        // Bare single-segment path with no leading colon → binding ident.
        return if let Some(ident) = path.as_ident().cloned() {
            Ok(Pattern::Ident(PatIdent {
                attrs,
                by_ref: None,
                mutability: Mutability::Immutable,
                ident,
                subpat: None,
            }))
        } else {
            Ok(Pattern::Path(PatPath {
                attrs,
                qself: None,
                path,
            }))
        };
    }

    Err(LexError::new(at).message("expected pattern").into())
}
