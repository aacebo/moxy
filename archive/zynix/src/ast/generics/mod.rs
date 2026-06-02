use crate::ast::Punctuated;
use crate::parse::{ParseError, ParseStream};
use crate::token::ToTokens;
use crate::token::keyword::Where;
use crate::token::punct::{Comma, Gt, Lt};
use crate::{Parse, Span, TokenStream};

mod const_param;
mod generic_param;
mod lifetime_param;
mod lifetime_predicate;
mod trait_bound;
mod trait_ref;
mod type_bound;
mod type_param;
mod type_predicate;
mod use_bound;
mod where_clause;
mod where_predicate;

pub use const_param::*;
pub use generic_param::*;
pub use lifetime_param::*;
pub use lifetime_predicate::*;
pub use trait_bound::*;
pub use trait_ref::*;
pub use type_bound::*;
pub use type_param::*;
pub use type_predicate::*;
pub use use_bound::*;
pub use where_clause::*;
pub use where_predicate::*;

#[doc = "Generic parameters and an optional `where` clause."]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Generics {
    pub span: Span,
    pub params: Punctuated<GenericParam, Comma>,
    pub where_clause: Option<WhereClause>,
}

impl Parse for Generics {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let params = if stream.peek::<Lt>().is_some() {
            let _ = stream.parse::<Lt>()?;
            let mut params = Punctuated::new();

            while !stream.peek_angle_close() && !stream.is_empty() {
                params.push_value(stream.parse::<GenericParam>()?);
                if stream.peek::<Comma>().is_some() {
                    params.push_punct(stream.parse::<Comma>()?);
                } else {
                    break;
                }
            }

            stream.eat_angle_close()?;
            params
        } else {
            Punctuated::new()
        };

        let where_clause = if stream.peek::<Where>().is_some() {
            Some(stream.parse::<WhereClause>()?)
        } else {
            None
        };

        Ok(Self {
            span: Span::default(),
            params,
            where_clause,
        })
    }
}

impl ToTokens for Generics {
    fn to_tokens(&self, t: &mut TokenStream) {
        if !self.params.is_empty() {
            Lt::default().to_tokens(t);
            self.params.to_tokens(t);
            Gt::default().to_tokens(t);
        }
        if let Some(w) = &self.where_clause {
            w.to_tokens(t);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use crate::TokenStream;
    use crate::token::ToTokenStream;

    fn parse<T: Parse>(src: &str) -> T {
        let ts = TokenStream::from_str(src).unwrap();
        ts.parse().parse::<T>().unwrap()
    }

    fn render<T: ToTokenStream>(v: &T) -> String {
        v.to_token_stream().to_string()
    }

    #[test]
    fn generics_basic() {
        let g: Generics = parse("<T>");
        assert_eq!(g.params.len(), 1);
        assert!(matches!(g.params.first().unwrap(), GenericParam::Type(_)));

        let g2: Generics = parse("<'a, T: Clone, const N: usize>");
        assert_eq!(g2.params.len(), 3);
        assert!(matches!(g2.params.first().unwrap(), GenericParam::Lifetime(_)));
    }

    #[test]
    fn generics_where() {
        let g: Generics = parse("<T> where T: Clone");
        assert!(g.where_clause.is_some());
    }

    #[test]
    fn type_bounds() {
        assert!(matches!(parse::<TypeBound>("Clone"), TypeBound::Trait(_)));
        assert!(matches!(parse::<TypeBound>("'a"), TypeBound::Lifetime(_)));
        assert!(matches!(parse::<TypeBound>("?Sized"), TypeBound::Trait(_)));
    }

    #[test]
    fn impl_dyn_types() {
        use crate::ast::Type;
        assert!(matches!(parse::<Type>("impl Clone"), Type::ImplTrait(_)));
        assert!(matches!(parse::<Type>("dyn Clone + 'a"), Type::TraitObject(_)));
        assert_eq!(render(&parse::<Type>("impl Clone")), "impl Clone");
    }
}
