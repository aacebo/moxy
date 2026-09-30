use super::*;

pub struct Generics {
    pub impl_params: TokenStream,
    pub type_params: TokenStream,
    pub where_clause: TokenStream,
}

impl moxy::ast::Parse for Generics {
    fn peek(cursor: moxy::ast::Cursor<'_>) -> bool {
        moxy::ast::Generics::peek(cursor)
    }

    fn parse(parser: &moxy::ast::Parser) -> Result<Self, moxy::ast::ParseError> {
        let generics = moxy::ast::Generics::parse(parser)?;
        Ok(generics.into())
    }

    fn skip(cursor: moxy::ast::Cursor<'_>) -> Option<moxy::ast::Cursor<'_>> {
        moxy::ast::Generics::skip(cursor)
    }
}

impl From<moxy::ast::Generics> for Generics {
    fn from(generics: moxy::ast::Generics) -> Self {
        let mut impl_params = Vec::new();
        let mut type_params = Vec::new();
        let mut predicates = Vec::new();

        for param in generics.params.iter() {
            match param {
                moxy::ast::GenericParam::Lifetime(param) => {
                    impl_params.push(param.to_token_stream());
                    type_params.push(param.lifetime.to_token_stream());
                }
                moxy::ast::GenericParam::Type(param) => {
                    let mut param = (**param).clone();
                    param.eq_punct = None;
                    param.default = None;
                    impl_params.push(param.to_token_stream());
                    type_params.push(param.ident.to_token_stream());
                    let ident = &param.ident;
                    predicates.push(moxy::template! { {{ ident }}: ::moxy::ast::Parse });
                }
                moxy::ast::GenericParam::Const(param) => {
                    let mut param = (**param).clone();
                    param.default_eq_punct = None;
                    param.default = None;
                    impl_params.push(param.to_token_stream());
                    type_params.push(param.ident.to_token_stream());
                }
            }
        }

        if let Some(where_clause) = &generics.where_clause {
            predicates.splice(0..0, where_clause.predicates.iter().map(ToTokenStream::to_token_stream));
        }

        let impl_params = if impl_params.is_empty() {
            TokenStream::new()
        } else {
            moxy::template! { < @for (param in impl_params.iter()) { {{ param }}, } > }
        };
        let type_params = if type_params.is_empty() {
            TokenStream::new()
        } else {
            moxy::template! { < @for (param in type_params.iter()) { {{ param }}, } > }
        };
        let where_clause = if predicates.is_empty() {
            TokenStream::new()
        } else {
            moxy::template! { where @for (predicate in predicates.iter()) { {{ predicate }}, } }
        };

        Self {
            impl_params,
            type_params,
            where_clause,
        }
    }
}
