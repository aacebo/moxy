use std::cell::Cell;

/// The deepest nesting that lexing, parsing, and skipping accept. Deeper input
/// is an error instead of a stack overflow.
///
/// Lexing counts each delimited group as one level and reports deeper groups as
/// "delimiters nest deeper than 144 levels". Converting a `proc_macro` or
/// `proc_macro2` stream counts groups the same way:
/// [`TryIntoTokenStream::try_into_token_stream`](crate::TryIntoTokenStream)
/// returns that error, and `From` panics with it. Real source and macro
/// bodies do not nest groups that deep.
///
/// Token streams and syntax trees built in code are not limited. Dropping a
/// token stream does not recurse, and
/// [`TokenStream::nests_deeper_than`](crate::TokenStream::nests_deeper_than)
/// checks its groups without recursing; cloning, comparing, printing, and
/// converting one into a `proc_macro` or `proc_macro2` stream recurse once per
/// group. With the `visit` feature, the `nests_past_max_depth` method of a
/// file, item, statement, block, expression, type, or pattern checks a syntax
/// tree against [`MAX_NESTED_NODES`], and formatting counts the same nodes and
/// reports a tree past it as an error. Emitting, cloning, comparing,
/// printing, serializing, visiting, and dropping a syntax tree recurse once
/// per level of nesting and check nothing, so a tree that fails the check can
/// overflow the stack.
///
/// Parsing reports its own limit as "nesting is deeper than 144 levels". It
/// counts each nested operand, group, block, pattern, type, generic argument,
/// module, assignment, or set of match arms as one level, and each chain of
/// binary operators whose operands hold a chain of stronger operators as one
/// more, as in `a || b && c`. It charges the forms whose parse takes more
/// stack more levels:
///
/// - Two levels: a call, method call, or index together with its arguments,
///   as in `f(f(x))`, whose argument parse frames are large; a postfix chain
///   or cast whose base is a group, as in `(x).a()`; the final `else` block
///   of an `if`; a binding's subpattern, as in `a @ b @ c`; the elements of a
///   tuple struct pattern; an `impl` type; a bound in parentheses; a struct
///   literal or struct pattern, a closure with a return type, a function
///   pointer type, parenthesized path arguments as in `Fn() -> T`, an array
///   type, a qualified self type as in `<T as A>::B`, an item inside a block,
///   a module's body, a `let` initializer, a `const` block in a pattern, and
///   an associated type or constant binding (`A<B = T>`), whose value takes
///   the second level.
/// - Three levels: an associated type bound (`A<B: C>`), whose bounds take
///   the second and third, and a member of an impl, trait, or extern block.
///
/// The steps of one chain of binary operators, postfix operations, casts,
/// `else if` branches, or use path segments are held flat, so a chain counts
/// as one level however long it is. A chain of postfix operations or casts,
/// and each of those that continue a block-like statement as in
/// `{ x }.a() as T`, counts a level above the base it wraps, as walking the
/// tree nests through it; so does a chain of binary operators that takes an
/// earlier chain as its first operand, as `+` in `a * b + c`, and a range or
/// assignment whose left side is such a chain, as in `a + b..`, and an
/// assignment that takes the assignment before a range as its left side, as
/// in `a = 0.. = 0`. The end of a range is one level. The outer attributes of
/// an expression or a closure parameter count from the level of the node they
/// belong to, and a block-like statement or match arm parsed alone takes the
/// level that a block or match around it would give it. In unparsed tokens,
/// such as a macro body or attribute arguments, each nested group takes a level
/// because traversal descends through each group. Skipping takes the same
/// levels as parsing.
///
/// The count is the same on every build and platform, so a source parses or
/// fails alike everywhere. It is sized for the 2 MiB stack of a spawned thread
/// or a test in a debug build: the depth tests parse input at the limit of
/// each kind and run every traversal on a 1.5 MiB thread. A caller on a
/// smaller stack, such as a 1 MiB main thread on Windows, should parse on a
/// thread with at least 2 MiB.
pub const MAX_DEPTH: usize = 144;

/// The most nested nodes a syntax tree may hold, for a tree built in code.
///
/// Expressions, types, patterns, items, blocks, use groups, and use path
/// chains count one node each, an associated argument two, its bounds or
/// value four or two more, and each group nested in a macro body or attribute
/// arguments two: half a level of [`MAX_DEPTH`] for each level the parser
/// charges them. Trees built to the limit run every traversal on the stack
/// that `MAX_DEPTH` is sized for when they nest through unary, list, and block
/// expressions, references, patterns, types, use trees, associated arguments,
/// macro bodies, items in blocks, impl members, match arms, struct literals,
/// or closures with a return type: those shapes are tested, other shapes are
/// not.
///
/// Parsing does not count nodes, since walking the tree once more would add
/// about an eighth to the cost of a parse. It charges levels so that its
/// trees stay within the limit, which is tested, not proven: for every
/// construct of the depth tests at its deepest, and for sampled pairs of them
/// nested in each other around leaves and with suffixes that add nodes
/// without levels.
pub const MAX_NESTED_NODES: usize = 2 * MAX_DEPTH;

thread_local! {
    static DEPTH: Cell<usize> = const { Cell::new(0) };
    // The deepest nesting reached in the subtree being read, with the levels
    // of nodes that wrap a part of it after that part is read
    static HEIGHT: Cell<usize> = const { Cell::new(0) };
    // Whether entering a level failed since the innermost `Watch` began
    static EXCEEDED: Cell<bool> = const { Cell::new(false) };
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

    /// Whether no level is held, as for a node parsed as the root of its tree.
    pub fn is_outermost() -> bool {
        DEPTH.with(Cell::get) == 0
    }

    /// A level for each group that `tokens` nest, for tokens a node holds
    /// unparsed, such as a macro body, which a traversal walks as deep as they
    /// nest.
    pub fn enter_tokens(tokens: &crate::TokenStream) -> Option<Self> {
        Self::enter_by(tokens.nesting())
    }

    /// `levels` levels of nesting, for a node whose parse takes that many
    /// levels' worth of stack, such as an item inside a block.
    pub fn enter_by(levels: usize) -> Option<Self> {
        DEPTH.with(|depth| {
            if depth.get() + levels > MAX_DEPTH {
                EXCEEDED.with(|exceeded| exceeded.set(true));
                return None;
            }

            depth.set(depth.get() + levels);
            HEIGHT.with(|height| height.set(height.get().max(depth.get())));
            Some(Self(levels))
        })
    }

    /// Starts measuring the height of the subtree read next, for a node that
    /// learns only after reading it that it wraps it, as a method call wraps
    /// the block before it in `{ x }.a()`.
    pub fn subtree() -> Subtree {
        let base = DEPTH.with(Cell::get);
        let saved = HEIGHT.with(|height| height.replace(base));
        Subtree { saved, base }
    }
}

/// Watches whether a lookahead fails for the depth limit rather than for its
/// tokens, so a peek can leave the error to the parse. Public only for the
/// parser crate.
#[doc(hidden)]
pub struct Watch(bool);

impl Watch {
    /// Starts watching.
    pub fn start() -> Self {
        Self(EXCEEDED.with(|exceeded| exceeded.replace(false)))
    }

    /// Whether entering a level failed since the watch started.
    pub fn exceeded(&self) -> bool {
        EXCEEDED.with(Cell::get)
    }
}

impl Drop for Watch {
    fn drop(&mut self) {
        let outer = self.0;
        EXCEEDED.with(|exceeded| exceeded.set(exceeded.get() || outer));
    }
}

/// The height of a subtree being read. Public only for the parser crate.
#[doc(hidden)]
pub struct Subtree {
    saved: usize,
    base: usize,
}

impl Subtree {
    /// The levels that the part of the subtree read so far nests below its
    /// start.
    pub fn height(&self) -> usize {
        HEIGHT.with(Cell::get) - self.base
    }

    /// Counts a node that wraps a part of the subtree `part` levels high, a
    /// level above that part, or `None` when that takes the subtree past
    /// [`MAX_DEPTH`].
    pub fn wrap(&self, part: usize) -> Option<()> {
        let wrapped = self.base + part + 1;

        HEIGHT.with(|height| (wrapped <= MAX_DEPTH).then(|| height.set(height.get().max(wrapped))))
    }
}

impl Drop for Subtree {
    fn drop(&mut self) {
        HEIGHT.with(|height| height.set(height.get().max(self.saved)));
    }
}

impl Drop for Depth {
    fn drop(&mut self) {
        DEPTH.with(|depth| depth.set(depth.get() - self.0));
    }
}
