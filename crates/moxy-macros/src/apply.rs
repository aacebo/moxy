use moxy_token::TokenStream;

type Paths = moxy::ast::List<moxy::ast::Path, moxy::ast::Token![,]>;

pub fn expand(attr: TokenStream, item: TokenStream) -> TokenStream {
    let parser = moxy::ast::Parser::from_tokens(&attr);
    let list = match Paths::parse_separated_nonempty(&parser) {
        Err(err) => return err.to_compile_error(),
        Ok(v) if parser.is_empty() => v,
        Ok(_) => return parser.error("expected comma-separated macro paths").to_compile_error(),
    };

    let mut paths = list.into_iter();
    let path = paths.next().expect("nonempty parser result");
    let remaining: Paths = paths.collect();

    if remaining.is_empty() {
        return moxy::template! {
            {{ path }}! {
                {{ item }}
            }
        };
    }

    moxy::template! {
        {{ path }}! {
            #[::moxy::apply({{ remaining }})]
            {{ item }}
        }
    }
}
