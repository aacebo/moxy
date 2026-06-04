/// Constructs a [`Delimited`](crate::Delimited) with `Delim::Paren`.
///
/// ```
/// parenthesized!(expr)           // default span
/// parenthesized!(span => expr)   // explicit DelimSpan
/// ```
#[macro_export]
macro_rules! parenthesized {
    ($span:expr => $inner:expr) => {
        $crate::Delimited::paren($span, $inner)
    };
    ($inner:expr) => {
        $crate::Delimited::paren(::moxy_token::span::DelimSpan::default(), $inner)
    };
}

/// Constructs a [`Delimited`](crate::Delimited) with `Delim::Bracket`.
///
/// ```
/// bracketed!(elems)           // default span
/// bracketed!(span => elems)   // explicit DelimSpan
/// ```
#[macro_export]
macro_rules! bracketed {
    ($span:expr => $inner:expr) => {
        $crate::Delimited::bracket($span, $inner)
    };
    ($inner:expr) => {
        $crate::Delimited::bracket(::moxy_token::span::DelimSpan::default(), $inner)
    };
}

/// Constructs a [`Delimited`](crate::Delimited) with `Delim::Brace`.
///
/// ```
/// braced!(stmts)           // default span
/// braced!(span => stmts)   // explicit DelimSpan
/// ```
#[macro_export]
macro_rules! braced {
    ($span:expr => $inner:expr) => {
        $crate::Delimited::brace($span, $inner)
    };
    ($inner:expr) => {
        $crate::Delimited::brace(::moxy_token::span::DelimSpan::default(), $inner)
    };
}

/// Constructs a [`Punctuated`](crate::Punctuated) from a list of values,
/// inserting default punctuation between each element.
///
/// ```
/// punctuated![a, b, c]   // Punctuated<_, P> where P: Default
/// ```
#[macro_export]
macro_rules! punctuated {
    () => {
        $crate::Punctuated::new()
    };
    ($($item:expr),+ $(,)?) => {{
        let mut p = $crate::Punctuated::new();
        $(p.push($item);)+
        p
    }};
}
