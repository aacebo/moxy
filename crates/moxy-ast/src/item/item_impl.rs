use moxy_token::{Span, Spanner, ToTokenStream, ToTokens, TokenStream};

use crate::*;

/// An `impl` block, optionally implementing a trait (`impl Trait for Type { ... }`).
#[derive(Clone)]
#[cfg_attr(feature = "derives", derive(Debug, PartialEq, Eq))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ItemImpl {
    pub attrs: Attributes,
    pub defaultness: Option<Token![default]>,
    pub unsafety: Option<Token![unsafe]>,
    pub impl_keyword: Token![impl],
    pub generics: Generics,
    pub for_keyword: Option<Token![for]>,
    pub trait_ref: Option<TraitRef>,
    pub self_ty: Type,
    /// The inner attributes that open the body, as `#![a]`.
    pub inner_attrs: Attributes,
    pub items: Delimited<Vec<ImplItem>>,
}

impl Attributed for ItemImpl {
    fn attrs(&self) -> &[Attribute] {
        &self.attrs
    }
}

impl ItemImpl {
    fn type_to_trait_ref(ty: Type, polarity: Option<Token![!]>) -> Result<TraitRef, ParseError> {
        match ty {
            Type::Path(tp) => Ok(TraitRef { polarity, path: tp.path }),
            ty => Err(ParseError::new(
                ty.span(),
                format!("expected trait path, found {}", found_node(&ty)),
            )),
        }
    }

    pub fn into_item(self) -> super::Item {
        super::Item::from(self)
    }
}

impl Parse for ItemImpl {
    fn peek(cursor: crate::Cursor<'_>) -> bool {
        let cursor = Attributes::skip(cursor).unwrap_or(cursor);
        let cursor = Option::<Token![default]>::skip(cursor).unwrap_or(cursor);
        let cursor = Option::<Token![unsafe]>::skip(cursor).unwrap_or(cursor);
        <Token![impl]>::peek(cursor)
    }

    fn parse(parser: &Parser) -> Result<Self, ParseError> {
        let attrs = <_ as Parse>::parse(parser)?;
        let defaultness = <_ as Parse>::parse(parser)?;
        let unsafety = <_ as Parse>::parse(parser)?;
        let impl_keyword = <_ as Parse>::parse(parser)?;
        let mut generics: Generics = <_ as Parse>::parse(parser)?;
        let polarity: Option<Token![!]> = if negative(parser.cursor()) {
            Some(<_ as Parse>::parse(parser)?)
        } else {
            None
        };

        let first = <_ as Parse>::parse(parser)?;

        // Only a trait impl is negative, as in rustc
        if let Some(bang) = &polarity
            && !<Token![for]>::peek(parser.cursor())
        {
            return Err(ParseError::new(bang.span(), "an inherent impl cannot be negative"));
        }

        let (for_keyword, trait_ref, self_ty) = if <Token![for]>::peek(parser.cursor()) {
            let for_keyword = <_ as Parse>::parse(parser)?;
            let self_ty = <_ as Parse>::parse(parser)?;
            (Some(for_keyword), Some(Self::type_to_trait_ref(first, polarity)?), self_ty)
        } else {
            (None, None, first)
        };

        generics.where_clause = <_ as Parse>::parse(parser)?;
        let mut inner_attrs = Attributes::default();
        let items = Delimited::parse_brace_with(parser, |parser| {
            inner_attrs = Attributes::parse_inner(parser)?;
            <Vec<ImplItem> as Parse>::parse(parser)
        })?;

        Ok(Self {
            attrs,
            defaultness,
            unsafety,
            impl_keyword,
            generics,
            for_keyword,
            trait_ref,
            self_ty,
            inner_attrs,
            items,
        })
    }

    fn skip(mut cursor: crate::Cursor<'_>) -> Option<crate::Cursor<'_>> {
        cursor = Attributes::skip(cursor)?;
        cursor = Option::<Token![default]>::skip(cursor)?;
        cursor = Option::<Token![unsafe]>::skip(cursor)?;
        cursor = <Token![impl]>::skip(cursor)?;
        cursor = Generics::skip(cursor)?;
        let negative = negative(cursor);

        if negative {
            cursor = <Token![!]>::skip(cursor)?;
        }

        cursor = Type::skip(cursor)?;

        if negative && !<Token![for]>::peek(cursor) {
            return None;
        }

        if <Token![for]>::peek(cursor) {
            cursor = <Token![for]>::skip(cursor)?;
            cursor = Type::skip(cursor)?;
        }

        cursor = Option::<crate::WhereClause>::skip(cursor)?;
        let mut inner = Attributes::skip_inner(cursor.descend(moxy_token::Delim::Brace)?)?;

        while !inner.is_empty() {
            inner = ImplItem::skip(inner)?;
        }

        Some(cursor.offset(1))
    }
}

impl Spanner for ItemImpl {
    fn span(&self) -> Span {
        self.to_token_stream().span()
    }
}

impl ToTokens for ItemImpl {
    fn to_tokens(&self, t: &mut TokenStream) {
        self.attrs.to_tokens(t);
        self.defaultness.to_tokens(t);
        self.unsafety.to_tokens(t);
        self.impl_keyword.to_tokens(t);
        self.generics.lt.to_tokens(t);
        self.generics.params.to_tokens(t);
        self.generics.gt.to_tokens(t);
        self.trait_ref.to_tokens(t);
        self.for_keyword.to_tokens(t);
        self.self_ty.to_tokens(t);
        self.generics.where_clause.to_tokens(t);
        let mut body = TokenStream::new();
        self.inner_attrs.to_tokens(&mut body);
        self.items.inner.to_tokens(&mut body);
        self.items.surround(t, body);
    }
}

// As in rustc, a `!` is the polarity of a negative impl when a type follows it,
// and otherwise the never type, as in `impl ! {}`
fn negative(cursor: crate::Cursor<'_>) -> bool {
    <Token![!]>::peek(cursor) && Type::peek(cursor.offset(1))
}
