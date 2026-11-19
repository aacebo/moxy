use std::cell::Cell;

/// The deepest nesting that lexing, parsing, and skipping accept. Deeper input
/// is an error instead of a stack overflow.
///
/// Lexing counts each delimited group as one level. Parsing counts each nested
/// operand, group, block, pattern, type, generic argument, module, assignment,
/// `else if` branch, or use path segment as one level, and charges the forms
/// whose parse takes more stack more levels:
///
/// - Two levels: a struct literal or struct pattern, a closure with a return
///   type, a function pointer type, parenthesized path arguments as in
///   `Fn() -> T`, an array type, and an item inside a block.
/// - Three levels: an associated type or constant argument (`A<B = T>`), an
///   associated type bound (`A<B: C>`), and a member of an impl, trait, or
///   extern block.
/// - Half a level: each step of a chain such as `a + b + c`, `a.b().c()`,
///   `a??`, or `a as T as U`, whose tree nests one node deeper per step. A
///   chain counts with its tallest operand, since emitting, spanning,
///   cloning, and dropping the tree recurse through it.
///
/// A level takes up to about 12 KiB of stack in a debug build and less in a
/// release build, so input at the limit parses and traverses in under 1.9 MiB
/// on the 2 MiB stack of a spawned thread or a test. rustc grows its stack to
/// accept any depth; syn parses 256 nested parentheses on an 8 MiB stack and
/// overflows a 2 MiB stack at that depth in a debug build.
pub const MAX_DEPTH: usize = 144;

// Each level costs two units, so a step of a chain such as `a.b().c()` or
// `a + b + c`, which nests the tree half as deeply in stack, costs one
const UNITS_PER_LEVEL: usize = 2;

thread_local! {
    static DEPTH: Cell<usize> = const { Cell::new(0) };
    // The deepest the depth reached since the operand that a `Chain` measures
    // began
    static PEAK: Cell<usize> = const { Cell::new(0) };
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
        Self::enter_units(levels * UNITS_PER_LEVEL)
    }

    /// Half a level, for one step of a chain of operators or method calls,
    /// which the tree nests one node deeper.
    pub fn enter_chain() -> Option<Self> {
        Self::enter_units(1)
    }

    fn enter_units(units: usize) -> Option<Self> {
        DEPTH.with(|depth| {
            if depth.get() + units > MAX_DEPTH * UNITS_PER_LEVEL {
                return None;
            }

            depth.set(depth.get() + units);
            PEAK.with(|peak| peak.set(peak.get().max(depth.get())));
            Some(Self(units))
        })
    }
}

impl Drop for Depth {
    fn drop(&mut self) {
        DEPTH.with(|depth| depth.set(depth.get() - self.0));
    }
}

/// The height of a chain of operators or postfix operations, such as
/// `a + b + c` or `a.b().c()`, whose tree nests one node deeper per step
/// although the parser reads it in a loop. Every traversal of the tree, such
/// as emitting, cloning, or dropping it, recurses that deep, so the steps and
/// the tallest operand count against [`MAX_DEPTH`]. Public only for the parser
/// crate.
#[doc(hidden)]
pub struct Chain {
    start: usize,
    saved: usize,
    steps: usize,
    height: usize,
}

impl Chain {
    /// Starts a chain before its first operand.
    pub fn begin() -> Self {
        let start = DEPTH.with(Cell::get);
        let saved = PEAK.with(|peak| peak.replace(start));
        Self {
            start,
            saved,
            steps: 0,
            height: 0,
        }
    }

    /// Records the height of the operand read since the last call.
    pub fn operand(&mut self) {
        let peak = PEAK.with(|peak| peak.replace(self.start));
        self.height = self.height.max(peak - self.start);
    }

    /// One more step, or `false` when the chain would nest too deeply.
    pub fn step(&mut self) -> bool {
        self.steps += 1;
        self.start + self.steps + self.height <= MAX_DEPTH * UNITS_PER_LEVEL
    }
}

impl Drop for Chain {
    // An enclosing chain sees this chain's whole height as its operand's
    fn drop(&mut self) {
        let top = self.start + self.steps + self.height;
        PEAK.with(|peak| peak.set(self.saved.max(top).max(peak.get())));
    }
}
