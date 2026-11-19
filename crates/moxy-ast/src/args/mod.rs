use moxy_token::{Group, Ident, Punct, Span, Spanner, ToTokens, TokenStream, TokenTree};

use crate::*;

mod angle_arguments;
mod assoc_const_argument;
mod assoc_type_argument;
mod constraint_argument;
mod paren_arguments;

pub use angle_arguments::*;
pub use assoc_const_argument::*;
pub use assoc_type_argument::*;
pub use constraint_argument::*;
pub use paren_arguments::*;

/// A single generic argument inside `<...>`.
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum GenericArgument {
    Lifetime(Lifetime),
    Type(Type),
    Const(Expr),
    AssocType(AssocTypeArgument),
    AssocConst(AssocConstArgument),
    Constraint(ConstraintArgument),
}

impl Spanner for GenericArgument {
    fn span(&self) -> Span {
        match self {
            Self::Lifetime(v) => v.span(),
            Self::Type(v) => v.span(),
            Self::Const(v) => v.span(),
            Self::AssocType(v) => v.span(),
            Self::AssocConst(v) => v.span(),
            Self::Constraint(v) => v.span(),
        }
    }
}

impl Parse for GenericArgument {
    fn peek(cursor: Cursor<'_>) -> bool {
        if Lifetime::peek(cursor) {
            return true;
        }

        let is_const = starts_const(cursor);

        if is_const {
            return true;
        }

        Type::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let _depth = moxy_token::Depth::enter().ok_or_else(|| ParseError::too_deep(parser.span()))?;

        let Some(token) = parser.cursor().curr() else {
            return Err(ParseError::new(parser.span(), "eof"));
        };

        // One call site for every form keeps one return slot in this frame,
        // which every nested argument holds
        let parse: fn(&Parser) -> Result<Self, ParseError> = if token.is_punct_quote() {
            |parser| Ok(Self::Lifetime(<_ as Parse>::parse(parser)?))
        } else if starts_const(parser.cursor()) {
            // A literal or block expression
            |parser| Ok(Self::Const(expr::parse::unary(parser, Default::default())?))
        } else if token.is_ident() {
            Self::parse_named
        } else {
            |parser| Ok(Self::Type(<_ as Parse>::parse(parser)?))
        };

        parse(parser)
    }

    fn skip(mut cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        let _depth = moxy_token::Depth::enter()?;

        if Lifetime::peek(cursor) {
            return Lifetime::skip(cursor);
        }

        let is_const = starts_const(cursor);

        if is_const {
            return expr::skip::pattern_bound(cursor);
        }

        if Ident::peek(cursor) {
            cursor = Ident::skip(cursor)?;

            if ParenArguments::peek(cursor) {
                return Path::skip_rest(ParenArguments::skip(cursor)?);
            }

            cursor = Option::<AngleArguments>::skip(cursor)?;

            if peek_lone_colon(cursor) {
                cursor = <Token![:]>::skip(cursor)?;
                return List::<TypeBound, Token![+]>::skip_while_present(cursor);
            }

            if <Token![=]>::peek(cursor) {
                cursor = <Token![=]>::skip(cursor)?;

                let is_const = starts_const(cursor);

                return if is_const {
                    expr::skip::pattern_bound(cursor)
                } else {
                    Type::skip(cursor)
                };
            }

            cursor = Path::skip_rest(cursor)?;

            if <Token![+]>::peek(cursor) {
                return TypeBound::skip_bounds_after(cursor);
            }

            if <Token![!]>::peek(cursor) {
                cursor = <Token![!]>::skip(cursor)?;
                cursor = Group::skip(cursor)?;
            }

            return Some(cursor);
        }

        Type::skip(cursor)
    }
}

impl GenericArgument {
    // An argument that starts with an identifier: a constraint, an associated
    // type or constant, or a path type. Its own stack frame keeps these forms
    // out of the frame of every nested argument.
    fn parse_named(parser: &Parser) -> Result<Self, ParseError> {
        let ident = <_ as Parse>::parse(parser)?;

        // A bare trait object with parenthesized arguments, as in `Box<Fn(u8)>`
        if ParenArguments::peek(parser.cursor()) {
            return Self::parse_paren_path(parser, ident);
        }

        let generics: Option<AngleArguments> = <_ as Parse>::parse(parser)?;
        Self::parse_named_rest(parser, ident, generics)
    }

    fn parse_paren_path(parser: &Parser, ident: Ident) -> Result<Self, ParseError> {
        let args = PathArguments::Parenthesized(<_ as Parse>::parse(parser)?);
        let path = Path::parse_rest(parser, PathSegment { ident, args })?;
        Ok(Self::Type(Type::Path(TypePath { qself: None, path })))
    }

    // The form after a named argument's name and generic arguments, in its own
    // frame, which the nested generic arguments do not hold
    fn parse_named_rest(parser: &Parser, ident: Ident, generics: Option<AngleArguments>) -> Result<Self, ParseError> {
        if peek_lone_colon(parser.cursor()) {
            return Self::parse_constraint(parser, ident, generics);
        }

        if <Token![=]>::peek(parser.cursor()) {
            return Self::parse_assoc(parser, ident, generics);
        }

        Self::parse_path(parser, ident, generics)
    }

    // An associated type bound takes three levels of stack, as in
    // `A<B: A<B: C>>`
    fn parse_constraint(parser: &Parser, ident: Ident, generics: Option<AngleArguments>) -> Result<Self, ParseError> {
        let _weight = moxy_token::Depth::enter_by(2).ok_or_else(|| ParseError::too_deep(parser.span()))?;
        let colon_punct = <_ as Parse>::parse(parser)?;
        let bounds = List::parse_while_present(parser)?;

        Ok(ConstraintArgument {
            ident,
            generics,
            colon_punct,
            bounds,
        }
        .into_generic_argument())
    }

    // An associated type or constant: `Item = u8`, `N = 3`
    fn parse_assoc(parser: &Parser, ident: Ident, generics: Option<AngleArguments>) -> Result<Self, ParseError> {
        // Another level of stack, as in `A<B = A<B = u8>>`
        let _weight = moxy_token::Depth::enter_by(1).ok_or_else(|| ParseError::too_deep(parser.span()))?;
        let eq_punct = <_ as Parse>::parse(parser)?;
        let is_const = starts_const(parser.cursor());

        if is_const {
            let expr = expr::parse::unary(parser, Default::default())?;

            return Ok(AssocConstArgument {
                ident,
                generics,
                eq_punct,
                expr,
            }
            .into_generic_argument());
        }

        let ty = <_ as Parse>::parse(parser)?;

        Ok(AssocTypeArgument {
            ident,
            generics,
            eq_punct,
            ty,
        }
        .into_generic_argument())
    }

    // A path type or type macro that starts with `ident` and its arguments
    fn parse_path(parser: &Parser, ident: Ident, generics: Option<AngleArguments>) -> Result<Self, ParseError> {
        let args = generics.map_or(PathArguments::None, PathArguments::AngleBracketed);
        let path = Path::parse_rest(parser, PathSegment { ident, args })?;
        let ty = if <Token![+]>::peek(parser.cursor()) {
            Type::parse_bare_trait_object(parser, path)?
        } else if <Token![!]>::peek(parser.cursor()) {
            Type::Macro(TypeMacro {
                mac: MacroCall {
                    path,
                    bang: <_ as Parse>::parse(parser)?,
                    body: <_ as Parse>::parse(parser)?,
                },
            })
        } else {
            Type::Path(TypePath { qself: None, path })
        };

        Ok(Self::Type(ty))
    }
}

impl ToTokens for GenericArgument {
    fn to_tokens(&self, t: &mut TokenStream) {
        match self {
            Self::Lifetime(v) => v.to_tokens(t),
            Self::Type(v) => v.to_tokens(t),
            Self::Const(v) => v.to_tokens(t),
            Self::AssocType(v) => v.to_tokens(t),
            Self::AssocConst(v) => v.to_tokens(t),
            Self::Constraint(v) => v.to_tokens(t),
        }
    }
}

// Whether a const argument starts here: a literal, a block, or a negated
// literal. A `!` starts the never type, as in `Result<T, !>`.
fn starts_const(cursor: Cursor<'_>) -> bool {
    match cursor.curr() {
        Some(TokenTree::Literal(_) | TokenTree::Punct(Punct::Minus(_))) => true,
        Some(TokenTree::Group(group)) => group.delim.is_brace(),
        _ => false,
    }
}
