use std::path::{Path, PathBuf};

#[doc(hidden)]
pub fn glob(root: impl AsRef<Path>, pattern: impl AsRef<Path>) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let pattern = pattern.as_ref();
    let mut base = root.as_ref().to_path_buf();
    let mut parts = Vec::new();
    let mut matching = false;

    for component in pattern.components() {
        let part = component.as_os_str().to_string_lossy();

        if matching || part.contains(['*', '?']) {
            matching = true;
            parts.push(part.into_owned());
        } else {
            match component {
                std::path::Component::CurDir => {}
                _ => base.push(component),
            }
        }
    }

    let mut out = vec![];
    walk(&base, &parts, &mut out)?;
    Ok(out)
}

fn walk(base: &Path, pattern: &[String], out: &mut Vec<PathBuf>) -> std::io::Result<()> {
    let Some((head, tail)) = pattern.split_first() else {
        if base.is_file() {
            out.push(base.to_path_buf());
        }

        return Ok(());
    };

    if head == "**" {
        // `**` matches zero directories.
        walk(base, tail, out)?;

        // ** matches one or more directories.
        if base.is_dir() {
            for entry in std::fs::read_dir(base)? {
                let path = entry?.path();

                if path.is_dir() {
                    walk(&path, pattern, out)?;
                }
            }
        }

        return Ok(());
    }

    if !base.is_dir() {
        return Ok(());
    }

    for entry in std::fs::read_dir(base)? {
        let entry = entry?;
        let name = entry.file_name();
        let name = name.to_string_lossy();

        if wildcard_match(head.as_bytes(), name.as_bytes()) {
            walk(&entry.path(), tail, out)?;
        }
    }

    Ok(())
}

fn wildcard_match(pattern: &[u8], value: &[u8]) -> bool {
    match pattern {
        [] => value.is_empty(),
        [b'*', rest @ ..] => wildcard_match(rest, value) || (!value.is_empty() && wildcard_match(pattern, &value[1..])),
        [b'?', rest @ ..] => !value.is_empty() && wildcard_match(rest, &value[1..]),
        [ch, rest @ ..] => value.first() == Some(ch) && wildcard_match(rest, &value[1..]),
    }
}
