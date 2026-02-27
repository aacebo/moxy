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
        let display = attributes.get("display")?;
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

        let style = if let Some(attr) = display_attr {
            let styles: Vec<_> = attr
                .args()
                .iter()
                .filter_map(|arg| {
                    let name = arg.path().get_ident()?.to_string();
                    let is_style =
                        matches!(name.as_str(), "debug" | "compact" | "keyvalue" | "map")
                            || cfg!(feature = "json") && name == "json";
                    is_style.then_some((name, arg.path().clone()))
                })
                .collect();
            if styles.len() > 1 {
                return Err(syn::Error::new_spanned(
                    &styles[1].1,
                    "conflicting display styles; only one style may be specified",
                ));
            }
            styles.into_iter().next().map(|(name, _)| name)
        } else {
            None
        };

        let pretty = display_attr
            .map(|attr| attr.exists("pretty"))
            .unwrap_or(false);

        let theme: Option<String> = if cfg!(feature = "color") {
            display_attr.and_then(|attr| {
                let arg = attr.args().iter().find(|a| a.path().is_ident("color"))?;
                match arg.as_lit() {
                    Some(syn::Lit::Str(s)) => Some(s.value()),
                    _ => Some(String::new()),
                }
            })
        } else {
            None
        };

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
            .map(|(i, field)| Field::parse(i, field))
            .collect::<syn::Result<Vec<_>>>()?;

        let visible_fields = fields
            .iter()
            .map(|f| -> syn::Result<Option<&Field>> {
                let field_display = f.attrs().get("display")?;
                let field_attr = field_display.iter().find_map(|a| a.as_attr());
                Ok(if field_attr.map(|a| a.exists("skip")).unwrap_or(false) {
                    None
                } else {
                    Some(f)
                })
            })
            .collect::<syn::Result<Vec<_>>>()?
            .into_iter()
            .flatten()
            .collect::<Vec<_>>();

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
            render_style(
                &mode,
                &visible_fields,
                is_named,
                &name_str,
                pretty,
                theme.as_deref(),
            )?
        } else {
            render_default(
                &visible_fields,
                is_named,
                &name_str,
                pretty,
                theme.as_deref(),
            )?
        };

        #[cfg(feature = "color")]
        let body = if theme.is_some() {
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

#[cfg(feature = "color")]
fn themed_tokens(theme: &str) -> (TokenStream, TokenStream, TokenStream, TokenStream) {
    let t = super::themes::get(theme);
    let nc = t.name();
    let fc = t.field();
    let vc = t.value();
    let pc = t.punct();
    let (nr, ng, nb) = (nc.r, nc.g, nc.b);
    let (fr, fg, fb) = (fc.r, fc.g, fc.b);
    let (vr, vg, vb) = (vc.r, vc.g, vc.b);
    let (pr, pg, pb) = (pc.r, pc.g, pc.b);
    (
        quote! { .truecolor(#nr, #ng, #nb).bold() },
        quote! { .truecolor(#fr, #fg, #fb) },
        quote! { .truecolor(#vr, #vg, #vb) },
        quote! { .truecolor(#pr, #pg, #pb) },
    )
}

fn render_default(
    fields: &[&Field],
    is_named: bool,
    name: &str,
    pretty: bool,
    theme: Option<&str>,
) -> syn::Result<TokenStream> {
    let _ = &theme;
    #[cfg(feature = "color")]
    if let Some(t) = theme {
        let (nc, fc, vc, pc) = themed_tokens(t);
        let mut fmt = String::new();
        let mut args: Vec<TokenStream> = Vec::new();

        if is_named {
            fmt.push_str("{}{}");
            args.push(quote! { #name #nc });
            args.push(if pretty {
                quote! { " {\n" #pc }
            } else {
                quote! { " { " #pc }
            });

            for (i, f) in fields.iter().enumerate() {
                let fname = f.name();
                let dname = f.display_name()?;
                if pretty {
                    fmt.push_str("    {}{}{}{}");
                } else {
                    fmt.push_str("{}{}{}");
                    if i + 1 < fields.len() {
                        fmt.push_str("{}");
                    }
                }
                args.push(quote! { #dname #fc });
                args.push(quote! { ": " #pc });
                args.push(quote! { ::std::format!("{}", self.#fname) #vc });
                if pretty {
                    args.push(quote! { ",\n" #pc });
                } else if i + 1 < fields.len() {
                    args.push(quote! { ", " #pc });
                }
            }

            fmt.push_str("{}");
            args.push(if pretty {
                quote! { "}" #pc }
            } else {
                quote! { " }" #pc }
            });
        } else {
            fmt.push_str("{}{}");
            args.push(quote! { #name #nc });
            args.push(if pretty {
                quote! { "(\n" #pc }
            } else {
                quote! { "(" #pc }
            });

            for (i, f) in fields.iter().enumerate() {
                let fname = f.name();
                if pretty {
                    fmt.push_str("    {}{}");
                    args.push(quote! { ::std::format!("{}", self.#fname) #vc });
                    args.push(quote! { ",\n" #pc });
                } else {
                    fmt.push_str("{}");
                    args.push(quote! { ::std::format!("{}", self.#fname) #vc });
                    if i + 1 < fields.len() {
                        fmt.push_str("{}");
                        args.push(quote! { ", " #pc });
                    }
                }
            }

            fmt.push_str("{}");
            args.push(quote! { ")" #pc });
        }

        return Ok(quote! { ::std::write!(f, #fmt, #(#args),*) });
    }

    let mut fmt = String::new();
    let mut args = Vec::new();

    if is_named {
        fmt.push_str(name);
        fmt.push_str(if pretty { " {{\n" } else { " {{ " });

        for (i, f) in fields.iter().enumerate() {
            let fname = f.name();
            if pretty {
                fmt.push_str("    ");
                fmt.push_str(&f.display_name()?);
                fmt.push_str(": {},\n");
            } else {
                fmt.push_str(&f.display_name()?);
                fmt.push_str(": {}");
                if i + 1 < fields.len() {
                    fmt.push_str(", ");
                }
            }
            args.push(quote! { self.#fname });
        }

        fmt.push_str(if pretty { "}}" } else { " }}" });
    } else {
        fmt.push_str(name);
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

    Ok(quote! { ::std::write!(f, #fmt, #(#args),*) })
}

fn render_debug(
    fields: &[&Field],
    is_named: bool,
    name: &str,
    pretty: bool,
    theme: Option<&str>,
) -> syn::Result<TokenStream> {
    let _ = &theme;
    #[cfg(feature = "color")]
    if let Some(t) = theme {
        let (nc, fc, vc, pc) = themed_tokens(t);
        let mut fmt = String::new();
        let mut args: Vec<TokenStream> = Vec::new();

        fmt.push_str("{}");
        args.push(quote! { #name #nc });

        if is_named {
            fmt.push_str("{}");
            args.push(if pretty {
                quote! { " {\n" #pc }
            } else {
                quote! { " { " #pc }
            });

            for (i, f) in fields.iter().enumerate() {
                let fname = f.name();
                let dname = f.display_name()?;
                if pretty {
                    fmt.push_str("    {}{}{}{}");
                } else {
                    fmt.push_str("{}{}{}");
                    if i + 1 < fields.len() {
                        fmt.push_str("{}");
                    }
                }
                args.push(quote! { #dname #fc });
                args.push(quote! { ": " #pc });
                args.push(quote! { ::std::format!("{:?}", self.#fname) #vc });
                if pretty {
                    args.push(quote! { ",\n" #pc });
                } else if i + 1 < fields.len() {
                    args.push(quote! { ", " #pc });
                }
            }

            fmt.push_str("{}");
            args.push(if pretty {
                quote! { "}" #pc }
            } else {
                quote! { " }" #pc }
            });
        } else {
            fmt.push_str("{}");
            args.push(if pretty {
                quote! { "(\n" #pc }
            } else {
                quote! { "(" #pc }
            });

            for (i, f) in fields.iter().enumerate() {
                let fname = f.name();
                if pretty {
                    fmt.push_str("    {}{}");
                    args.push(quote! { ::std::format!("{:?}", self.#fname) #vc });
                    args.push(quote! { ",\n" #pc });
                } else {
                    fmt.push_str("{}");
                    args.push(quote! { ::std::format!("{:?}", self.#fname) #vc });
                    if i + 1 < fields.len() {
                        fmt.push_str("{}");
                        args.push(quote! { ", " #pc });
                    }
                }
            }

            fmt.push_str("{}");
            args.push(quote! { ")" #pc });
        }

        return Ok(quote! { ::std::write!(f, #fmt, #(#args),*) });
    }

    if !pretty {
        if is_named {
            let entries = fields
                .iter()
                .map(|f| {
                    let fname = f.name();
                    let fname_str = f.display_name()?;
                    Ok(quote! { .field(#fname_str, &self.#fname) })
                })
                .collect::<syn::Result<Vec<_>>>()?;

            return Ok(quote! {
                f.debug_struct(#name)
                    #(#entries)*
                    .finish()
            });
        }

        let entries: Vec<_> = fields
            .iter()
            .map(|f| {
                let fname = f.name();
                quote! { .field(&self.#fname) }
            })
            .collect();

        return Ok(quote! {
            f.debug_tuple(#name)
                #(#entries)*
                .finish()
        });
    }

    let mut fmt = String::new();
    let mut args = Vec::new();

    fmt.push_str(name);

    if is_named {
        fmt.push_str(" {{\n");

        for f in fields.iter() {
            let fname = f.name();
            fmt.push_str("    ");
            fmt.push_str(&f.display_name()?);
            fmt.push_str(": {:?},\n");
            args.push(quote! { self.#fname });
        }

        fmt.push_str("}}");
    } else {
        fmt.push_str("(\n");

        for f in fields.iter() {
            let fname = f.name();
            fmt.push_str("    {:?},\n");
            args.push(quote! { self.#fname });
        }

        fmt.push(')');
    }

    Ok(quote! { ::std::write!(f, #fmt, #(#args),*) })
}

fn render_compact(fields: &[&Field]) -> syn::Result<TokenStream> {
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

    Ok(quote! { ::std::write!(f, #fmt, #(#args),*) })
}

fn render_keyvalue(
    fields: &[&Field],
    pretty: bool,
    theme: Option<&str>,
) -> syn::Result<TokenStream> {
    let _ = &theme;
    #[cfg(feature = "color")]
    if let Some(t) = theme {
        let (_nc, fc, vc, pc) = themed_tokens(t);
        let sep = if pretty { "\n" } else { " " };
        let mut fmt = String::new();
        let mut args: Vec<TokenStream> = Vec::new();

        for (i, f) in fields.iter().enumerate() {
            let fname = f.name();
            let dname = f.display_name()?;
            fmt.push_str("{}{}{}");
            args.push(quote! { #dname #fc });
            args.push(quote! { "=" #pc });
            args.push(quote! { ::std::format!("{}", self.#fname) #vc });

            if i + 1 < fields.len() {
                fmt.push_str(sep);
            }
        }

        return Ok(quote! { ::std::write!(f, #fmt, #(#args),*) });
    }

    let sep = if pretty { "\n" } else { " " };
    let mut fmt = String::new();
    let mut args = Vec::new();

    for (i, f) in fields.iter().enumerate() {
        let fname = f.name();
        fmt.push_str(&f.display_name()?);
        fmt.push_str("={}");

        if i + 1 < fields.len() {
            fmt.push_str(sep);
        }
        args.push(quote! { self.#fname });
    }

    Ok(quote! { ::std::write!(f, #fmt, #(#args),*) })
}

fn render_map(fields: &[&Field], pretty: bool, theme: Option<&str>) -> syn::Result<TokenStream> {
    let _ = &theme;
    #[cfg(feature = "color")]
    if let Some(t) = theme {
        let (_nc, fc, vc, pc) = themed_tokens(t);
        let mut fmt = String::new();
        let mut args: Vec<TokenStream> = Vec::new();

        fmt.push_str("{}");
        args.push(if pretty {
            quote! { "{\n" #pc }
        } else {
            quote! { "{ " #pc }
        });

        for (i, f) in fields.iter().enumerate() {
            let fname = f.name();
            let dname = f.display_name()?;

            if pretty {
                fmt.push_str("    {}{}{}{}");
                args.push(quote! { #dname #fc });
                args.push(quote! { ": " #pc });
                args.push(quote! { ::std::format!("{}", self.#fname) #vc });
                args.push(quote! { ",\n" #pc });
            } else {
                fmt.push_str("{}{}{}");
                args.push(quote! { #dname #fc });
                args.push(quote! { ": " #pc });
                args.push(quote! { ::std::format!("{}", self.#fname) #vc });

                if i + 1 < fields.len() {
                    fmt.push_str("{}");
                    args.push(quote! { ", " #pc });
                }
            }
        }

        fmt.push_str("{}");
        args.push(if pretty {
            quote! { "}" #pc }
        } else {
            quote! { " }" #pc }
        });

        return Ok(quote! { ::std::write!(f, #fmt, #(#args),*) });
    }

    let mut fmt = String::new();
    let mut args = Vec::new();

    fmt.push_str(if pretty { "{{\n" } else { "{{ " });

    for (i, f) in fields.iter().enumerate() {
        let fname = f.name();

        if pretty {
            fmt.push_str("    ");
            fmt.push_str(&f.display_name()?);
            fmt.push_str(": {},\n");
        } else {
            fmt.push_str(&f.display_name()?);
            fmt.push_str(": {}");

            if i + 1 < fields.len() {
                fmt.push_str(", ");
            }
        }

        args.push(quote! { self.#fname });
    }

    fmt.push_str(if pretty { "}}" } else { " }}" });

    Ok(quote! { ::std::write!(f, #fmt, #(#args),*) })
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
fn render_json(fields: &[&Field], is_named: bool, pretty: bool) -> syn::Result<TokenStream> {
    if is_named {
        let inserts = fields
            .iter()
            .map(|f| {
                let fname = f.name();
                let dname = f.display_name()?;
                Ok(quote! {
                    __map.insert(
                        #dname.into(),
                        ::serde_json::to_value(&self.#fname).unwrap_or(::serde_json::Value::Null),
                    );
                })
            })
            .collect::<syn::Result<Vec<_>>>()?;

        let serialize = if pretty {
            quote! { ::serde_json::to_string_pretty(&__val) }
        } else {
            quote! { ::serde_json::to_string(&__val) }
        };

        Ok(quote! {
            let mut __map = ::serde_json::Map::new();
            #(#inserts)*
            let __val = ::serde_json::Value::Object(__map);
            ::std::write!(f, "{}", #serialize.unwrap_or_default())
        })
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

        Ok(quote! {
            let mut __arr = ::std::vec::Vec::new();
            #(#pushes)*
            let __val = ::serde_json::Value::Array(__arr);
            ::std::write!(f, "{}", #serialize.unwrap_or_default())
        })
    }
}

fn render_style(
    style: &str,
    fields: &[&Field],
    is_named: bool,
    name: &str,
    pretty: bool,
    theme: Option<&str>,
) -> syn::Result<TokenStream> {
    match style {
        "debug" => render_debug(fields, is_named, name, pretty, theme),
        "compact" => render_compact(fields),
        "keyvalue" => render_keyvalue(fields, pretty, theme),
        "map" => render_map(fields, pretty, theme),
        #[cfg(feature = "json")]
        "json" => render_json(fields, is_named, pretty),
        _ => unreachable!(),
    }
}
