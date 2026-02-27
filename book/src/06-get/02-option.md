# Option Fields

`Option<T>` fields return `Option<&Deref::Target>` via `.as_deref()`. For `Option<String>`, this means `Option<&str>` — more ergonomic than `&Option<String>` or `Option<&String>`.

```rust
use moxy::Get;

#[derive(Get)]
struct Profile {
    #[moxy(get)]
    bio: Option<String>,
}

let p = Profile { bio: Some("hello".into()) };
let bio: Option<&str> = p.bio();
assert_eq!(bio, Some("hello"));

let p = Profile { bio: None };
assert_eq!(p.bio(), None);
```
