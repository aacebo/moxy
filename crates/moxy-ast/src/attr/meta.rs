use moxy_token::{Group, Lit, Span, Spanner, ToTokens, TokenStream};

use crate::*;

/// Converts an attribute's parsed metadata into a caller-defined value.
///
/// Implement this trait for a type used with [`Attributed::parse_meta`]. The
/// method receives the full [`Meta`] node for the matching attribute, including
/// its path and content. Return a [`ParseError`] when that metadata does not
/// have the expected shape or values.
pub trait FromMeta: Sized {
    /// Converts a parsed metadata node into `Self`.
    fn from_meta(meta: &Meta) -> Result<Self, ParseError>;
}

/// A structured attribute meta item (`name`, `name(...)`, `name = expr`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Meta {
    pub path: Path,
    pub content: MetaContent,
}

impl Meta {
    pub fn for_each<P>(&self, mut parse: P) -> Result<(), ParseError>
    where
        P: FnMut(&Self) -> Result<(), ParseError>,
    {
        let MetaContent::List(group) = &self.content else {
            return ParseError::new(self.span(), "meta content must be a list to descend").into();
        };

        let parser = Parser::from_tokens(&group.tokens);

        while Path::peek(parser.cursor()) {
            let meta = <_ as Parse>::parse(&parser)?;
            parse(&meta)?;
        }

        Ok(())
    }

    /// Converts this metadata node through [`FromMeta`].
    ///
    /// This does not parse the metadata's inner tokens through [`Parse`]. Types
    /// accepted here, including nested values produced by `#[derive(Meta)]`,
    /// must implement [`FromMeta`].
    pub fn parse<T>(&self) -> Result<T, ParseError>
    where
        T: FromMeta,
    {
        T::from_meta(self)
    }
}

impl Spanner for Meta {
    fn span(&self) -> Span {
        self.path.span().join(self.content.span())
    }
}

impl ToTokens for Meta {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.path.to_tokens(tokens);
        self.content.to_tokens(tokens);
    }
}

impl Parse for Meta {
    fn peek(cursor: Cursor<'_>) -> bool {
        Path::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        Ok(Self {
            path: <_ as Parse>::parse(parser)?,
            content: <_ as Parse>::parse(parser)?,
        })
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        Path::skip(cursor)?;
        MetaContent::skip(cursor)
    }
}

/// The shape of a meta item after its path (`name`, `name = v`, `name(..)`, `name { .. }`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(untagged))]
pub enum MetaContent {
    /// `#[debug]`
    Unit,
    /// `#[debug(true, env = "test")]`
    List(Group),
    /// `#[debug = true]`
    Expr { eq: Token![=], expr: TokenStream },
}

impl Spanner for MetaContent {
    fn span(&self) -> Span {
        match self {
            Self::Unit => Default::default(),
            Self::List(v) => v.span(),
            Self::Expr { eq: _, expr } => expr.span(),
        }
    }
}

impl ToTokens for MetaContent {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Unit => {}
            Self::List(v) => v.to_tokens(tokens),
            Self::Expr { eq, expr } => {
                eq.to_tokens(tokens);
                expr.to_tokens(tokens);
            }
        }
    }
}

impl Parse for MetaContent {
    fn peek(cursor: Cursor<'_>) -> bool {
        if cursor.is_empty() || <Token![,]>::peek(cursor) {
            return true;
        }

        if <Token![=]>::peek(cursor) && !<Token![==]>::peek(cursor) && !<Token![=>]>::peek(cursor) && Expr::peek(cursor.offset(1))
        {
            return true;
        }

        Group::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        if parser.is_empty() || <Token![,]>::peek(parser.cursor()) {
            Ok(Self::Unit)
        } else if <Token![=]>::peek(parser.cursor())
            && !<Token![==]>::peek(parser.cursor())
            && !<Token![=>]>::peek(parser.cursor())
        {
            let eq = <_ as Parse>::parse(parser)?;
            let start = parser.cursor();
            let end = Expr::skip(parser.cursor()).unwrap_or(parser.cursor());

            parser.seek(&Parser::from_cursor(end));

            Ok(Self::Expr {
                eq,
                expr: end.range(start).into(),
            })
        } else {
            Ok(Self::List(<_ as Parse>::parse(parser)?))
        }
    }

    fn skip(cursor: Cursor<'_>) -> Option<Cursor<'_>> {
        if cursor.is_empty() || <Token![,]>::peek(cursor) {
            Some(cursor)
        } else if <Token![=]>::peek(cursor)
            && !<Token![==]>::peek(cursor)
            && !<Token![=>]>::peek(cursor)
            && Expr::peek(cursor.offset(1))
        {
            <Token![=]>::skip(cursor)?;
            Expr::skip(cursor)
        } else if Group::peek(cursor) {
            Group::skip(cursor)
        } else {
            None
        }
    }
}

impl<T: FromMeta> FromMeta for Box<T> {
    fn from_meta(meta: &Meta) -> Result<Self, ParseError> {
        Ok(Self::new(T::from_meta(meta)?))
    }
}

impl<T: FromMeta> FromMeta for std::rc::Rc<T> {
    fn from_meta(meta: &Meta) -> Result<Self, ParseError> {
        Ok(Self::new(T::from_meta(meta)?))
    }
}

impl<T: FromMeta> FromMeta for std::sync::Arc<T> {
    fn from_meta(meta: &Meta) -> Result<Self, ParseError> {
        Ok(Self::new(T::from_meta(meta)?))
    }
}

impl<T: FromMeta> FromMeta for Option<T> {
    fn from_meta(meta: &Meta) -> Result<Self, ParseError> {
        match T::from_meta(meta) {
            Err(_) => Ok(Self::None),
            Ok(v) => Ok(Self::Some(v)),
        }
    }
}

impl FromMeta for bool {
    fn from_meta(meta: &Meta) -> Result<Self, ParseError> {
        match &meta.content {
            MetaContent::Unit => Ok(true),
            MetaContent::Expr { expr, .. } => {
                let Expr::Lit(ExprLit {
                    lit: Lit::Bool(value), ..
                }) = Expr::parse(&Parser::from_tokens(expr))?
                else {
                    return ParseError::new(meta.span(), "expected bool").into();
                };
                Ok(value.value())
            }
            _ => ParseError::new(meta.span(), "expected bool").into(),
        }
    }
}

impl FromMeta for String {
    fn from_meta(meta: &Meta) -> Result<Self, ParseError> {
        let MetaContent::Expr { expr, .. } = &meta.content else {
            return ParseError::new(meta.span(), "expected string").into();
        };
        let Expr::Lit(ExprLit {
            lit: Lit::Str(value), ..
        }) = Expr::parse(&Parser::from_tokens(expr))?
        else {
            return ParseError::new(meta.span(), "expected string").into();
        };
        Ok(value.value().to_owned())
    }
}

impl FromMeta for char {
    fn from_meta(meta: &Meta) -> Result<Self, ParseError> {
        let MetaContent::Expr { expr, .. } = &meta.content else {
            return ParseError::new(meta.span(), "expected char").into();
        };
        let Expr::Lit(ExprLit {
            lit: Lit::Char(value), ..
        }) = Expr::parse(&Parser::from_tokens(expr))?
        else {
            return ParseError::new(meta.span(), "expected char").into();
        };
        Ok(value.value())
    }
}

impl FromMeta for u8 {
    fn from_meta(meta: &Meta) -> Result<Self, ParseError> {
        let MetaContent::Expr { expr, .. } = &meta.content else {
            return ParseError::new(meta.span(), "expected integer").into();
        };
        let Expr::Lit(ExprLit {
            lit: Lit::Int(value), ..
        }) = Expr::parse(&Parser::from_tokens(expr))?
        else {
            return ParseError::new(meta.span(), "expected integer").into();
        };
        let value = value.value();
        u8::try_from(value).map_err(|_| ParseError::new(meta.span(), "integer out of range for u8"))
    }
}

impl FromMeta for u16 {
    fn from_meta(meta: &Meta) -> Result<Self, ParseError> {
        let MetaContent::Expr { expr, .. } = &meta.content else {
            return ParseError::new(meta.span(), "expected integer").into();
        };
        let Expr::Lit(ExprLit {
            lit: Lit::Int(value), ..
        }) = Expr::parse(&Parser::from_tokens(expr))?
        else {
            return ParseError::new(meta.span(), "expected integer").into();
        };
        let value = value.value();
        u16::try_from(value).map_err(|_| ParseError::new(meta.span(), "integer out of range for u16"))
    }
}

impl FromMeta for u32 {
    fn from_meta(meta: &Meta) -> Result<Self, ParseError> {
        let MetaContent::Expr { expr, .. } = &meta.content else {
            return ParseError::new(meta.span(), "expected integer").into();
        };
        let Expr::Lit(ExprLit {
            lit: Lit::Int(value), ..
        }) = Expr::parse(&Parser::from_tokens(expr))?
        else {
            return ParseError::new(meta.span(), "expected integer").into();
        };
        let value = value.value();
        u32::try_from(value).map_err(|_| ParseError::new(meta.span(), "integer out of range for u32"))
    }
}

impl FromMeta for u64 {
    fn from_meta(meta: &Meta) -> Result<Self, ParseError> {
        let MetaContent::Expr { expr, .. } = &meta.content else {
            return ParseError::new(meta.span(), "expected integer").into();
        };
        let Expr::Lit(ExprLit {
            lit: Lit::Int(value), ..
        }) = Expr::parse(&Parser::from_tokens(expr))?
        else {
            return ParseError::new(meta.span(), "expected integer").into();
        };
        let value = value.value();
        u64::try_from(value).map_err(|_| ParseError::new(meta.span(), "integer out of range for u64"))
    }
}

impl FromMeta for u128 {
    fn from_meta(meta: &Meta) -> Result<Self, ParseError> {
        let MetaContent::Expr { expr, .. } = &meta.content else {
            return ParseError::new(meta.span(), "expected integer").into();
        };
        let Expr::Lit(ExprLit {
            lit: Lit::Int(value), ..
        }) = Expr::parse(&Parser::from_tokens(expr))?
        else {
            return ParseError::new(meta.span(), "expected integer").into();
        };
        Ok(value.value())
    }
}

impl FromMeta for usize {
    fn from_meta(meta: &Meta) -> Result<Self, ParseError> {
        let MetaContent::Expr { expr, .. } = &meta.content else {
            return ParseError::new(meta.span(), "expected integer").into();
        };
        let Expr::Lit(ExprLit {
            lit: Lit::Int(value), ..
        }) = Expr::parse(&Parser::from_tokens(expr))?
        else {
            return ParseError::new(meta.span(), "expected integer").into();
        };
        let value = value.value();
        usize::try_from(value).map_err(|_| ParseError::new(meta.span(), "integer out of range for usize"))
    }
}

impl FromMeta for i8 {
    fn from_meta(meta: &Meta) -> Result<Self, ParseError> {
        let MetaContent::Expr { expr, .. } = &meta.content else {
            return ParseError::new(meta.span(), "expected integer").into();
        };
        let expr = Expr::parse(&Parser::from_tokens(expr))?;
        let value = match expr {
            Expr::Lit(ExprLit {
                lit: Lit::Int(value), ..
            }) => i128::try_from(value.value()).ok(),
            Expr::Unary(ExprUnary {
                op: UnOp::Neg(_), expr, ..
            }) => match *expr {
                Expr::Lit(ExprLit {
                    lit: Lit::Int(value), ..
                }) => i128::try_from(value.value()).ok().and_then(i128::checked_neg),
                _ => None,
            },
            _ => None,
        }
        .ok_or_else(|| ParseError::new(meta.span(), "expected integer"))?;
        i8::try_from(value).map_err(|_| ParseError::new(meta.span(), "integer out of range for i8"))
    }
}

impl FromMeta for i16 {
    fn from_meta(meta: &Meta) -> Result<Self, ParseError> {
        let MetaContent::Expr { expr, .. } = &meta.content else {
            return ParseError::new(meta.span(), "expected integer").into();
        };
        let expr = Expr::parse(&Parser::from_tokens(expr))?;
        let value = match expr {
            Expr::Lit(ExprLit {
                lit: Lit::Int(value), ..
            }) => i128::try_from(value.value()).ok(),
            Expr::Unary(ExprUnary {
                op: UnOp::Neg(_), expr, ..
            }) => match *expr {
                Expr::Lit(ExprLit {
                    lit: Lit::Int(value), ..
                }) => i128::try_from(value.value()).ok().and_then(i128::checked_neg),
                _ => None,
            },
            _ => None,
        }
        .ok_or_else(|| ParseError::new(meta.span(), "expected integer"))?;
        i16::try_from(value).map_err(|_| ParseError::new(meta.span(), "integer out of range for i16"))
    }
}

impl FromMeta for i32 {
    fn from_meta(meta: &Meta) -> Result<Self, ParseError> {
        let MetaContent::Expr { expr, .. } = &meta.content else {
            return ParseError::new(meta.span(), "expected integer").into();
        };
        let expr = Expr::parse(&Parser::from_tokens(expr))?;
        let value = match expr {
            Expr::Lit(ExprLit {
                lit: Lit::Int(value), ..
            }) => i128::try_from(value.value()).ok(),
            Expr::Unary(ExprUnary {
                op: UnOp::Neg(_), expr, ..
            }) => match *expr {
                Expr::Lit(ExprLit {
                    lit: Lit::Int(value), ..
                }) => i128::try_from(value.value()).ok().and_then(i128::checked_neg),
                _ => None,
            },
            _ => None,
        }
        .ok_or_else(|| ParseError::new(meta.span(), "expected integer"))?;
        i32::try_from(value).map_err(|_| ParseError::new(meta.span(), "integer out of range for i32"))
    }
}

impl FromMeta for i64 {
    fn from_meta(meta: &Meta) -> Result<Self, ParseError> {
        let MetaContent::Expr { expr, .. } = &meta.content else {
            return ParseError::new(meta.span(), "expected integer").into();
        };
        let expr = Expr::parse(&Parser::from_tokens(expr))?;
        let value = match expr {
            Expr::Lit(ExprLit {
                lit: Lit::Int(value), ..
            }) => i128::try_from(value.value()).ok(),
            Expr::Unary(ExprUnary {
                op: UnOp::Neg(_), expr, ..
            }) => match *expr {
                Expr::Lit(ExprLit {
                    lit: Lit::Int(value), ..
                }) => i128::try_from(value.value()).ok().and_then(i128::checked_neg),
                _ => None,
            },
            _ => None,
        }
        .ok_or_else(|| ParseError::new(meta.span(), "expected integer"))?;
        i64::try_from(value).map_err(|_| ParseError::new(meta.span(), "integer out of range for i64"))
    }
}

impl FromMeta for i128 {
    fn from_meta(meta: &Meta) -> Result<Self, ParseError> {
        let MetaContent::Expr { expr, .. } = &meta.content else {
            return ParseError::new(meta.span(), "expected integer").into();
        };
        let expr = Expr::parse(&Parser::from_tokens(expr))?;
        match expr {
            Expr::Lit(ExprLit {
                lit: Lit::Int(value), ..
            }) => i128::try_from(value.value()).map_err(|_| ParseError::new(meta.span(), "integer out of range for i128")),
            Expr::Unary(ExprUnary {
                op: UnOp::Neg(_), expr, ..
            }) => {
                let Expr::Lit(ExprLit {
                    lit: Lit::Int(value), ..
                }) = *expr
                else {
                    return ParseError::new(meta.span(), "expected integer").into();
                };
                if value.value() == i128::MAX as u128 + 1 {
                    Ok(i128::MIN)
                } else {
                    i128::try_from(value.value())
                        .ok()
                        .and_then(i128::checked_neg)
                        .ok_or_else(|| ParseError::new(meta.span(), "integer out of range for i128"))
                }
            }
            _ => ParseError::new(meta.span(), "expected integer").into(),
        }
    }
}

impl FromMeta for isize {
    fn from_meta(meta: &Meta) -> Result<Self, ParseError> {
        let MetaContent::Expr { expr, .. } = &meta.content else {
            return ParseError::new(meta.span(), "expected integer").into();
        };
        let expr = Expr::parse(&Parser::from_tokens(expr))?;
        let value = match expr {
            Expr::Lit(ExprLit {
                lit: Lit::Int(value), ..
            }) => i128::try_from(value.value()).ok(),
            Expr::Unary(ExprUnary {
                op: UnOp::Neg(_), expr, ..
            }) => match *expr {
                Expr::Lit(ExprLit {
                    lit: Lit::Int(value), ..
                }) => i128::try_from(value.value()).ok().and_then(i128::checked_neg),
                _ => None,
            },
            _ => None,
        }
        .ok_or_else(|| ParseError::new(meta.span(), "expected integer"))?;
        isize::try_from(value).map_err(|_| ParseError::new(meta.span(), "integer out of range for isize"))
    }
}

impl FromMeta for f32 {
    fn from_meta(meta: &Meta) -> Result<Self, ParseError> {
        let MetaContent::Expr { expr, .. } = &meta.content else {
            return ParseError::new(meta.span(), "expected float").into();
        };
        let expr = Expr::parse(&Parser::from_tokens(expr))?;
        let (negative, expr) = match expr {
            Expr::Unary(ExprUnary {
                op: UnOp::Neg(_), expr, ..
            }) => (true, *expr),
            expr => (false, expr),
        };
        let Expr::Lit(ExprLit {
            lit: Lit::Float(value), ..
        }) = expr
        else {
            return ParseError::new(meta.span(), "expected float").into();
        };
        let value = if negative { -value.as_f64() } else { value.as_f64() };
        if !value.is_finite() || value < f32::MIN as f64 || value > f32::MAX as f64 {
            return ParseError::new(meta.span(), "float out of range for f32").into();
        }
        Ok(value as f32)
    }
}

impl FromMeta for f64 {
    fn from_meta(meta: &Meta) -> Result<Self, ParseError> {
        let MetaContent::Expr { expr, .. } = &meta.content else {
            return ParseError::new(meta.span(), "expected float").into();
        };
        let expr = Expr::parse(&Parser::from_tokens(expr))?;
        let (negative, expr) = match expr {
            Expr::Unary(ExprUnary {
                op: UnOp::Neg(_), expr, ..
            }) => (true, *expr),
            expr => (false, expr),
        };
        let Expr::Lit(ExprLit {
            lit: Lit::Float(value), ..
        }) = expr
        else {
            return ParseError::new(meta.span(), "expected float").into();
        };
        let value = if negative { -value.as_f64() } else { value.as_f64() };
        value
            .is_finite()
            .then_some(value)
            .ok_or_else(|| ParseError::new(meta.span(), "float out of range for f64"))
    }
}
