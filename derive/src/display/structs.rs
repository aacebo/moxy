use proc_macro2::TokenStream;
use quote::quote;

use crate::{
    Render,
    core::{Attrs, Field, FieldName},
    params,
};

#[derive(Clone, Default)]
pub struct StructSyntax;

impl Render for StructSyntax {
    type Args = params::StructParams;

    fn render(&self, args: Self::Args) -> syn::Result<TokenStream> {
        let ident = &args.input.ident;
        let (impl_generics, type_generics, where_generics) = &args.input.generics.split_for_impl();
        let attributes = Attrs::parse(&args.input.attrs)?;
        let display = attributes.get("display");
        let display_attr = display.iter().find_map(|arg| arg.as_attr());
        let custom_fmt = display_attr.and_then(|attr| {
            attr.args().iter().find_map(|arg| {
                if arg.path().is_ident("__value") {
                    arg.as_lit().and_then(|lit| match lit {
                        syn::Lit::Str(s) => Some(s),
                        _ => None,
                    })
                } else {
                    None
                }
            })
        });

        let style = display_attr.and_then(|attr| {
            attr.args().iter().find_map(|arg| {
                let name = arg.path().get_ident()?.to_string();
                let is_style = matches!(name.as_str(), "debug" | "compact" | "keyvalue" | "map")
                    || cfg!(feature = "json") && name == "json";
                is_style.then_some(name)
            })
        });

        let pretty = display_attr
            .map(|attr| attr.exists("pretty"))
            .unwrap_or(false);

        let color = cfg!(feature = "color")
            && display_attr
                .map(|attr| attr.exists("color"))
                .unwrap_or(false);

        let alias = display_attr.and_then(|attr| {
            attr.args().iter().find_map(|arg| {
                if arg.path().is_ident("alias") {
                    arg.as_lit().and_then(|lit| match lit {
                        syn::Lit::Str(s) => Some(s.value()),
                        _ => None,
                    })
                } else {
                    None
                }
            })
        });

        let fields: Vec<_> = args
            .data
            .fields
            .iter()
            .enumerate()
            .filter_map(|(i, field)| Field::parse(i, field).ok())
            .collect();

        let visible_fields: Vec<&Field> = fields
            .iter()
            .filter(|f| {
                let field_display = f.attrs().get("display");
                let field_attr = field_display.iter().find_map(|a| a.as_attr());
                !field_attr.map(|a| a.exists("skip")).unwrap_or(false)
            })
            .collect();

        let is_named = matches!(args.data.fields, syn::Fields::Named(_));
        let is_unit = matches!(args.data.fields, syn::Fields::Unit);
        let name_str = alias.unwrap_or_else(|| ident.to_string());
        let fmt_exprs: Vec<&syn::Expr> = display_attr
            .map(|attr| attr.args().iter().filter_map(|arg| arg.as_expr()).collect())
            .unwrap_or_default();

        let inner = if is_unit || visible_fields.is_empty() {
            quote! { ::std::write!(f, #name_str) }
        } else if let Some(fmt_str) = custom_fmt {
            render_custom_fmt(&visible_fields, is_named, &fmt_str, &fmt_exprs)
        } else if let Some(mode) = style {
            render_style(&mode, &visible_fields, is_named, &name_str, pretty, color)
        } else {
            render_default(&visible_fields, is_named, &name_str, pretty, color)
        };

        #[cfg(feature = "color")]
        let body = if color {
            quote! { use ::colored::Colorize as _; #inner }
        } else {
            inner
        };
        #[cfg(not(feature = "color"))]
        let body = inner;

        Ok(quote! {
            impl #impl_generics ::std::fmt::Display for #ident #type_generics #where_generics {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    #body
                }
            }
        })
    }
}

fn render_default(
    fields: &[&Field],
    is_named: bool,
    name: &str,
    pretty: bool,
    color: bool,
) -> TokenStream {
    let mut fmt = String::new();
    let mut args = Vec::new();

    if is_named {
        if color {
            fmt.push_str("{}");
            args.push(quote! { #name.cyan().bold() });
        } else {
            fmt.push_str(name);
        }
        fmt.push_str(if pretty { " {{\n" } else { " {{ " });

        for (i, f) in fields.iter().enumerate() {
            let fname = f.name();
            if pretty {
                fmt.push_str("    ");
                if color {
                    fmt.push_str("{}: {},\n");
                    let dname = f.display_name();
                    args.push(quote! { #dname.blue() });
                } else {
                    fmt.push_str(&f.display_name());
                    fmt.push_str(": {},\n");
                }
            } else {
                if color {
                    fmt.push_str("{}: {}");
                    let dname = f.display_name();
                    args.push(quote! { #dname.blue() });
                } else {
                    fmt.push_str(&f.display_name());
                    fmt.push_str(": {}");
                }
                if i + 1 < fields.len() {
                    fmt.push_str(", ");
                }
            }
            args.push(quote! { self.#fname });
        }

        fmt.push_str(if pretty { "}}" } else { " }}" });
    } else {
        if color {
            fmt.push_str("{}");
            args.push(quote! { #name.cyan().bold() });
        } else {
            fmt.push_str(name);
        }
        fmt.push_str(if pretty { "(\n" } else { "(" });

        for (i, f) in fields.iter().enumerate() {
            let fname = f.name();
            if pretty {
                fmt.push_str("    {},\n");
            } else {
                fmt.push_str("{}");
                if i + 1 < fields.len() {
                    fmt.push_str(", ");
                }
            }
            args.push(quote! { self.#fname });
        }

        fmt.push(')');
    }

    quote! { ::std::write!(f, #fmt, #(#args),*) }
}

fn render_debug(
    fields: &[&Field],
    is_named: bool,
    name: &str,
    pretty: bool,
    color: bool,
) -> TokenStream {
    if !pretty && !color {
        if is_named {
            let entries: Vec<_> = fields
                .iter()
                .map(|f| {
                    let fname = f.name();
                    let fname_str = f.display_name();
                    quote! { .field(#fname_str, &self.#fname) }
                })
                .collect();

            return quote! {
                f.debug_struct(#name)
                    #(#entries)*
                    .finish()
            };
        }

        let entries: Vec<_> = fields
            .iter()
            .map(|f| {
                let fname = f.name();
                quote! { .field(&self.#fname) }
            })
            .collect();

        return quote! {
            f.debug_tuple(#name)
                #(#entries)*
                .finish()
        };
    }

    let mut fmt = String::new();
    let mut args = Vec::new();

    if color {
        fmt.push_str("{}");
        args.push(quote! { #name.cyan().bold() });
    } else {
        fmt.push_str(name);
    }

    if is_named {
        fmt.push_str(if pretty { " {{\n" } else { " {{ " });

        for (i, f) in fields.iter().enumerate() {
            let fname = f.name();
            if pretty {
                fmt.push_str("    ");
            }
            if color {
                fmt.push_str("{}: {:?}");
                let dname = f.display_name();
                args.push(quote! { #dname.blue() });
            } else {
                fmt.push_str(&f.display_name());
                fmt.push_str(": {:?}");
            }
            if pretty {
                fmt.push_str(",\n");
            } else if i + 1 < fields.len() {
                fmt.push_str(", ");
            }
            args.push(quote! { self.#fname });
        }

        fmt.push_str(if pretty { "}}" } else { " }}" });
    } else {
        fmt.push_str(if pretty { "(\n" } else { "(" });

        for (i, f) in fields.iter().enumerate() {
            let fname = f.name();
            if pretty {
                fmt.push_str("    {:?},\n");
            } else {
                fmt.push_str("{:?}");
                if i + 1 < fields.len() {
                    fmt.push_str(", ");
                }
            }
            args.push(quote! { self.#fname });
        }

        fmt.push(')');
    }

    quote! { ::std::write!(f, #fmt, #(#args),*) }
}

fn render_compact(fields: &[&Field]) -> TokenStream {
    let mut fmt = String::new();
    let mut args = Vec::new();

    for (i, f) in fields.iter().enumerate() {
        let fname = f.name();
        fmt.push_str("{}");

        if i + 1 < fields.len() {
            fmt.push(' ');
        }

        args.push(quote! { self.#fname });
    }

    quote! { ::std::write!(f, #fmt, #(#args),*) }
}

fn render_keyvalue(fields: &[&Field], pretty: bool, color: bool) -> TokenStream {
    let sep = if pretty { "\n" } else { " " };
    let mut fmt = String::new();
    let mut args = Vec::new();

    for (i, f) in fields.iter().enumerate() {
        let fname = f.name();
        if color {
            fmt.push_str("{}={}");
            let dname = f.display_name();
            args.push(quote! { #dname.blue() });
        } else {
            fmt.push_str(&f.display_name());
            fmt.push_str("={}");
        }

        if i + 1 < fields.len() {
            fmt.push_str(sep);
        }
        args.push(quote! { self.#fname });
    }

    quote! { ::std::write!(f, #fmt, #(#args),*) }
}

fn render_map(fields: &[&Field], pretty: bool, color: bool) -> TokenStream {
    let mut fmt = String::new();
    let mut args = Vec::new();

    fmt.push_str(if pretty { "{{\n" } else { "{{ " });

    for (i, f) in fields.iter().enumerate() {
        let fname = f.name();

        if pretty {
            fmt.push_str("    ");
            if color {
                fmt.push_str("{}: {},\n");
                let dname = f.display_name();
                args.push(quote! { #dname.blue() });
            } else {
                fmt.push_str(&f.display_name());
                fmt.push_str(": {},\n");
            }
        } else {
            if color {
                fmt.push_str("{}: {}");
                let dname = f.display_name();
                args.push(quote! { #dname.blue() });
            } else {
                fmt.push_str(&f.display_name());
                fmt.push_str(": {}");
            }

            if i + 1 < fields.len() {
                fmt.push_str(", ");
            }
        }

        args.push(quote! { self.#fname });
    }

    fmt.push_str(if pretty { "}}" } else { " }}" });

    quote! { ::std::write!(f, #fmt, #(#args),*) }
}

fn render_custom_fmt(
    fields: &[&Field],
    is_named: bool,
    pattern: &syn::LitStr,
    exprs: &[&syn::Expr],
) -> TokenStream {
    if is_named {
        let field_idents: Vec<_> = fields
            .iter()
            .filter_map(|f| match f.name() {
                FieldName::Ident(id) => Some(id.clone()),
                _ => None,
            })
            .collect();

        if exprs.is_empty() {
            quote! {
                #[allow(unused)]
                let Self { #(#field_idents,)* .. } = self;
                ::std::write!(f, #pattern)
            }
        } else {
            quote! {
                #[allow(unused)]
                let Self { #(#field_idents,)* .. } = self;
                ::std::write!(f, #pattern, #(#exprs),*)
            }
        }
    } else {
        if exprs.is_empty() {
            let field_indices: Vec<_> = fields.iter().map(|f| f.name().clone()).collect();

            quote! {
                ::std::write!(f, #pattern, #(self.#field_indices,)*)
            }
        } else {
            quote! {
                ::std::write!(f, #pattern, #(#exprs),*)
            }
        }
    }
}

#[cfg(feature = "json")]
fn render_json(fields: &[&Field], is_named: bool, pretty: bool) -> TokenStream {
    if is_named {
        let inserts: Vec<_> = fields
            .iter()
            .map(|f| {
                let fname = f.name();
                let dname = f.display_name();
                quote! {
                    __map.insert(
                        #dname.into(),
                        ::serde_json::to_value(&self.#fname).unwrap_or(::serde_json::Value::Null),
                    );
                }
            })
            .collect();

        let serialize = if pretty {
            quote! { ::serde_json::to_string_pretty(&__val) }
        } else {
            quote! { ::serde_json::to_string(&__val) }
        };

        quote! {
            let mut __map = ::serde_json::Map::new();
            #(#inserts)*
            let __val = ::serde_json::Value::Object(__map);
            ::std::write!(f, "{}", #serialize.unwrap_or_default())
        }
    } else {
        let pushes: Vec<_> = fields
            .iter()
            .map(|f| {
                let fname = f.name();
                quote! {
                    __arr.push(
                        ::serde_json::to_value(&self.#fname).unwrap_or(::serde_json::Value::Null),
                    );
                }
            })
            .collect();

        let serialize = if pretty {
            quote! { ::serde_json::to_string_pretty(&__val) }
        } else {
            quote! { ::serde_json::to_string(&__val) }
        };

        quote! {
            let mut __arr = ::std::vec::Vec::new();
            #(#pushes)*
            let __val = ::serde_json::Value::Array(__arr);
            ::std::write!(f, "{}", #serialize.unwrap_or_default())
        }
    }
}

fn render_style(
    style: &str,
    fields: &[&Field],
    is_named: bool,
    name: &str,
    pretty: bool,
    color: bool,
) -> TokenStream {
    match style {
        "debug" => render_debug(fields, is_named, name, pretty, color),
        "compact" => render_compact(fields),
        "keyvalue" => render_keyvalue(fields, pretty, color),
        "map" => render_map(fields, pretty, color),
        #[cfg(feature = "json")]
        "json" => render_json(fields, is_named, pretty),
        _ => unreachable!(),
    }
}
