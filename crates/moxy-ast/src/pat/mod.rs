use moxy_token::keyword::{Mut, Ref};
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::{And, At, Colon, Comma, DotDot, Or};
use moxy_token::{Delim, LexError, Parse, Punctuation, Span, Spanner, ToTokens, Token, TokenStream, TokenTree};

use crate::{Attribute, Delimited, Expr, Ident, Member, Mutability, Path, Punctuated, RangeLimits, Type};

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
#[derive(Debug, Clone)]
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
    Box(Box<Pattern>),
    Const(crate::StmtBlock),
}

impl Spanner for Pattern {
    fn span(&self) -> Span {
        match self {
            Pattern::Wild | Pattern::Rest => Span::call_site(),
            Pattern::Ident(v) => v.span(),
            Pattern::Path(v) => v.span(),
            Pattern::Tuple(v) => v.span(),
            Pattern::TupleStruct(v) => v.span(),
            Pattern::Struct(v) => v.span(),
            Pattern::Slice(v) => v.span(),
            Pattern::Reference(v) => v.span(),
            Pattern::Or(v) => v.span(),
            Pattern::Lit(v) => v.span(),
            Pattern::Range(v) => v.span(),
            Pattern::Macro(v) => v.span(),
            Pattern::Type(v) => v.span(),
            Pattern::Group(v) => v.span(),
            Pattern::Paren(v) => v.span(),
            Pattern::Box(p) => p.span(),
            Pattern::Const(b) => b.span(),
        }
    }
}

impl From<PatIdent> for Pattern {
    fn from(value: PatIdent) -> Self {
        Pattern::Ident(value)
    }
}

impl From<PatPath> for Pattern {
    fn from(value: PatPath) -> Self {
        Pattern::Path(value)
    }
}

impl From<PatTuple> for Pattern {
    fn from(value: PatTuple) -> Self {
        Pattern::Tuple(value)
    }
}

impl From<PatTupleStruct> for Pattern {
    fn from(value: PatTupleStruct) -> Self {
        Pattern::TupleStruct(value)
    }
}

impl From<PatStruct> for Pattern {
    fn from(value: PatStruct) -> Self {
        Pattern::Struct(value)
    }
}

impl From<PatSlice> for Pattern {
    fn from(value: PatSlice) -> Self {
        Pattern::Slice(value)
    }
}

impl From<PatReference> for Pattern {
    fn from(value: PatReference) -> Self {
        Pattern::Reference(value)
    }
}

impl From<PatOr> for Pattern {
    fn from(value: PatOr) -> Self {
        Pattern::Or(value)
    }
}

impl From<PatLit> for Pattern {
    fn from(value: PatLit) -> Self {
        Pattern::Lit(value)
    }
}

impl From<PatRange> for Pattern {
    fn from(value: PatRange) -> Self {
        Pattern::Range(Box::new(value))
    }
}

impl From<PatType> for Pattern {
    fn from(value: PatType) -> Self {
        Pattern::Type(value)
    }
}

impl From<PatGroup> for Pattern {
    fn from(value: PatGroup) -> Self {
        Pattern::Group(value)
    }
}

impl From<PatParen> for Pattern {
    fn from(value: PatParen) -> Self {
        Pattern::Paren(value)
    }
}

impl Parse for Pattern {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        // Optional leading `|`, then one-or-more `|`-separated alternatives.
        let leading = stream.peek::<Or>();

        if leading {
            let _ = stream.parse::<Or>()?;
        }

        let first = parse_single(stream)?;

        if !leading && !stream.peek::<Or>() {
            return Ok(first);
        }

        let mut cases = Punctuated::new();
        cases.push_value(first);

        while stream.peek::<Or>() {
            cases.push_punct(stream.parse::<Or>()?);
            cases.push_value(parse_single(stream)?);
        }

        Ok(Pattern::Or(PatOr {
            attrs: Vec::new(),
            cases,
        }))
    }
}

impl Pattern {
    /// Parse a single pattern alternative (no top-level `|` or-collection).
    /// Used where `|` is a delimiter (closure params), not an or-pattern.
    pub fn parse_single(stream: &mut ParseStream) -> Result<Pattern, ParseError> {
        parse_single(stream)
    }
}

impl ToTokens for Pattern {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            Pattern::Wild => {
                moxy_token::Ident::new("_", Span::default()).to_tokens(t);
            }
            Pattern::Rest => DotDot::default().to_tokens(t),
            Pattern::Ident(v) => v.to_tokens(t),
            Pattern::Path(v) => v.to_tokens(t),
            Pattern::Tuple(v) => v.to_tokens(t),
            Pattern::TupleStruct(v) => v.to_tokens(t),
            Pattern::Struct(v) => v.to_tokens(t),
            Pattern::Slice(v) => v.to_tokens(t),
            Pattern::Reference(v) => v.to_tokens(t),
            Pattern::Or(v) => v.to_tokens(t),
            Pattern::Lit(v) => v.to_tokens(t),
            Pattern::Range(v) => v.to_tokens(t),
            Pattern::Macro(v) => v.to_tokens(t),
            Pattern::Type(v) => v.to_tokens(t),
            Pattern::Group(v) => v.to_tokens(t),
            Pattern::Paren(v) => v.to_tokens(t),
            Pattern::Box(p) => {
                moxy_token::keyword::Box::default().to_tokens(t);
                p.to_tokens(t);
            }
            Pattern::Const(b) => {
                moxy_token::keyword::Const::default().to_tokens(t);
                b.to_tokens(t);
            }
        }
    }
}

impl PatIdent {
    pub fn parse_from(stream: &mut ParseStream, attrs: Vec<Attribute>) -> Result<Self, ParseError> {
        let by_ref = stream.parse_if::<Ref>();

        let mutability = stream.parse::<Mutability>()?;
        let ident = stream.parse::<Ident>()?;

        let subpat = if stream.peek::<At>() {
            let at = stream.parse::<At>()?;
            Some((at, Box::new(Pattern::parse(stream)?)))
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
    pub fn parse_body(stream: &mut ParseStream) -> Result<(Punctuated<PatField, Comma>, Option<DotDot>), ParseError> {
        let mut fields = Punctuated::new();
        let mut rest = None;

        while !stream.is_empty() {
            if stream.peek::<DotDot>() {
                rest = Some(stream.parse::<DotDot>()?);
                break;
            }

            let member = stream.parse::<Member>()?;

            let (colon, pat, shorthand) = if stream.peek::<Colon>() {
                let colon = stream.parse::<Colon>()?;
                (Some(colon), stream.parse::<Pattern>()?, false)
            } else {
                // shorthand `{ field }`
                let ident = match &member {
                    Member::Named(id) => id.clone(),
                    Member::Unnamed(_) => {
                        return Err(LexError::new(stream.span()).message("tuple index needs a pattern").into());
                    }
                };
                (
                    None,
                    Pattern::Ident(PatIdent {
                        attrs: Vec::new(),
                        by_ref: None,
                        mutability: Mutability::Immutable,
                        ident,
                        subpat: None,
                    }),
                    true,
                )
            };

            fields.push_value(PatField {
                attrs: Vec::new(),
                member,
                colon,
                pat,
                shorthand,
            });

            if stream.peek::<Comma>() {
                fields.push_punct(stream.parse::<Comma>()?);
            } else {
                break;
            }
        }

        Ok((fields, rest))
    }
}

fn parse_single(stream: &mut ParseStream) -> Result<Pattern, ParseError> {
    let at = stream.span();
    let attrs = stream.parse::<Vec<Attribute>>()?;

    // Wildcard `_`
    if matches!(stream.curr(), Some(tt) if tt.name().as_deref() == Some("_")) {
        stream.advance();
        return Ok(Pattern::Wild);
    }

    // Rest `..`
    if stream.peek::<DotDot>() {
        let _ = stream.parse::<DotDot>()?;
        return Ok(Pattern::Rest);
    }

    // `box pat`
    if matches!(stream.curr(), Some(tt) if tt.name().as_deref() == Some("box")) {
        stream.advance();
        return Ok(Pattern::Box(Box::new(parse_single(stream)?)));
    }

    // `const { ... }` block pattern
    if matches!(stream.curr(), Some(tt) if tt.name().as_deref() == Some("const"))
        && matches!(stream.nth(1), Some(moxy_token::TokenTree::Group(g)) if g.delim() == Delim::Brace)
    {
        stream.advance();
        return Ok(Pattern::Const(stream.parse::<crate::StmtBlock>()?));
    }

    // Reference `&`/`&mut`
    if stream.peek::<And>() {
        let and = stream.parse::<And>()?;
        let mutability = stream.parse::<Mutability>()?;
        let pat = Box::new(Pattern::parse(stream)?);
        return Ok(Pattern::Reference(PatReference {
            attrs,
            and,
            mutability,
            pat,
        }));
    }

    // Tuple/paren `(...)`
    if matches!(stream.curr(), Some(tt) if tt.delim() == Some(Delim::Paren)) {
        let elems = Delimited::parse_paren_with(stream, Punctuated::parse_terminated)?;
        return Ok(Pattern::Tuple(PatTuple { attrs, elems }));
    }

    // Slice `[...]`
    if matches!(stream.curr(), Some(tt) if tt.delim() == Some(Delim::Bracket)) {
        let elems = Delimited::parse_bracket_with(stream, Punctuated::parse_terminated)?;
        return Ok(Pattern::Slice(PatSlice { attrs, elems }));
    }

    // `ref`/`mut`-led binding
    if stream.peek::<Ref>() || stream.peek::<Mut>() {
        return Ok(Pattern::Ident(PatIdent::parse_from(stream, attrs)?));
    }

    // Literal pattern
    if matches!(stream.curr(), Some(tt) if matches!(tt, TokenTree::Token(Token::Literal(_)))) {
        let expr = stream.parse::<Expr>()?;
        return Ok(Pattern::Lit(PatLit { attrs, expr }));
    }

    // Path-led: ident binding, path, tuple-struct, or struct pattern.
    if matches!(
        stream.curr(),
        Some(
            TokenTree::Token(Token::Ident(_))
                | TokenTree::Token(Token::Keyword(_))
                | TokenTree::Token(Token::Punct(Punctuation::PathSep(_)))
        )
    ) {
        // Single bare ident with no `::`/`(`/`{` → binding.
        let mut fork = stream.fork();
        let path = fork.parse::<Path>()?;

        if matches!(fork.curr(), Some(tt) if tt.delim() == Some(Delim::Paren)) {
            stream.seek(&fork);
            let elems = Delimited::parse_paren_with(stream, Punctuated::parse_terminated)?;
            return Ok(Pattern::TupleStruct(PatTupleStruct {
                attrs,
                qself: None,
                path,
                elems,
            }));
        }

        if matches!(fork.curr(), Some(tt) if tt.delim() == Some(Delim::Brace)) {
            stream.seek(&fork);
            let body = Delimited::parse_brace_with(stream, |inner| {
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
        if !path.leading_colon && path.segments.len() == 1 {
            stream.seek(&fork);
            let ident = match path.segments.into_iter().next() {
                Some(seg) => seg.ident,
                None => return Err(LexError::new(at).message("expected pattern").into()),
            };
            return Ok(Pattern::Ident(PatIdent {
                attrs,
                by_ref: None,
                mutability: Mutability::Immutable,
                ident,
                subpat: None,
            }));
        }

        stream.seek(&fork);
        return Ok(Pattern::Path(PatPath {
            attrs,
            qself: None,
            path,
        }));
    }

    Err(LexError::new(at).message("expected pattern").into())
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use moxy_token::ToTokenStream;

    use super::*;

    fn roundtrip(src: &str) -> String {
        let p: Pattern = moxy_token::parse!(src).unwrap();
        p.to_token_stream().to_string()
    }

    #[test]
    fn wildcard() {
        assert!(matches!(moxy_token::parse!("_" as Pattern).unwrap(), Pattern::Wild));
        assert_eq!(roundtrip("_"), "_");
    }

    #[test]
    fn rest() {
        assert!(matches!(moxy_token::parse!(".." as Pattern).unwrap(), Pattern::Rest));
        assert_eq!(roundtrip(".."), "..");
    }

    #[test]
    fn ident_binding() {
        assert!(matches!(moxy_token::parse!("x" as Pattern).unwrap(), Pattern::Ident(_)));
        assert_eq!(roundtrip("x"), "x");
    }

    #[test]
    fn mut_binding() {
        assert!(matches!(moxy_token::parse!("mut x" as Pattern).unwrap(), Pattern::Ident(_)));
    }

    #[test]
    fn ref_binding() {
        assert!(matches!(moxy_token::parse!("ref x" as Pattern).unwrap(), Pattern::Ident(_)));
    }

    #[test]
    fn tuple_pattern() {
        assert!(matches!(moxy_token::parse!("(a, b)" as Pattern).unwrap(), Pattern::Tuple(_)));
    }

    #[test]
    fn slice_pattern() {
        assert!(matches!(moxy_token::parse!("[a, b]" as Pattern).unwrap(), Pattern::Slice(_)));
    }

    #[test]
    fn reference_pattern() {
        assert!(matches!(moxy_token::parse!("&x" as Pattern).unwrap(), Pattern::Reference(_)));
    }

    #[test]
    fn lit_pattern() {
        assert!(matches!(moxy_token::parse!("42" as Pattern).unwrap(), Pattern::Lit(_)));
    }

    #[test]
    fn or_pattern() {
        assert!(matches!(moxy_token::parse!("A | B" as Pattern).unwrap(), Pattern::Or(_)));
    }
}
