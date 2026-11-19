use std::cell::Cell;

/// The deepest nesting that lexing, parsing, and skipping accept. Deeper input
/// is an error instead of a stack overflow.
///
/// Lexing counts each delimited group as one level. Parsing counts each nested
/// operand, group, block, pattern, type, generic argument, module, assignment,
/// or `else if` branch as one level.
///
/// A level of nesting takes up to about 12 KiB of stack in a debug build and
/// 5 KiB in a release build, so input at the limit parses in under 1.7 MiB on
/// the 2 MiB stack of a spawned thread or a test, with room for the caller.
/// rustc grows its stack to accept any depth; syn parses 256 nested
/// parentheses on an 8 MiB stack and overflows a 2 MiB stack at that depth in
/// a debug build.
pub const MAX_DEPTH: usize = 144;

thread_local! {
    static DEPTH: Cell<usize> = const { Cell::new(0) };
}

/// One level of nesting, held while a nested group or node is read, or `None`
/// past [`MAX_DEPTH`]. Public only for the parser crate.
#[doc(hidden)]
pub struct Depth(());

impl Depth {
    pub fn enter() -> Option<Self> {
        DEPTH.with(|depth| {
            if depth.get() >= MAX_DEPTH {
                return None;
            }

            depth.set(depth.get() + 1);
            Some(Self(()))
        })
    }
}

impl Drop for Depth {
    fn drop(&mut self) {
        DEPTH.with(|depth| depth.set(depth.get() - 1));
    }
}
