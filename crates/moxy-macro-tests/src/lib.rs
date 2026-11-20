#[proc_macro]
pub fn nonzero_u8(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let result = syn::parse::<syn::LitInt>(input).and_then(|literal| {
        let value = literal.base10_parse::<u8>()?;

        if value == 0 {
            return Err(syn::Error::new(literal.span(), "expected a nonzero u8 literal"));
        }

        let mut token = proc_macro2::Literal::u8_unsuffixed(value);
        token.set_span(literal.span());
        Ok(token)
    });

    match result {
        Ok(literal) => moxy::template! {
            ::core::num::NonZeroU8::new({{ &literal }}).unwrap()
        }
        .into(),
        Err(error) => {
            let error = error.to_compile_error();
            moxy::template! { {{ &error }} }.into()
        }
    }
}

// Re-emits its input after a round trip through moxy's token stream
#[proc_macro]
pub fn round_trip(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let tokens: moxy::token::TokenStream = input.into();
    tokens.into()
}

#[derive(moxy::FromMeta)]
struct RangeArgs {
    #[meta(default)]
    min: u8,
    max: u8,
}

#[proc_macro_derive(Range, attributes(range))]
pub fn derive_range(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    use moxy::ast::Attributed;

    let tokens: moxy::token::TokenStream = input.into();
    let item: moxy::ast::ItemStruct = match moxy::parse!(tokens) {
        Ok(item) => item,
        Err(error) => return error.to_compile_error().into(),
    };

    let args = match item.parse_meta::<RangeArgs>("range") {
        Ok(Some(args)) => args,
        Ok(None) => {
            return moxy::ast::ParseError::new(item.ident.span(), "expected `#[range(...)]` attribute")
                .to_compile_error()
                .into();
        }
        Err(error) => return error.to_compile_error().into(),
    };

    let ident = &item.ident;
    let min = proc_macro2::Literal::u8_unsuffixed(args.min);
    let max = proc_macro2::Literal::u8_unsuffixed(args.max);

    moxy::template! {
        impl {{ ident }} {
            pub const fn contains(value: u8) -> bool {
                value >= {{ min }} && value <= {{ max }}
            }
        }
    }
    .into()
}

// Re-emits its input, through the entry that `moxy::function` generates
#[moxy::function(name = "echo")]
pub fn echo(tokens: moxy::token::TokenStream) -> Result<moxy::token::TokenStream, moxy::ast::ParseError> {
    Ok(tokens)
}

// Re-emits its item, through the entry that `moxy::attribute` generates
#[moxy::attribute(name = "echo_item")]
pub fn echo_item(
    _meta: moxy::token::TokenStream,
    tokens: moxy::token::TokenStream,
) -> Result<moxy::token::TokenStream, moxy::ast::ParseError> {
    Ok(tokens)
}

// The sources on the expanding thread's map once it has lexed a string, as a
// literal: the macro releases them after each expansion, so every expansion
// gives the same count
#[moxy::function(name = "lexed_sources")]
pub fn lexed_sources(_tokens: moxy::token::TokenStream) -> Result<moxy::token::TokenStream, moxy::ast::ParseError> {
    let _: moxy::ast::Expr = moxy::parse!("a + b")?;
    let count = moxy::token::source::SourceMap::with(|map| map.len());
    let count = moxy::token::Lit::usize_unsuffixed(count);
    Ok(moxy::template! { {{ &count }} })
}

// Wraps its input expression in parentheses that the template builds
#[proc_macro]
pub fn parenthesized(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let tokens: moxy::token::TokenStream = input.into();
    moxy::template! { ({{ &tokens }}) }.into()
}
