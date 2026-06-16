//! The `define_visit!` macro: from one node->children spec it generates the
//! `Visit`/`VisitMut` traits (one method per node, default body recurses) and the
//! free walker functions those defaults delegate to.
//!
//! A single invocation owns both traits and every walker, the same way
//! `define_leaf!` owns whole types. Per node the macro emits four things: the
//! immutable trait method, the mutable trait method, the immutable walker, and
//! the mutable walker.
//!
//! `macro_rules!` cannot build new identifiers, so the spec names all four
//! symbols per node explicitly: `visit_x` / `visit_x_mut` (trait methods) and
//! `walk_x` / `walk_x_mut` (free walkers). Per child field the spec also names
//! the immutable and mutable visitor methods to call. This keeps the macro pure
//! `macro_rules!` with no proc-macro / `paste` dependency.

/// Generate `Visit`, `VisitMut`, and all walkers from a node spec.
///
/// Each node is declared as:
/// ```ignore
/// struct ExprBinary {
///     visit:  visit_expr_binary,  visit_mut:  visit_expr_binary_mut,
///     walk:   walk_expr_binary,   walk_mut:   walk_expr_binary_mut,
///     fields {
///         attrs            => visit_attributes / visit_attributes_mut,
///         left:  box       => visit_expr / visit_expr_mut,
///         op:    leaf,
///         right: box       => visit_expr / visit_expr_mut,
///     }
/// }
/// enum Expr {
///     visit: visit_expr, visit_mut: visit_expr_mut,
///     walk:  walk_expr,  walk_mut:  walk_expr_mut,
///     variants {
///         Unary(visit_unary_expr / visit_unary_expr_mut),
///         Infer,            // childless
///         Verbatim(skip),   // leaf payload
///     }
/// }
/// ```
macro_rules! define_visit {
    (
        $(
            $kind:ident $name:ident {
                visit: $visit:ident, visit_mut: $visit_mut:ident,
                walk:  $walk:ident,  walk_mut:  $walk_mut:ident,
                $section:ident { $($body:tt)* }
            }
        )+
    ) => {
        pub trait Visit<'ast>: Sized {
            $(
                #[allow(unused_variables)]
                fn $visit(&mut self, node: &'ast $name) { $walk(self, node) }
            )+
        }

        pub trait VisitMut: Sized {
            $(
                #[allow(unused_variables)]
                fn $visit_mut(&mut self, node: &mut $name) { $walk_mut(self, node) }
            )+
        }

        $(
            define_visit!(@walk $kind $name, $walk, $walk_mut, $section { $($body)* });
        )+
    };

    // `manual` nodes: emit only the trait methods; the `walk_*` / `walk_*_mut`
    // functions they delegate to are hand-written elsewhere in the module.
    (@walk manual $name:ident, $walk:ident, $walk_mut:ident, manual { }) => {};

    // ---- struct walkers --------------------------------------------------
    (@walk struct $name:ident, $walk:ident, $walk_mut:ident, fields { $($body:tt)* }) => {
        #[allow(unused_variables)]
        pub fn $walk<'ast, V: Visit<'ast>>(v: &mut V, node: &'ast $name) {
            define_visit!(@fields ref v, node, $($body)*);
        }
        #[allow(unused_variables)]
        pub fn $walk_mut<V: VisitMut>(v: &mut V, node: &mut $name) {
            define_visit!(@fields mut v, node, $($body)*);
        }
    };

    // ---- enum walkers ----------------------------------------------------
    (@walk enum $name:ident, $walk:ident, $walk_mut:ident, variants { $($body:tt)* }) => {
        #[allow(unused_variables)]
        pub fn $walk<'ast, V: Visit<'ast>>(v: &mut V, node: &'ast $name) {
            define_visit!(@variants ref v, node, $name, () $($body)*);
        }
        #[allow(unused_variables)]
        pub fn $walk_mut<V: VisitMut>(v: &mut V, node: &mut $name) {
            define_visit!(@variants mut v, node, $name, () $($body)*);
        }
    };

    // ===== struct field dispatch =========================================
    // Walk each field in order. `$m` is `ref` or `mut`. Each non-trivial field
    // names both the immutable (`$vm`) and mutable (`$vmm`) visitor method.

    (@fields $m:tt $v:ident, $node:ident, ) => {};

    // bare:   field => vm / vmm   (T)
    (@fields ref $v:ident, $node:ident, $f:ident => $vm:ident / $vmm:ident, $($rest:tt)*) => {
        $v.$vm(&$node.$f);
        define_visit!(@fields ref $v, $node, $($rest)*);
    };
    (@fields mut $v:ident, $node:ident, $f:ident => $vm:ident / $vmm:ident, $($rest:tt)*) => {
        $v.$vmm(&mut $node.$f);
        define_visit!(@fields mut $v, $node, $($rest)*);
    };

    // box:    field: box => vm / vmm   (Box<T>)
    (@fields ref $v:ident, $node:ident, $f:ident: box => $vm:ident / $vmm:ident, $($rest:tt)*) => {
        $v.$vm(&$node.$f);
        define_visit!(@fields ref $v, $node, $($rest)*);
    };
    (@fields mut $v:ident, $node:ident, $f:ident: box => $vm:ident / $vmm:ident, $($rest:tt)*) => {
        $v.$vmm(&mut $node.$f);
        define_visit!(@fields mut $v, $node, $($rest)*);
    };

    // opt:    field: opt => vm / vmm   (Option<T>)
    (@fields ref $v:ident, $node:ident, $f:ident: opt => $vm:ident / $vmm:ident, $($rest:tt)*) => {
        if let Some(x) = &$node.$f { $v.$vm(x); }
        define_visit!(@fields ref $v, $node, $($rest)*);
    };
    (@fields mut $v:ident, $node:ident, $f:ident: opt => $vm:ident / $vmm:ident, $($rest:tt)*) => {
        if let Some(x) = &mut $node.$f { $v.$vmm(x); }
        define_visit!(@fields mut $v, $node, $($rest)*);
    };

    // opt_box: field: opt_box => vm / vmm   (Option<Box<T>>)
    (@fields ref $v:ident, $node:ident, $f:ident: opt_box => $vm:ident / $vmm:ident, $($rest:tt)*) => {
        if let Some(x) = &$node.$f { $v.$vm(x); }
        define_visit!(@fields ref $v, $node, $($rest)*);
    };
    (@fields mut $v:ident, $node:ident, $f:ident: opt_box => $vm:ident / $vmm:ident, $($rest:tt)*) => {
        if let Some(x) = &mut $node.$f { $v.$vmm(x); }
        define_visit!(@fields mut $v, $node, $($rest)*);
    };

    // vec:    field: vec => vm / vmm   (Vec<T>)
    (@fields ref $v:ident, $node:ident, $f:ident: vec => $vm:ident / $vmm:ident, $($rest:tt)*) => {
        for x in &$node.$f { $v.$vm(x); }
        define_visit!(@fields ref $v, $node, $($rest)*);
    };
    (@fields mut $v:ident, $node:ident, $f:ident: vec => $vm:ident / $vmm:ident, $($rest:tt)*) => {
        for x in &mut $node.$f { $v.$vmm(x); }
        define_visit!(@fields mut $v, $node, $($rest)*);
    };

    // punct:  field: punct => vm / vmm   (Punctuated<T, P>)
    (@fields ref $v:ident, $node:ident, $f:ident: punct => $vm:ident / $vmm:ident, $($rest:tt)*) => {
        for x in $node.$f.iter() { $v.$vm(x); }
        define_visit!(@fields ref $v, $node, $($rest)*);
    };
    (@fields mut $v:ident, $node:ident, $f:ident: punct => $vm:ident / $vmm:ident, $($rest:tt)*) => {
        for x in $node.$f.iter_mut() { $v.$vmm(x); }
        define_visit!(@fields mut $v, $node, $($rest)*);
    };

    // seq:    field: seq => vm / vmm   (Delimited<Vec<T>>)
    (@fields ref $v:ident, $node:ident, $f:ident: seq => $vm:ident / $vmm:ident, $($rest:tt)*) => {
        for x in &$node.$f.inner { $v.$vm(x); }
        define_visit!(@fields ref $v, $node, $($rest)*);
    };
    (@fields mut $v:ident, $node:ident, $f:ident: seq => $vm:ident / $vmm:ident, $($rest:tt)*) => {
        for x in &mut $node.$f.inner { $v.$vmm(x); }
        define_visit!(@fields mut $v, $node, $($rest)*);
    };

    // seq_punct: field: seq_punct => vm / vmm   (Delimited<Punctuated<T, P>>)
    (@fields ref $v:ident, $node:ident, $f:ident: seq_punct => $vm:ident / $vmm:ident, $($rest:tt)*) => {
        for x in $node.$f.inner.iter() { $v.$vm(x); }
        define_visit!(@fields ref $v, $node, $($rest)*);
    };
    (@fields mut $v:ident, $node:ident, $f:ident: seq_punct => $vm:ident / $vmm:ident, $($rest:tt)*) => {
        for x in $node.$f.inner.iter_mut() { $v.$vmm(x); }
        define_visit!(@fields mut $v, $node, $($rest)*);
    };

    // delim:  field: delim => vm / vmm   (Delimited<T>)
    (@fields ref $v:ident, $node:ident, $f:ident: delim => $vm:ident / $vmm:ident, $($rest:tt)*) => {
        $v.$vm(&$node.$f.inner);
        define_visit!(@fields ref $v, $node, $($rest)*);
    };
    (@fields mut $v:ident, $node:ident, $f:ident: delim => $vm:ident / $vmm:ident, $($rest:tt)*) => {
        $v.$vmm(&mut $node.$f.inner);
        define_visit!(@fields mut $v, $node, $($rest)*);
    };

    // opt_pair: field: opt_pair => vm / vmm   (Option<(Tok, Node)>; node is .1)
    (@fields ref $v:ident, $node:ident, $f:ident: opt_pair => $vm:ident / $vmm:ident, $($rest:tt)*) => {
        if let Some((_, x)) = &$node.$f { $v.$vm(x); }
        define_visit!(@fields ref $v, $node, $($rest)*);
    };
    (@fields mut $v:ident, $node:ident, $f:ident: opt_pair => $vm:ident / $vmm:ident, $($rest:tt)*) => {
        if let Some((_, x)) = &mut $node.$f { $v.$vmm(x); }
        define_visit!(@fields mut $v, $node, $($rest)*);
    };

    // leaf / skip: no recursion
    (@fields $m:tt $v:ident, $node:ident, $f:ident: leaf, $($rest:tt)*) => {
        define_visit!(@fields $m $v, $node, $($rest)*);
    };
    (@fields $m:tt $v:ident, $node:ident, $f:ident: skip, $($rest:tt)*) => {
        define_visit!(@fields $m $v, $node, $($rest)*);
    };

    // ===== enum variant dispatch =========================================
    // Build a match by peeling variants into an accumulator (leaf.rs pattern).

    // variant with a node payload: delegate to its visitor
    (@variants ref $v:ident, $node:ident, $name:ident, ($($arms:tt)*)
        $variant:ident ($vm:ident / $vmm:ident), $($rest:tt)*
    ) => {
        define_visit!(@variants ref $v, $node, $name,
            ($($arms)* $name::$variant(inner) => $v.$vm(inner),) $($rest)*)
    };
    (@variants mut $v:ident, $node:ident, $name:ident, ($($arms:tt)*)
        $variant:ident ($vm:ident / $vmm:ident), $($rest:tt)*
    ) => {
        define_visit!(@variants mut $v, $node, $name,
            ($($arms)* $name::$variant(inner) => $v.$vmm(inner),) $($rest)*)
    };

    // variant whose payload is a leaf/skip: no recursion
    (@variants $m:tt $v:ident, $node:ident, $name:ident, ($($arms:tt)*)
        $variant:ident (skip), $($rest:tt)*
    ) => {
        define_visit!(@variants $m $v, $node, $name,
            ($($arms)* $name::$variant(_) => {},) $($rest)*)
    };

    // childless variant
    (@variants $m:tt $v:ident, $node:ident, $name:ident, ($($arms:tt)*)
        $variant:ident, $($rest:tt)*
    ) => {
        define_visit!(@variants $m $v, $node, $name,
            ($($arms)* $name::$variant => {},) $($rest)*)
    };

    (@variants $m:tt $v:ident, $node:ident, $name:ident, ($($arms:tt)*)) => {
        match $node { $($arms)* }
    };
}
