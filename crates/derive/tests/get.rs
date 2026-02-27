use std::sync::Arc;

use moxy_derive::Get;

#[derive(Get)]
struct User {
    #[moxy(get)]
    name: String,
    #[moxy(get)]
    email: String,
    #[allow(dead_code)]
    password_hash: String,
}

#[test]
fn test_ref_getter() {
    let user = User {
        name: "alice".into(),
        email: "a@b.com".into(),
        password_hash: "hash".into(),
    };

    assert_eq!(user.name(), "alice");
    assert_eq!(user.email(), "a@b.com");
}

#[derive(Get)]
struct Flags {
    #[moxy(get)]
    is_active: bool,
    #[moxy(get)]
    verified: bool,
}

#[test]
fn test_bool_getter() {
    let flags = Flags {
        is_active: true,
        verified: false,
    };

    assert_eq!(flags.is_active(), true);
    assert_eq!(flags.verified(), false);
}

#[derive(Get)]
struct Profile {
    #[allow(dead_code)]
    name: String,
    #[moxy(get)]
    bio: Option<String>,
}

#[test]
fn test_option_getter() {
    let p = Profile {
        name: "alice".into(),
        bio: Some("hello".into()),
    };

    let bio: Option<&str> = p.bio();
    assert_eq!(bio, Some("hello"));
}

#[test]
fn test_option_getter_none() {
    let p = Profile {
        name: "alice".into(),
        bio: None,
    };

    assert_eq!(p.bio(), None);
}

#[derive(Get)]
struct Metrics {
    #[moxy(get(copy))]
    count: u32,
}

#[test]
fn test_copy_getter() {
    let m = Metrics { count: 42 };
    let v: u32 = m.count();
    assert_eq!(v, 42);
}

#[derive(Get)]
struct Labels {
    #[moxy(get(clone))]
    label: Arc<String>,
}

#[test]
fn test_clone_getter() {
    let l = Labels {
        label: Arc::new("test".into()),
    };

    let v: Arc<String> = l.label();
    assert_eq!(*v, "test");
}

#[derive(Get)]
struct Buffer {
    #[moxy(get(mutable))]
    data: Vec<u8>,
}

#[test]
fn test_mut_getter() {
    let b = Buffer {
        data: vec![1, 2, 3],
    };
    assert_eq!(b.data(), &[1, 2, 3]);
}

#[test]
fn test_mut_getter_mutate() {
    let mut b = Buffer {
        data: vec![1, 2, 3],
    };
    b.data_mut().push(4);
    assert_eq!(b.data(), &[1, 2, 3, 4]);
}

#[derive(Get)]
struct Row {
    #[moxy(get(copy, "id"))]
    row_id: u64,
}

#[test]
fn test_custom_name() {
    let r = Row { row_id: 99 };
    assert_eq!(r.id(), 99);
}

static mut GET_CALLBACK_COUNT: u32 = 0;

#[derive(Get)]
struct Tracked {
    #[moxy(get(on = unsafe { GET_CALLBACK_COUNT += 1 }))]
    value: String,
}

#[test]
fn test_on_callback() {
    let t = Tracked {
        value: "hello".into(),
    };

    unsafe { GET_CALLBACK_COUNT = 0 };
    assert_eq!(t.value(), "hello");
    assert_eq!(unsafe { GET_CALLBACK_COUNT }, 1);
    assert_eq!(t.value(), "hello");
    assert_eq!(unsafe { GET_CALLBACK_COUNT }, 2);
}

#[derive(Get)]
struct WithDocs {
    /// The user's name
    #[moxy(get)]
    name: String,
}

#[test]
fn test_doc_forwarding() {
    let w = WithDocs {
        name: "alice".into(),
    };
    assert_eq!(w.name(), "alice");
}
