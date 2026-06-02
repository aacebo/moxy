use crate::parse::{ParseError, ParseStream};
use crate::token::{LexError, ToTokenStream, ToTokens};
use crate::{Parse, TokenStream};

macro_rules! define_leaf {
    ($(
        $(#[doc = $doc:literal])?
        pub enum $name:ident {
            $($variant:ident $(=> $token:ty)?),+ $(,)?
        }
    )+) => {
        $(
            $(#[doc = $doc])?
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize))]
            pub enum $name {
                $($variant,)+
            }

            impl Parse for $name {
                #[allow(unreachable_code)]
                fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
                    $(
                        define_leaf!(@parse_arm stream, Self::$variant $(=> $token)?);
                    )+

                    Err(LexError::new(stream.span())
                        .message(concat!("expected `", stringify!($name), "`"))
                        .into())
                }
            }

            impl ToTokens for $name {
                fn to_tokens(&self, tokens: &mut TokenStream) {
                    match self {
                        $(Self::$variant => define_leaf!(@emit tokens, $($token)?),)+
                    }
                }
            }

            impl std::fmt::Display for $name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{}", self.to_token_stream())
                }
            }
        )+
    };

    (@parse_arm $stream:ident, $value:expr => $token:ty) => {
        {
            let mut fork = $stream.fork();

            if <$token as Parse>::parse(&mut fork).is_ok() {
                $stream.seek(&fork);
                return Ok($value);
            }
        }
    };

    (@parse_arm $stream:ident, $value:expr) => {
        return Ok($value);
    };

    (@emit $tokens:ident, $token:ty) => {
        <$token>::default().to_tokens($tokens)
    };

    (@emit $tokens:ident,) => {
        {}
    };
}

use crate::token::{keyword, punct};

define_leaf! {
    #[doc = "A binary operator (`+`, `==`, `&&`, ...)."]
    pub enum BinOp {
        And => punct::AndAnd,
        Or => punct::OrOr,
        Shl => punct::Shl,
        Shr => punct::Shr,
        Eq => punct::EqEq,
        Ne => punct::Ne,
        Le => punct::Le,
        Ge => punct::Ge,
        Add => punct::Plus,
        Sub => punct::Minus,
        Mul => punct::Star,
        Div => punct::Slash,
        Rem => punct::Percent,
        BitXor => punct::Caret,
        BitAnd => punct::And,
        BitOr => punct::Or,
        Lt => punct::Lt,
        Gt => punct::Gt,
    }

    #[doc = "A unary operator (`*`, `!`, `-`)."]
    pub enum UnOp {
        Deref => punct::Star,
        Not => punct::Not,
        Neg => punct::Minus,
    }

    #[doc = "A compound assignment operator (`+=`, `<<=`, ...)."]
    pub enum AssignOp {
        ShlAssign => punct::ShlEq,
        ShrAssign => punct::ShrEq,
        AddAssign => punct::PlusEq,
        SubAssign => punct::MinusEq,
        MulAssign => punct::StarEq,
        DivAssign => punct::SlashEq,
        RemAssign => punct::PercentEq,
        BitXorAssign => punct::CaretEq,
        BitAndAssign => punct::AndEq,
        BitOrAssign => punct::OrEq,
    }

    #[doc = "Whether a function is `async`."]
    pub enum Asyncness {
        Async => keyword::Async,
        Sync,
    }

    #[doc = "Whether an item is `const`."]
    pub enum Constness {
        Const => keyword::Const,
        NoConst,
    }

    #[doc = "Whether an item is `unsafe`."]
    pub enum Unsafety {
        Unsafe => keyword::Unsafe,
        Safe,
    }

    #[doc = "Whether an impl item is `default`."]
    pub enum Defaultness {
        Default => keyword::Default,
        Final,
    }

    #[doc = "Whether a binding, reference, or pointer is `mut`."]
    pub enum Mutability {
        Mutable => keyword::Mut,
        Immutable,
    }

    #[doc = "Whether a closure is `static` (immovable)."]
    pub enum Movability {
        Static => keyword::Static,
        Movable,
    }

    #[doc = "The limits of a range expression (`..` or `..=`)."]
    pub enum RangeLimits {
        Closed => punct::DotDotEq,
        HalfOpen => punct::DotDot,
    }

    #[doc = "A trait bound modifier (`?Sized`)."]
    pub enum TraitBoundModifier {
        Maybe => punct::Question,
        None,
    }

    #[doc = "The polarity of a trait bound (`Trait` or `!Trait`)."]
    pub enum BoundPolarity {
        Negative => punct::Not,
        Positive,
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use crate::TokenStream;
    use crate::token::ToTokenStream;

    fn parse<T: Parse>(src: &str) -> Result<T, ParseError> {
        let ts = TokenStream::from_str(src).unwrap();
        let mut ps = ts.parse();
        ps.parse::<T>()
    }

    #[test]
    fn bin_ops() {
        assert_eq!(parse::<BinOp>("+").unwrap(), BinOp::Add);
        assert_eq!(parse::<BinOp>("==").unwrap(), BinOp::Eq);
        assert_eq!(parse::<BinOp>("&&").unwrap(), BinOp::And);
        assert_eq!(parse::<BinOp>("&").unwrap(), BinOp::BitAnd);
        assert_eq!(parse::<BinOp>("<<").unwrap(), BinOp::Shl);
        assert!(parse::<BinOp>("foo").is_err());
    }

    #[test]
    fn un_ops() {
        assert_eq!(parse::<UnOp>("*").unwrap(), UnOp::Deref);
        assert_eq!(parse::<UnOp>("!").unwrap(), UnOp::Not);
        assert_eq!(parse::<UnOp>("-").unwrap(), UnOp::Neg);
    }

    #[test]
    fn assign_ops() {
        assert_eq!(parse::<AssignOp>("+=").unwrap(), AssignOp::AddAssign);
        assert_eq!(parse::<AssignOp>("<<=").unwrap(), AssignOp::ShlAssign);
    }

    #[test]
    fn markers_parse_from_present_and_absent() {
        assert_eq!(parse::<Mutability>("mut").unwrap(), Mutability::Mutable);
        assert_eq!(parse::<Mutability>("").unwrap(), Mutability::Immutable);
        assert_eq!(parse::<Asyncness>("async").unwrap(), Asyncness::Async);
        assert_eq!(parse::<Asyncness>("").unwrap(), Asyncness::Sync);
        assert_eq!(parse::<Unsafety>("unsafe").unwrap(), Unsafety::Unsafe);
        assert_eq!(parse::<Unsafety>("").unwrap(), Unsafety::Safe);
    }

    #[test]
    fn roundtrips() {
        assert_eq!(parse::<BinOp>("+").unwrap().to_token_stream().to_string(), "+");
        assert_eq!(parse::<BinOp>("==").unwrap().to_token_stream().to_string(), "==");
        assert_eq!(parse::<AssignOp>("<<=").unwrap().to_token_stream().to_string(), "<<=");
        assert_eq!(parse::<RangeLimits>("..=").unwrap().to_token_stream().to_string(), "..=");
        assert_eq!(parse::<Mutability>("mut").unwrap().to_token_stream().to_string(), "mut");
        assert_eq!(parse::<Mutability>("").unwrap().to_token_stream().to_string(), "");
    }

    #[test]
    fn range_and_modifiers() {
        assert_eq!(parse::<RangeLimits>("..").unwrap(), RangeLimits::HalfOpen);
        assert_eq!(parse::<RangeLimits>("..=").unwrap(), RangeLimits::Closed);
        assert_eq!(parse::<TraitBoundModifier>("?").unwrap(), TraitBoundModifier::Maybe);
        assert_eq!(parse::<TraitBoundModifier>("").unwrap(), TraitBoundModifier::None);
        assert_eq!(parse::<BoundPolarity>("!").unwrap(), BoundPolarity::Negative);
        assert_eq!(parse::<BoundPolarity>("").unwrap(), BoundPolarity::Positive);
    }
}
