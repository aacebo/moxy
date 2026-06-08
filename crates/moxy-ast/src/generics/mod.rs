use moxy_token::keyword::Where;
use moxy_token::parser::{ParseError, ParseStream};
use moxy_token::punct::{Comma, Gt, Lt};
use moxy_token::{Parse, Span, Spanner, ToTokens, TokenStream};

use crate::Punctuated;

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

/// Generic parameters and an optional `where` clause.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Generics {
    pub lt_punct: Lt,
    pub gt_punct: Gt,
    pub params: Punctuated<GenericParam, Comma>,
    pub where_clause: Option<WhereClause>,
}

impl Parse for Generics {
    fn parse(stream: &mut ParseStream) -> Result<Self, ParseError> {
        let params = if stream.peek::<Lt>() {
            let _ = stream.parse::<Lt>()?;
            let params = Punctuated::parse_separated_nonempty(stream)?;
            let _ = stream.parse::<Gt>()?;
            params
        } else {
            Punctuated::new()
        };

        let where_clause = if stream.peek::<Where>() {
            Some(stream.parse::<WhereClause>()?)
        } else {
            None
        };

        Ok(Self {
            lt_punct: Lt::default(),
            gt_punct: Gt::default(),
            params,
            where_clause,
        })
    }
}

impl Spanner for Generics {
    fn span(&self) -> Span {
        if self.params.is_empty() {
            return Span::call_site();
        }
        let end = if let Some(w) = &self.where_clause {
            w.span()
        } else {
            self.gt_punct.span()
        };
        self.lt_punct.span().join(end)
    }
}

impl ToTokens for Generics {
    fn to_tokens(&self, t: &mut TokenStream) {
        if !self.params.is_empty() {
            self.lt_punct.to_tokens(t);
            self.params.to_tokens(t);
            self.gt_punct.to_tokens(t);
        }
        if let Some(w) = &self.where_clause {
            w.to_tokens(t);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use moxy_token::{ToTokenStream, TokenStream};

    use super::*;

    fn render<T: ToTokenStream>(v: &T) -> String {
        v.to_token_stream().to_string()
    }

    #[test]
    fn generics_basic() {
        let g = moxy_token::parse!("<T>" as Generics).unwrap();
        assert_eq!(g.params.len(), 1);
        assert!(matches!(g.params.first().unwrap(), GenericParam::Type(_)));

        let g2 = moxy_token::parse!("<'a, T: Clone, const N: usize>" as Generics).unwrap();
        assert_eq!(g2.params.len(), 3);
        assert!(matches!(g2.params.first().unwrap(), GenericParam::Lifetime(_)));
    }

    #[test]
    fn generics_where() {
        let g = moxy_token::parse!("<T> where T: Clone" as Generics).unwrap();
        assert!(g.where_clause.is_some());
    }

    #[test]
    fn type_bounds() {
        assert!(matches!(
            moxy_token::parse!("Clone" as TypeBound).unwrap(),
            TypeBound::Trait(_)
        ));
        assert!(matches!(
            moxy_token::parse!("'a" as TypeBound).unwrap(),
            TypeBound::Lifetime(_)
        ));
        assert!(matches!(
            moxy_token::parse!("?Sized" as TypeBound).unwrap(),
            TypeBound::Trait(_)
        ));
    }

    #[test]
    fn impl_dyn_types() {
        use crate::Type;
        assert!(matches!(
            moxy_token::parse!("impl Clone" as Type).unwrap(),
            Type::ImplTrait(_)
        ));
        assert!(matches!(
            moxy_token::parse!("dyn Clone + 'a" as Type).unwrap(),
            Type::TraitObject(_)
        ));
        assert_eq!(render(&moxy_token::parse!("impl Clone" as Type).unwrap()), "impl Clone");
    }
}
