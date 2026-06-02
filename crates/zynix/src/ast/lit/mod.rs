use crate::parse::{ParseError, ParseStream};
use crate::token::{self, LexError, ToTokens};
use crate::{Parse, Token, TokenStream, TokenTree};

mod lit_bool;
mod lit_byte;
mod lit_byte_str;
mod lit_c_str;
mod lit_char;
mod lit_float;
mod lit_int;
mod lit_str;

pub use lit_bool::*;
pub use lit_byte::*;
pub use lit_byte_str::*;
pub use lit_c_str::*;
pub use lit_char::*;
pub use lit_float::*;
pub use lit_int::*;
pub use lit_str::*;

#[doc = "A literal value in source code (string, integer, float, byte, char, or boolean)."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum Lit {
    Str(LitStr),
    ByteStr(LitByteStr),
    CStr(LitCStr),
    Byte(LitByte),
    Char(LitChar),
    Int(LitInt),
    Float(LitFloat),
    Bool(LitBool),
    Verbatim(token::Literal),
}

impl From<LitStr> for Lit {
    fn from(value: LitStr) -> Self {
        Lit::Str(value)
    }
}

impl From<LitByteStr> for Lit {
    fn from(value: LitByteStr) -> Self {
        Lit::ByteStr(value)
    }
}

impl From<LitCStr> for Lit {
    fn from(value: LitCStr) -> Self {
        Lit::CStr(value)
    }
}

impl From<LitByte> for Lit {
    fn from(value: LitByte) -> Self {
        Lit::Byte(value)
    }
}

impl From<LitChar> for Lit {
    fn from(value: LitChar) -> Self {
        Lit::Char(value)
    }
}

impl From<LitInt> for Lit {
    fn from(value: LitInt) -> Self {
        Lit::Int(value)
    }
}

impl From<LitFloat> for Lit {
    fn from(value: LitFloat) -> Self {
        Lit::Float(value)
    }
}

impl From<LitBool> for Lit {
    fn from(value: LitBool) -> Self {
        Lit::Bool(value)
    }
}

impl From<token::Literal> for Lit {
    fn from(value: token::Literal) -> Self {
        Lit::Verbatim(value)
    }
}

impl Parse for Lit {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let at = stream.span();

        match stream.advance() {
            Some(TokenTree::Token(Token::Literal(lit))) => {
                let span = lit.span();
                let repr = lit.repr().to_string();

                // Classify by repr prefix and build the matching variant.
                Ok(
                    if repr.starts_with("b\"") || repr.starts_with("br\"") || repr.starts_with("br#") {
                        Lit::ByteStr(LitByteStr { span, repr })
                    } else if repr.starts_with("c\"") || repr.starts_with("cr\"") || repr.starts_with("cr#") {
                        Lit::CStr(LitCStr { span, repr })
                    } else if repr.starts_with("b'") {
                        Lit::Byte(LitByte { span, repr })
                    } else if repr.starts_with('"') || repr.starts_with("r\"") || repr.starts_with("r#") {
                        Lit::Str(LitStr { span, repr })
                    } else if repr.starts_with('\'') {
                        Lit::Char(LitChar { span, repr })
                    } else if !repr.starts_with("0x")
                        && !repr.starts_with("0o")
                        && !repr.starts_with("0b")
                        && (repr.contains('.') || repr.contains('e') || repr.contains('E'))
                    {
                        Lit::Float(LitFloat { span, repr })
                    } else if repr.starts_with(|c: char| c.is_ascii_digit()) {
                        Lit::Int(LitInt { span, repr })
                    } else {
                        Lit::Verbatim(lit.clone())
                    },
                )
            }
            Some(TokenTree::Token(Token::Ident(id))) if id.name() == "true" || id.name() == "false" => Ok(Lit::Bool(LitBool {
                span: id.span(),
                value: id.name() == "true",
            })),
            _ => Err(LexError::new(at).message("expected literal").into()),
        }
    }
}

impl ToTokens for Lit {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Lit::Str(v) => v.to_tokens(tokens),
            Lit::ByteStr(v) => v.to_tokens(tokens),
            Lit::CStr(v) => v.to_tokens(tokens),
            Lit::Byte(v) => v.to_tokens(tokens),
            Lit::Char(v) => v.to_tokens(tokens),
            Lit::Int(v) => v.to_tokens(tokens),
            Lit::Float(v) => v.to_tokens(tokens),
            Lit::Bool(v) => v.to_tokens(tokens),
            Lit::Verbatim(v) => v.to_tokens(tokens),
        }
    }
}

impl std::fmt::Display for Lit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::token::ToTokenStream;
        write!(f, "{}", self.to_token_stream())
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use crate::Span;
    use crate::token::ToTokenStream;

    fn parse<T: Parse>(src: &str) -> Result<T, ParseError> {
        let ts = TokenStream::from_str(src).unwrap();
        ts.parse().parse::<T>()
    }

    fn lit(src: &str) -> Lit {
        parse::<Lit>(src).unwrap()
    }

    fn roundtrip(src: &str) -> String {
        lit(src).to_token_stream().to_string()
    }

    #[test]
    fn classifies() {
        assert!(matches!(lit("\"s\""), Lit::Str(_)));
        assert!(matches!(lit("42"), Lit::Int(_)));
        assert!(matches!(lit("1.5"), Lit::Float(_)));
        assert!(matches!(lit("'c'"), Lit::Char(_)));
        assert!(matches!(lit("b'x'"), Lit::Byte(_)));
        assert!(matches!(lit("b\"x\""), Lit::ByteStr(_)));
        assert!(matches!(lit("c\"x\""), Lit::CStr(_)));
        assert!(matches!(lit("true"), Lit::Bool(_)));
        assert!(matches!(lit("false"), Lit::Bool(_)));
    }

    #[test]
    fn bool_value() {
        assert!(matches!(lit("true"), Lit::Bool(LitBool { value: true, .. })));
        assert!(matches!(lit("false"), Lit::Bool(LitBool { value: false, .. })));
    }

    #[test]
    fn hex_oct_bin_are_ints() {
        assert!(matches!(lit("0xff"), Lit::Int(_)));
        assert!(matches!(lit("0o17"), Lit::Int(_)));
        assert!(matches!(lit("0b1010"), Lit::Int(_)));
    }

    #[test]
    fn roundtrips() {
        for src in ["\"s\"", "42", "1.5", "'c'", "true", "false", "0xff", "1_000usize"] {
            assert_eq!(roundtrip(src), src, "roundtrip mismatch for {src}");
        }
    }

    #[test]
    fn leaves_parse_their_own_kind() {
        assert!(parse::<LitInt>("42").is_ok());
        assert!(parse::<LitStr>("\"s\"").is_ok());
        assert!(parse::<LitFloat>("1.5").is_ok());
        assert!(parse::<LitChar>("'c'").is_ok());
        assert!(parse::<LitBool>("true").is_ok());
    }

    #[test]
    fn leaves_reject_other_kinds() {
        assert!(parse::<LitInt>("\"s\"").is_err());
        assert!(parse::<LitStr>("42").is_err());
        assert!(parse::<LitFloat>("42").is_err());
        assert!(parse::<LitInt>("1.5").is_err());
        assert!(parse::<LitBool>("42").is_err());
    }

    #[test]
    fn from_variant() {
        let s = LitStr {
            span: Span::default(),
            repr: "\"x\"".into(),
        };
        assert!(matches!(Lit::from(s), Lit::Str(_)));

        let b = LitBool {
            span: Span::default(),
            value: true,
        };
        assert!(matches!(Lit::from(b), Lit::Bool(_)));
    }
}
