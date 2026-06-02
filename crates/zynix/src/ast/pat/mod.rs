use crate::ast::{Attribute, Expr, Ident, Member, Mutability, Path, Punctuated, RangeLimits, Type};
use crate::parse::{ParseError, ParseStream};
use crate::token::keyword::{Mut, Ref};
use crate::token::punct::{And, At, Colon, Comma, DotDot, Or as OrPunct};
use crate::token::{Delim, LexError, Punctuation, ToTokens};
use crate::{Parse, Span, Token, TokenStream, TokenTree};

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

#[doc = "A Rust pattern (in `let`, `match`, function params, etc.)."]
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
    Range(PatRange),
    Macro(crate::ast::MacroCall),
    Type(PatType),
    Group(PatGroup),
    Paren(PatParen),
    Box(Box<Pattern>),
    Const(crate::ast::StmtBlock),
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
        Pattern::Range(value)
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
        let leading = stream.peek::<OrPunct>().is_some();

        if leading {
            let _ = stream.parse::<OrPunct>()?;
        }

        let first = parse_single(stream)?;

        if !leading && stream.peek::<OrPunct>().is_none() {
            return Ok(first);
        }

        let mut cases = Punctuated::new();
        cases.push_value(first);

        while stream.peek::<OrPunct>().is_some() {
            cases.push_punct(stream.parse::<OrPunct>()?);
            cases.push_value(parse_single(stream)?);
        }

        Ok(Pattern::Or(PatOr {
            span: Span::default(),
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
                crate::token::Ident::new("_", Span::default()).to_tokens(t);
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
                crate::token::keyword::Box::default().to_tokens(t);
                p.to_tokens(t);
            }
            Pattern::Const(b) => {
                crate::token::keyword::Const::default().to_tokens(t);
                b.to_tokens(t);
            }
        }
    }
}

impl PatIdent {
    pub fn parse_from(stream: &mut ParseStream, attrs: Vec<Attribute>) -> Result<Self, ParseError> {
        let by_ref = if stream.peek::<Ref>().is_some() {
            let _ = stream.parse::<Ref>()?;
            true
        } else {
            false
        };

        let mutability = stream.parse::<Mutability>()?;
        let ident = stream.parse::<Ident>()?;

        let subpat = if stream.peek::<At>().is_some() {
            let _ = stream.parse::<At>()?;
            Some(Box::new(Pattern::parse(stream)?))
        } else {
            None
        };

        Ok(Self {
            span: Span::default(),
            attrs,
            by_ref,
            mutability,
            ident,
            subpat,
        })
    }
}

impl PatStruct {
    pub fn parse_body(stream: &mut ParseStream) -> Result<(Punctuated<PatField, Comma>, bool), ParseError> {
        let mut fields = Punctuated::new();
        let mut rest = false;

        while !stream.is_empty() {
            if stream.peek::<DotDot>().is_some() {
                let _ = stream.parse::<DotDot>()?;
                rest = true;
                break;
            }

            let member = stream.parse::<Member>()?;

            let (pat, shorthand) = if stream.peek::<Colon>().is_some() {
                let _ = stream.parse::<Colon>()?;
                (stream.parse::<Pattern>()?, false)
            } else {
                // shorthand `{ field }`
                let ident = match &member {
                    Member::Named(id) => id.clone(),
                    Member::Unnamed(_) => {
                        return Err(LexError::new(stream.span()).message("tuple index needs a pattern").into());
                    }
                };
                (
                    Pattern::Ident(PatIdent {
                        span: Span::default(),
                        attrs: Vec::new(),
                        by_ref: false,
                        mutability: Mutability::Immutable,
                        ident,
                        subpat: None,
                    }),
                    true,
                )
            };

            fields.push_value(PatField {
                span: Span::default(),
                attrs: Vec::new(),
                member,
                pat,
                shorthand,
            });

            if stream.peek::<Comma>().is_some() {
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
    let attrs = stream.parse_vec::<Attribute>()?;

    // Wildcard `_`
    if matches!(stream.curr(), Some(tt) if tt.name().as_deref() == Some("_")) {
        stream.advance();
        return Ok(Pattern::Wild);
    }

    // Rest `..`
    if stream.peek::<DotDot>().is_some() {
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
        && matches!(stream.nth(1), Some(crate::TokenTree::Group(g)) if g.delim() == Delim::Brace)
    {
        stream.advance();
        return Ok(Pattern::Const(stream.parse::<crate::ast::StmtBlock>()?));
    }

    // Reference `&`/`&mut`
    if stream.peek::<And>().is_some() {
        let _ = stream.parse::<And>()?;
        let mutability = stream.parse::<Mutability>()?;
        let pat = Box::new(Pattern::parse(stream)?);
        return Ok(Pattern::Reference(PatReference {
            span: Span::default(),
            attrs,
            mutability,
            pat,
        }));
    }

    // Tuple/paren `(...)`
    if matches!(stream.curr(), Some(tt) if tt.delim() == Some(Delim::Paren)) {
        let group = stream.parse_group(Delim::Paren)?;
        let mut inner = group.parse();
        let elems = Punctuated::parse_terminated(&mut inner)?;
        return Ok(Pattern::Tuple(PatTuple {
            span: Span::default(),
            attrs,
            elems,
        }));
    }

    // Slice `[...]`
    if matches!(stream.curr(), Some(tt) if tt.delim() == Some(Delim::Bracket)) {
        let group = stream.parse_group(Delim::Bracket)?;
        let mut inner = group.parse();
        let elems = Punctuated::parse_terminated(&mut inner)?;
        return Ok(Pattern::Slice(PatSlice {
            span: Span::default(),
            attrs,
            elems,
        }));
    }

    // `ref`/`mut`-led binding
    if stream.peek::<Ref>().is_some() || stream.peek::<Mut>().is_some() {
        return Ok(Pattern::Ident(PatIdent::parse_from(stream, attrs)?));
    }

    // Literal pattern
    if matches!(stream.curr(), Some(tt) if matches!(tt, TokenTree::Token(Token::Literal(_)))) {
        let expr = stream.parse::<Expr>()?;
        return Ok(Pattern::Lit(PatLit {
            span: Span::default(),
            attrs,
            expr,
        }));
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
            let group = stream.parse_group(Delim::Paren)?;
            let mut inner = group.parse();
            let elems = Punctuated::parse_terminated(&mut inner)?;
            return Ok(Pattern::TupleStruct(PatTupleStruct {
                span: Span::default(),
                attrs,
                qself: None,
                path,
                elems,
            }));
        }

        if matches!(fork.curr(), Some(tt) if tt.delim() == Some(Delim::Brace)) {
            stream.seek(&fork);
            let group = stream.parse_group(Delim::Brace)?;
            let mut inner = group.parse();
            let (fields, rest) = PatStruct::parse_body(&mut inner)?;
            return Ok(Pattern::Struct(PatStruct {
                span: Span::default(),
                attrs,
                qself: None,
                path,
                fields,
                rest,
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
                span: Span::default(),
                attrs,
                by_ref: false,
                mutability: Mutability::Immutable,
                ident,
                subpat: None,
            }));
        }

        stream.seek(&fork);
        return Ok(Pattern::Path(PatPath {
            span: Span::default(),
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

    use super::*;
    use crate::token::ToTokenStream;

    fn parse_pat(src: &str) -> Result<Pattern, ParseError> {
        let ts = TokenStream::from_str(src).unwrap();
        ts.parse().parse::<Pattern>()
    }

    fn pat(src: &str) -> Pattern {
        parse_pat(src).unwrap()
    }

    fn roundtrip(src: &str) -> String {
        pat(src).to_token_stream().to_string()
    }

    #[test]
    fn wildcard() {
        assert!(matches!(pat("_"), Pattern::Wild));
        assert_eq!(roundtrip("_"), "_");
    }

    #[test]
    fn rest() {
        assert!(matches!(pat(".."), Pattern::Rest));
        assert_eq!(roundtrip(".."), "..");
    }

    #[test]
    fn ident_binding() {
        assert!(matches!(pat("x"), Pattern::Ident(_)));
        assert_eq!(roundtrip("x"), "x");
    }

    #[test]
    fn mut_binding() {
        assert!(matches!(pat("mut x"), Pattern::Ident(_)));
    }

    #[test]
    fn ref_binding() {
        assert!(matches!(pat("ref x"), Pattern::Ident(_)));
    }

    #[test]
    fn tuple_pattern() {
        assert!(matches!(pat("(a, b)"), Pattern::Tuple(_)));
    }

    #[test]
    fn slice_pattern() {
        assert!(matches!(pat("[a, b]"), Pattern::Slice(_)));
    }

    #[test]
    fn reference_pattern() {
        assert!(matches!(pat("&x"), Pattern::Reference(_)));
    }

    #[test]
    fn lit_pattern() {
        assert!(matches!(pat("42"), Pattern::Lit(_)));
    }

    #[test]
    fn or_pattern() {
        assert!(matches!(pat("A | B"), Pattern::Or(_)));
    }
}
