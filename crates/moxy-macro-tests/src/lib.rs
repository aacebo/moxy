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
            return moxy::ast::ParseError::new(item.ident.span(), "expected a range attribute")
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
