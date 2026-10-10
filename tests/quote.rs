#[test]
fn quoted_interpolates_syn_literal_and_expression_ast_nodes() {
    let literal: syn::LitStr = syn::parse_quote!("hello");
    let expression: syn::Expr = syn::parse_quote!(a::B);
    let tokens = moxy::template! {
        {{ moxy::quoted!(literal) }}
        {{ moxy::quoted!(&expression) }}
    };

    assert_eq!(tokens.to_string(), "\"hello\" a :: B");
}
