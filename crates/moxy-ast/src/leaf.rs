use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::{LexError, Parse, Span, Spanner, ToTokenStream, ToTokens, TokenStream};

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

            impl Spanner for $name {
                fn span(&self) -> Span {
                    define_leaf!(@span_match self, () $($variant $(($token))?,)+)
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

    (@span_match $self:ident, ($($arms:tt)*) $variant:ident ($token:ty), $($rest:tt)*) => {
        define_leaf!(@span_match $self, ($($arms)* Self::$variant(tok) => tok.span(),) $($rest)*)
    };
    (@span_match $self:ident, ($($arms:tt)*) $variant:ident, $($rest:tt)*) => {
        define_leaf!(@span_match $self, ($($arms)* Self::$variant => Span::call_site(),) $($rest)*)
    };
    (@span_match $self:ident, ($($arms:tt)*)) => {
        match $self { $($arms)* }
    };
}

use moxy_token::{keyword, punct};

define_leaf! {
    /// A binary operator (`+`, `==`, `&&`, ...).
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

    /// A unary operator (`*`, `!`, `-`).
    pub enum UnOp {
        Deref => punct::Star,
        Not => punct::Not,
        Neg => punct::Minus,
    }

    /// A compound assignment operator (`+=`, `<<=`, ...).
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

    /// Whether a function is `async`.
    pub enum Asyncness {
        Async => keyword::Async,
        Sync,
    }

    /// Whether an item is `const`.
    pub enum Constness {
        Const => keyword::Const,
        NoConst,
    }

    /// Whether an item is `unsafe`.
    pub enum Unsafety {
        Unsafe => keyword::Unsafe,
        Safe,
    }

    /// Whether an impl item is `default`.
    pub enum Defaultness {
        Default => keyword::Default,
        Final,
    }

    /// Whether a binding, reference, or pointer is `mut`.
    pub enum Mutability {
        Mutable => keyword::Mut,
        Immutable,
    }

    /// Whether a closure is `static` (immovable).
    pub enum Movability {
        Static => keyword::Static,
        Movable,
    }

    /// The limits of a range expression (`..` or `..=`).
    pub enum RangeLimits {
        Closed => punct::DotDotEq,
        HalfOpen => punct::DotDot,
    }

    /// A trait bound modifier (`?Sized`).
    pub enum TraitBoundModifier {
        Maybe => punct::Question,
        None,
    }

    /// The polarity of a trait bound (`Trait` or `!Trait`).
    pub enum BoundPolarity {
        Negative => punct::Not,
        Positive,
    }
}
