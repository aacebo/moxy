#[macro_export]
macro_rules! ident {
    ($fmt:expr) => {{
        $crate::Ident::lex($fmt).expect("invalid syntax")
    }};
    ($fmt:expr, $($token:tt)*) => {{
        $crate::Ident::lex(format!($fmt, $($token)*)).expect("invalid syntax")
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn literal() {
        assert_eq!(ident!("test"), "test");
    }

    #[test]
    fn formatted() {
        assert_eq!(ident!("testing{}", 123), "testing123");
    }
}
