use std::cell::Cell;

/// The deepest nesting that lexing, parsing, and skipping accept. Deeper input
/// is an error instead of a stack overflow.
///
/// Lexing counts each delimited group as one level. Parsing counts each nested
/// operand, group, block, pattern, type, generic argument, module, or
/// assignment as one level, and each chain of binary operators whose operands
/// hold a chain of stronger operators as one more, as in `a || b && c`. It
/// charges the forms whose parse takes more stack more levels:
///
/// - Two levels: a struct literal or struct pattern, a closure with a return
///   type, a function pointer type, parenthesized path arguments as in
///   `Fn() -> T`, an array type, and an item inside a block.
/// - Three levels: an associated type or constant argument (`A<B = T>`), an
///   associated type bound (`A<B: C>`), and a member of an impl, trait, or
///   extern block.
///
/// The steps of one chain of binary operators, postfix operations, casts,
/// `else if` branches, or use path segments are held flat, so a chain counts
/// as one level however long it is.
///
/// A level takes up to about 12 KiB of stack in a debug build and less in a
/// release build, so input at the limit parses and traverses in under 1.9 MiB
/// on the 2 MiB stack of a spawned thread or a test. rustc grows its stack to
/// accept any depth; syn parses 256 nested parentheses on an 8 MiB stack and
/// overflows a 2 MiB stack at that depth in a debug build.
pub const MAX_DEPTH: usize = 144;

thread_local! {
    static DEPTH: Cell<usize> = const { Cell::new(0) };
}

/// Nesting held while a nested group or node is read, or `None` past
/// [`MAX_DEPTH`]. Public only for the parser crate.
#[doc(hidden)]
pub struct Depth(usize);

impl Depth {
    /// One level of nesting.
    pub fn enter() -> Option<Self> {
        Self::enter_by(1)
    }

    /// `levels` levels of nesting, for a node whose parse takes that many
    /// levels' worth of stack, such as an item inside a block.
    pub fn enter_by(levels: usize) -> Option<Self> {
        DEPTH.with(|depth| {
            if depth.get() + levels > MAX_DEPTH {
                return None;
            }

            depth.set(depth.get() + levels);
            Some(Self(levels))
        })
    }
}

impl Drop for Depth {
    fn drop(&mut self) {
        DEPTH.with(|depth| depth.set(depth.get() - self.0));
    }
}
