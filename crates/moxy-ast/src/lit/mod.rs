use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::{LexError, Parse, Span, Spanner, ToTokens, Token, TokenStream, TokenTree};

mod lit_bool;
mod lit_byte;
mod lit_byte_str;
mod lit_c_str;
mod lit_char;
mod lit_float;
mod lit_int;
mod lit_str;

#[doc(inline)]
pub use lit_bool::*;
#[doc(inline)]
pub use lit_byte::*;
#[doc(inline)]
pub use lit_byte_str::*;
#[doc(inline)]
pub use lit_c_str::*;
#[doc(inline)]
pub use lit_char::*;
#[doc(inline)]
pub use lit_float::*;
#[doc(inline)]
pub use lit_int::*;
#[doc(inline)]
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
    Verbatim(moxy_token::Literal),
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

impl From<moxy_token::Literal> for Lit {
    fn from(value: moxy_token::Literal) -> Self {
        Lit::Verbatim(value)
    }
}

impl Spanner for Lit {
    fn span(&self) -> Span {
        match self {
            Lit::Str(v) => v.span(),
            Lit::ByteStr(v) => v.span(),
            Lit::CStr(v) => v.span(),
            Lit::Byte(v) => v.span(),
            Lit::Char(v) => v.span(),
            Lit::Int(v) => v.span(),
            Lit::Float(v) => v.span(),
            Lit::Bool(v) => v.span(),
            Lit::Verbatim(v) => v.span(),
        }
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
        use moxy_token::ToTokenStream;
        write!(f, "{}", self.to_token_stream())
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use moxy_token::{Span, ToTokenStream};

    use super::*;

    fn roundtrip(src: &str) -> String {
        let l: Lit = moxy_token::parse!(src).unwrap();
        l.to_token_stream().to_string()
    }

    #[test]
    fn classifies() {
        assert!(matches!(moxy_token::parse!("\"s\"" as Lit).unwrap(), Lit::Str(_)));
        assert!(matches!(moxy_token::parse!("42" as Lit).unwrap(), Lit::Int(_)));
        assert!(matches!(moxy_token::parse!("1.5" as Lit).unwrap(), Lit::Float(_)));
        assert!(matches!(moxy_token::parse!("'c'" as Lit).unwrap(), Lit::Char(_)));
        assert!(matches!(moxy_token::parse!("b'x'" as Lit).unwrap(), Lit::Byte(_)));
        assert!(matches!(moxy_token::parse!("b\"x\"" as Lit).unwrap(), Lit::ByteStr(_)));
        assert!(matches!(moxy_token::parse!("c\"x\"" as Lit).unwrap(), Lit::CStr(_)));
        assert!(matches!(moxy_token::parse!("true" as Lit).unwrap(), Lit::Bool(_)));
        assert!(matches!(moxy_token::parse!("false" as Lit).unwrap(), Lit::Bool(_)));
    }

    #[test]
    fn bool_value() {
        assert!(matches!(
            moxy_token::parse!("true" as Lit).unwrap(),
            Lit::Bool(LitBool { value: true, .. })
        ));
        assert!(matches!(
            moxy_token::parse!("false" as Lit).unwrap(),
            Lit::Bool(LitBool { value: false, .. })
        ));
    }

    #[test]
    fn hex_oct_bin_are_ints() {
        assert!(matches!(moxy_token::parse!("0xff" as Lit).unwrap(), Lit::Int(_)));
        assert!(matches!(moxy_token::parse!("0o17" as Lit).unwrap(), Lit::Int(_)));
        assert!(matches!(moxy_token::parse!("0b1010" as Lit).unwrap(), Lit::Int(_)));
    }

    #[test]
    fn roundtrips() {
        for src in ["\"s\"", "42", "1.5", "'c'", "true", "false", "0xff", "1_000usize"] {
            assert_eq!(roundtrip(src), src, "roundtrip mismatch for {src}");
        }
    }

    #[test]
    fn leaves_parse_their_own_kind() {
        assert!(moxy_token::parse!("42" as LitInt).is_ok());
        assert!(moxy_token::parse!("\"s\"" as LitStr).is_ok());
        assert!(moxy_token::parse!("1.5" as LitFloat).is_ok());
        assert!(moxy_token::parse!("'c'" as LitChar).is_ok());
        assert!(moxy_token::parse!("true" as LitBool).is_ok());
    }

    #[test]
    fn leaves_reject_other_kinds() {
        assert!(moxy_token::parse!("\"s\"" as LitInt).is_err());
        assert!(moxy_token::parse!("42" as LitStr).is_err());
        assert!(moxy_token::parse!("42" as LitFloat).is_err());
        assert!(moxy_token::parse!("1.5" as LitInt).is_err());
        assert!(moxy_token::parse!("42" as LitBool).is_err());
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
