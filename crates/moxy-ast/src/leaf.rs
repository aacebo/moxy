use moxy_token::parse::{ParseError, ParseStream};
use moxy_token::{LexError, Parse, ToTokenStream, ToTokens, TokenStream};

macro_rules! define_leaf {
    ($(
        $(#[doc = $doc:literal])?
        pub enum $name:ident {
            $($variant:ident $(=> $token:ty)?),+ $(,)?
        }
    )+) => {
        $(
            $(#[doc = $doc])?
            #[derive(Debug, Clone)]
            pub enum $name {
                $($variant $(( $token ))? ,)+
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
                    define_leaf!(@emit_match self, tokens, () $($variant $(($token))?,)+);
                }
            }

            impl std::fmt::Display for $name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{}", self.to_token_stream())
                }
            }

            // Equality/hashing ignore the contained token's span: two `BinOp::Add`
            // parsed at different offsets are the same operator.
            impl PartialEq for $name {
                fn eq(&self, other: &Self) -> bool {
                    ::std::mem::discriminant(self) == ::std::mem::discriminant(other)
                }
            }

            impl Eq for $name {}

            impl ::std::hash::Hash for $name {
                fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
                    ::std::mem::discriminant(self).hash(state);
                }
            }

            #[cfg(feature = "serde")]
            impl serde::Serialize for $name {
                fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
                where
                    S: serde::Serializer,
                {
                    define_leaf!(@serialize_match self, s, () $($variant $(($token))?,)+)
                }
            }
        )+
    };

    (@parse_arm $stream:ident, $value:expr => $token:ty) => {
        {
            let mut fork = $stream.fork();

            if let Ok(tok) = <$token as Parse>::parse(&mut fork) {
                $stream.seek(&fork);
                return Ok($value(tok));
            }
        }
    };

    (@parse_arm $stream:ident, $value:expr) => {
        return Ok($value);
    };

    // Build the `ToTokens` match by peeling variants into an accumulator, so the
    // bound token `tok` and its use stay in one expansion (hygiene) and no macro
    // call sits in match-arm position.
    (@emit_match $self:ident, $tokens:ident, ($($arms:tt)*) $variant:ident ($token:ty), $($rest:tt)*) => {
        define_leaf!(@emit_match $self, $tokens,
            ($($arms)* Self::$variant(tok) => tok.to_tokens($tokens),) $($rest)*)
    };
    (@emit_match $self:ident, $tokens:ident, ($($arms:tt)*) $variant:ident, $($rest:tt)*) => {
        define_leaf!(@emit_match $self, $tokens,
            ($($arms)* Self::$variant => {},) $($rest)*)
    };
    (@emit_match $self:ident, $tokens:ident, ($($arms:tt)*)) => {
        match $self { $($arms)* }
    };

    // Same accumulator approach for the serde `Serialize` impl.
    (@serialize_match $self:ident, $s:ident, ($($arms:tt)*) $variant:ident ($token:ty), $($rest:tt)*) => {
        define_leaf!(@serialize_match $self, $s,
            ($($arms)* Self::$variant(_) => $s.serialize_str(stringify!($variant)),) $($rest)*)
    };
    (@serialize_match $self:ident, $s:ident, ($($arms:tt)*) $variant:ident, $($rest:tt)*) => {
        define_leaf!(@serialize_match $self, $s,
            ($($arms)* Self::$variant => $s.serialize_str(stringify!($variant)),) $($rest)*)
    };
    (@serialize_match $self:ident, $s:ident, ($($arms:tt)*)) => {
        match $self { $($arms)* }
    };
}

use moxy_token::{keyword, punct};

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

    use moxy_token::{ToTokenStream, TokenStream};

    use super::*;

    fn parse<T: Parse>(src: &str) -> Result<T, ParseError> {
        let ts = TokenStream::from_str(src).unwrap();
        let mut ps = ts.parse();
        ps.parse::<T>()
    }

    #[test]
    fn bin_ops() {
        assert!(matches!(parse::<BinOp>("+").unwrap(), BinOp::Add(_)));
        assert!(matches!(parse::<BinOp>("==").unwrap(), BinOp::Eq(_)));
        assert!(matches!(parse::<BinOp>("&&").unwrap(), BinOp::And(_)));
        assert!(matches!(parse::<BinOp>("&").unwrap(), BinOp::BitAnd(_)));
        assert!(matches!(parse::<BinOp>("<<").unwrap(), BinOp::Shl(_)));
        assert!(parse::<BinOp>("foo").is_err());
    }

    #[test]
    fn un_ops() {
        assert!(matches!(parse::<UnOp>("*").unwrap(), UnOp::Deref(_)));
        assert!(matches!(parse::<UnOp>("!").unwrap(), UnOp::Not(_)));
        assert!(matches!(parse::<UnOp>("-").unwrap(), UnOp::Neg(_)));
    }

    #[test]
    fn assign_ops() {
        assert!(matches!(parse::<AssignOp>("+=").unwrap(), AssignOp::AddAssign(_)));
        assert!(matches!(parse::<AssignOp>("<<=").unwrap(), AssignOp::ShlAssign(_)));
    }

    #[test]
    fn markers_parse_from_present_and_absent() {
        assert!(matches!(parse::<Mutability>("mut").unwrap(), Mutability::Mutable(_)));
        assert!(matches!(parse::<Mutability>("").unwrap(), Mutability::Immutable));
        assert!(matches!(parse::<Asyncness>("async").unwrap(), Asyncness::Async(_)));
        assert!(matches!(parse::<Asyncness>("").unwrap(), Asyncness::Sync));
        assert!(matches!(parse::<Unsafety>("unsafe").unwrap(), Unsafety::Unsafe(_)));
        assert!(matches!(parse::<Unsafety>("").unwrap(), Unsafety::Safe));
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
        assert!(matches!(parse::<RangeLimits>("..").unwrap(), RangeLimits::HalfOpen(_)));
        assert!(matches!(parse::<RangeLimits>("..=").unwrap(), RangeLimits::Closed(_)));
        assert!(matches!(
            parse::<TraitBoundModifier>("?").unwrap(),
            TraitBoundModifier::Maybe(_)
        ));
        assert!(matches!(parse::<TraitBoundModifier>("").unwrap(), TraitBoundModifier::None));
        assert!(matches!(parse::<BoundPolarity>("!").unwrap(), BoundPolarity::Negative(_)));
        assert!(matches!(parse::<BoundPolarity>("").unwrap(), BoundPolarity::Positive));
    }

    #[test]
    fn equality_ignores_span() {
        // Same operator parsed at different offsets compares equal.
        assert_eq!(parse::<BinOp>("+ ").unwrap(), parse::<BinOp>(" +").unwrap());
    }
}
