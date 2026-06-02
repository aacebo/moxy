use syn::{Expr, Path, Type};

/// Options read by the `Parse` derive from `#[parse(...)]`.
#[derive(Default)]
pub struct ParseOptions {
    pub skip: bool,
    pub value: Option<Expr>,
    pub call: Option<Path>,
    pub peek: Option<Path>,
    pub separated: bool,
    pub terminated: bool,
    pub prefix: Option<Path>,
    pub suffix: Option<Path>,
    pub group: Option<&'static str>,
}

impl ParseOptions {
    pub fn parse(attrs: &[syn::Attribute]) -> syn::Result<Self> {
        let mut opts = ParseOptions::default();

        each_meta(attrs, |key, meta| {
            match key {
                "skip" => opts.skip = true,
                "separated" => opts.separated = true,
                "terminated" => opts.terminated = true,
                "paren" => opts.group = Some("Paren"),
                "bracket" => opts.group = Some("Bracket"),
                "brace" => opts.group = Some("Brace"),
                "value" => opts.value = Some(meta.value()?.parse()?),
                "call" => opts.call = Some(meta.value()?.parse()?),
                "peek" => opts.peek = Some(meta.value()?.parse()?),
                "prefix" => opts.prefix = Some(meta.value()?.parse()?),
                "suffix" => opts.suffix = Some(meta.value()?.parse()?),
                other => return Err(meta.error(format!("unknown parse option `{other}`"))),
            }

            Ok(())
        })?;

        Ok(opts)
    }
}

/// Options read by the `ToTokens` derive from `#[parse(...)]`. Parse-only
/// options (`call`, `separated`, `terminated`) are accepted but ignored.
#[derive(Default)]
pub struct ToTokenOptions {
    pub skip: bool,
    pub value: bool,
    pub peek: Option<Path>,
    pub prefix: Option<Path>,
    pub suffix: Option<Path>,
    pub group: Option<&'static str>,
}

impl ToTokenOptions {
    pub fn parse(attrs: &[syn::Attribute]) -> syn::Result<Self> {
        let mut opts = ToTokenOptions::default();

        each_meta(attrs, |key, meta| {
            match key {
                "skip" => opts.skip = true,
                "value" => opts.value = true,
                "paren" => opts.group = Some("Paren"),
                "bracket" => opts.group = Some("Bracket"),
                "brace" => opts.group = Some("Brace"),
                "peek" => opts.peek = Some(meta.value()?.parse()?),
                "prefix" => opts.prefix = Some(meta.value()?.parse()?),
                "suffix" => opts.suffix = Some(meta.value()?.parse()?),
                // Parse-only options — consume any payload and ignore.
                "call" | "separated" | "terminated" => {
                    if matches!(key, "call") {
                        let _: Path = meta.value()?.parse()?;
                    }
                }
                other => return Err(meta.error(format!("unknown parse option `{other}`"))),
            }

            Ok(())
        })?;

        Ok(opts)
    }
}

fn each_meta(
    attrs: &[syn::Attribute],
    mut f: impl FnMut(&str, syn::meta::ParseNestedMeta) -> syn::Result<()>,
) -> syn::Result<()> {
    for attr in attrs {
        if !attr.path().is_ident("parse") {
            continue;
        }

        attr.parse_nested_meta(|meta| {
            let key = meta.path.get_ident().map(|i| i.to_string()).unwrap_or_default();

            f(&key, meta)
        })?;
    }

    Ok(())
}

pub fn type_is(ty: &Type, name: &str) -> bool {
    matches!(ty, Type::Path(p) if p.path.segments.last().is_some_and(|s| s.ident == name))
}
