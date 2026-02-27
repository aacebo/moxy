# Option Fields

`Option<T>` fields are handled automatically — the setter accepts the inner type `T` and wraps it in `Some`. This is consistent with the Build macro's `Option<T>` handling.

```rust
use moxy::Set;

#[derive(Set)]
struct Profile {
    #[moxy(set)]
    bio: Option<String>,
}

let mut p = Profile { bio: None };

// Accepts &str (Into<String>), wraps in Some
p.set_bio("hello");
assert_eq!(p.bio, Some("hello".to_string()));
```

The generated setter signature is `fn set_bio<V: Into<String>>(&mut self, value: V) -> &mut Self`.
