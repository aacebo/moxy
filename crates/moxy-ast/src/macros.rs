/// Maps a Rust punctuation or keyword spelling to its corresponding AST token type.
///
/// For example, `Token![::]` is the AST's `PathSep` token type; other token
/// spellings are delegated to `moxy::token::Token!`.
#[macro_export]
macro_rules! Token {
    [&&]    => { $crate::AndAnd };
    [||]    => { $crate::OrOr };
    [<<]    => { $crate::Shl };
    [>>]    => { $crate::Shr };
    [==]    => { $crate::EqEq };
    [!=]    => { $crate::Ne };
    [<=]    => { $crate::Le };
    [>=]    => { $crate::Ge };
    [&=]    => { $crate::AndEq };
    [|=]    => { $crate::OrEq };
    [+=]    => { $crate::PlusEq };
    [-=]    => { $crate::MinusEq };
    [*=]    => { $crate::StarEq };
    [/=]    => { $crate::SlashEq };
    [%=]    => { $crate::PercentEq };
    [^=]    => { $crate::CaretEq };
    [=>]    => { $crate::FatArrow };
    [->]    => { $crate::RArrow };
    [<-]    => { $crate::LArrow };
    [::]    => { $crate::ColonColon };
    [..]    => { $crate::DotDot };
    [<<=]   => { $crate::ShlEq };
    [>>=]   => { $crate::ShrEq };
    [...]   => { $crate::DotDotDot };
    [..=]   => { $crate::DotDotEq };
    [$($tt:tt)*] => {
        $crate::__private::moxy_token::Token![$($tt)*]
    };
}
