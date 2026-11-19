// Canonical renderings of moxy and syn trees: every operation in parentheses and
// every node tagged with its kind, so two trees compare by shape
use moxy::ast as m;
use moxy::token::ToTokenStream;
use quote::ToTokens;

// syn gives syntax it does not support as opaque tokens, which only the emitted
// tokens can compare
pub(super) const SYN_VERBATIM: &str = "<syn verbatim>";

fn grp(inner: String) -> String {
    format!("({inner})")
}

pub(super) fn strip(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

fn mt<T: ToTokenStream>(t: &T) -> String {
    strip(&t.to_token_stream().to_string())
}

fn st<T: ToTokens>(t: &T) -> String {
    strip(&t.to_token_stream().to_string())
}

pub(super) fn mattrs(a: &m::Attributes) -> String {
    let s = mt(a);
    if s.is_empty() { s } else { format!("@[{s}]") }
}

pub(super) fn sattrs(a: &[syn::Attribute]) -> String {
    let s: String = a.iter().map(st).collect();
    if s.is_empty() { s } else { format!("@[{s}]") }
}

fn mopt(e: &Option<Box<m::Expr>>) -> String {
    e.as_ref().map(|e| mexpr(e)).unwrap_or_default()
}

fn sopt(e: &Option<Box<syn::Expr>>) -> String {
    e.as_ref().map(|e| sexpr(e)).unwrap_or_default()
}

pub(super) fn mblock(b: &m::StmtBlock) -> String {
    let mut s = String::from("{");
    for stmt in b.stmts.iter() {
        s.push_str(&mstmt(stmt));
        s.push(' ');
    }
    s.push('}');
    s
}

fn mstmt(stmt: &m::Stmt) -> String {
    match stmt {
        m::Stmt::Empty(_) => ";".into(),
        m::Stmt::Local(l) => {
            let mut s = format!("{}let {}", mattrs(&l.attrs), mpat(&l.pat));
            if let Some((_, ty)) = &l.ty {
                s.push_str(&format!(":{}", mt(ty)));
            }
            if let Some(init) = &l.init {
                s.push_str(&format!(" = {}", mexpr(&init.expr)));
                if let Some((_, e)) = &init.diverge {
                    s.push_str(&format!(" else {}", mexpr(e)));
                }
            }
            if l.semi.is_some() {
                s.push(';');
            }
            s
        }
        m::Stmt::Block(b) => mblock(b),
        m::Stmt::Item(i) => format!("item<{}>", mt(&**i)),
        m::Stmt::Expr(e, semi) => format!("{}{}", mexpr(e), if semi.is_some() { ";" } else { "" }),
        m::Stmt::Macro(mac) => format!(
            "{}mac<{}>{}",
            mattrs(&mac.attrs),
            mt(&mac.mac),
            if mac.semi.is_some() { ";" } else { "" }
        ),
        _ => "UNKNOWN".into(),
    }
}

pub(super) fn mpat(p: &m::Pattern) -> String {
    use m::Pattern as P;
    match p {
        P::Wild(_) => "_".into(),
        P::Rest(_) => "..".into(),
        P::Ident(i) => format!(
            "id({}{}{}{})",
            if i.by_ref.is_some() { "ref " } else { "" },
            if i.mutability.is_some() { "mut " } else { "" },
            mt(&i.ident),
            i.subpat.as_ref().map(|(_, p)| format!("@{}", mpat(p))).unwrap_or_default()
        ),
        P::Path(p) => format!("path<{}>", mt(p)),
        P::Tuple(t) => format!("tup({})", t.elems.iter().map(mpat).collect::<Vec<_>>().join(",")),
        P::TupleStruct(t) => format!(
            "ts<{}{}>({})",
            t.qself.as_ref().map(|_| "Q:").unwrap_or(""),
            {
                let all = mt(t);
                let elems = mt(&t.elems);
                all.strip_suffix(&elems).unwrap_or(&all).to_string()
            },
            t.elems.iter().map(mpat).collect::<Vec<_>>().join(",")
        ),
        P::Struct(s) => format!(
            "st<{}>{{{}{}}}",
            mt(&s.path),
            s.body
                .fields
                .iter()
                .map(|f| format!("{}{}:{}", mattrs(&f.attrs), mt(&f.member), mpat(&f.pat)))
                .collect::<Vec<_>>()
                .join(","),
            if s.body.dotdot.is_some() { ",.." } else { "" }
        ),
        P::Slice(s) => format!("sl[{}]", s.elems.iter().map(mpat).collect::<Vec<_>>().join(",")),
        P::Reference(r) => format!("(&{}{})", if r.mutability.is_some() { "mut " } else { "" }, mpat(&r.pat)),
        P::Or(o) => format!(
            "or({}{})",
            if o.leading_vert.is_some() { "|" } else { "" },
            o.cases.iter().map(mpat).collect::<Vec<_>>().join("|")
        ),
        P::Lit(l) => format!("lit<{}>", mt(l)),
        P::Range(r) => format!(
            "range({} {} {})",
            r.start.as_ref().map(mexpr).unwrap_or_default(),
            mt(&r.limits),
            r.end.as_ref().map(mexpr).unwrap_or_default()
        ),
        P::Macro(mac) => format!("mac<{}>", mt(mac)),
        P::Type(t) => format!("({}:{})", mpat(&t.pat), mt(&*t.ty)),
        P::Group(g) => mpat(&g.pat),
        P::Paren(p) => format!("paren({})", mpat(&p.content)),
        P::Box(b) => format!("box {}", mpat(&b.pattern)),
        P::Const(c) => format!("const{}", mblock(&c.block)),
        _ => "UNKNOWN".into(),
    }
}

pub(super) fn mexpr(e: &m::Expr) -> String {
    use m::Expr as E;
    match e {
        E::Array(x) => format!(
            "{}arr[{}]",
            mattrs(&x.attrs),
            x.elems.iter().map(mexpr).collect::<Vec<_>>().join(",")
        ),
        E::Assign(x) => format!(
            "{}{}",
            mattrs(&x.attrs),
            grp(format!("{} = {}", mexpr(&x.left), mexpr(&x.right)))
        ),
        E::Async(x) => format!(
            "{}async{}{}",
            mattrs(&x.attrs),
            if x.move_keyword.is_some() { " move" } else { "" },
            mblock(&x.block)
        ),
        E::Await(x) => format!("{}({}.await)", mattrs(&x.attrs), mexpr(&x.base)),
        E::Binary(x) => format!(
            "{}{}",
            mattrs(&x.attrs),
            grp(format!("{} {} {}", mexpr(&x.left), mt(&x.op), mexpr(&x.right)))
        ),
        E::Block(x) => format!(
            "{}{}{}",
            mattrs(&x.attrs),
            x.label.as_ref().map(mt).unwrap_or_default(),
            mblock(&x.block)
        ),
        E::Break(x) => format!(
            "{}(break {} {})",
            mattrs(&x.attrs),
            x.label.as_ref().map(mt).unwrap_or_default(),
            mopt(&x.expr)
        ),
        E::Call(x) => format!(
            "{}call({}; {})",
            mattrs(&x.attrs),
            mexpr(&x.func),
            x.args.iter().map(mexpr).collect::<Vec<_>>().join(",")
        ),
        E::Cast(x) => format!("{}({} as {})", mattrs(&x.attrs), mexpr(&x.expr), mt(&*x.ty)),
        E::Closure(x) => format!(
            "{}closure[{}{}{}{}{}|{}|{}]{}",
            mattrs(&x.attrs),
            x.lifetimes.as_ref().map(mt).unwrap_or_default(),
            if x.constness.is_some() { "const " } else { "" },
            if x.movability.is_some() { "static " } else { "" },
            if x.asyncness.is_some() { "async " } else { "" },
            if x.capture.is_some() { "move " } else { "" },
            x.inputs
                .iter()
                .map(|p| match p {
                    m::ClosureParam::Typed { pat, ty, .. } => format!("({}:{})", mpat(pat), mt(&**ty)),
                    m::ClosureParam::Inferred { pat } => mpat(pat),
                })
                .collect::<Vec<_>>()
                .join(","),
            mt(&x.output),
            mexpr(&x.body)
        ),
        E::Const(x) => format!("{}const{}", mattrs(&x.attrs), mblock(&x.block)),
        E::Continue(x) => format!(
            "{}(continue {})",
            mattrs(&x.attrs),
            x.label.as_ref().map(mt).unwrap_or_default()
        ),
        E::Field(x) => format!("{}({}.{})", mattrs(&x.attrs), mexpr(&x.base), mt(&x.member)),
        E::ForLoop(x) => format!(
            "{}{}for {} in {} {}",
            mattrs(&x.attrs),
            x.label.as_ref().map(mt).unwrap_or_default(),
            mpat(&x.pat),
            mexpr(&x.expr),
            mblock(&x.body)
        ),
        E::Group(x) => mexpr(&x.expr),
        E::If(x) => format!(
            "{}if {} {}{}",
            mattrs(&x.attrs),
            mexpr(&x.cond),
            mblock(&x.then_branch),
            x.else_branch
                .as_ref()
                .map(|e| format!(" else {}", mexpr(e)))
                .unwrap_or_default()
        ),
        E::Index(x) => format!("{}idx({}; {})", mattrs(&x.attrs), mexpr(&x.base), mexpr(&x.index)),
        E::Infer(x) => format!("{}_", mattrs(&x.attrs)),
        E::Let(x) => format!(
            "{}{}",
            mattrs(&x.attrs),
            grp(format!("let {} = {}", mpat(&x.pat), mexpr(&x.expr)))
        ),
        E::Lit(x) => format!("{}{}", mattrs(&x.attrs), mt(&x.lit)),
        E::Loop(x) => format!(
            "{}{}loop{}",
            mattrs(&x.attrs),
            x.label.as_ref().map(mt).unwrap_or_default(),
            mblock(&x.body)
        ),
        E::Macro(x) => format!("{}mac<{}>", mattrs(&x.attrs), mt(&x.mac)),
        E::Match(x) => format!(
            "{}match {} {{{}}}",
            mattrs(&x.attrs),
            mexpr(&x.expr),
            x.arms
                .iter()
                .map(|a| format!(
                    "{}{}{} => {}",
                    mattrs(&a.attrs),
                    mpat(&a.pat),
                    a.guard.as_ref().map(|g| format!(" if {}", mexpr(g))).unwrap_or_default(),
                    mexpr(&a.body)
                ))
                .collect::<Vec<_>>()
                .join(", ")
        ),
        E::MethodCall(x) => format!(
            "{}mc({}.{}{}; {})",
            mattrs(&x.attrs),
            mexpr(&x.receiver),
            mt(&x.method),
            x.turbofish.as_ref().map(mt).unwrap_or_default(),
            x.args.iter().map(mexpr).collect::<Vec<_>>().join(",")
        ),
        E::Paren(x) => format!("{}paren({})", mattrs(&x.attrs), mexpr(&x.content)),
        E::Path(x) => format!("{}path<{}>", mattrs(&x.attrs), mt(x).trim_start_matches(&mt(&x.attrs))),
        E::Range(x) => format!(
            "{}{}",
            mattrs(&x.attrs),
            grp(format!("{} {} {}", mopt(&x.start), mt(&x.limits), mopt(&x.end)))
        ),
        E::RawAddr(x) => format!("{}(&raw {} {})", mattrs(&x.attrs), mt(&x.mutability), mexpr(&x.expr)),
        E::Reference(x) => format!(
            "{}(&{}{})",
            mattrs(&x.attrs),
            if x.mutability.is_some() { "mut " } else { "" },
            mexpr(&x.expr)
        ),
        E::Repeat(x) => format!(
            "{}rep[{}; {}]",
            mattrs(&x.attrs),
            mexpr(&x.content.elem),
            mexpr(&x.content.len)
        ),
        E::Return(x) => format!("{}(return {})", mattrs(&x.attrs), mopt(&x.expr)),
        E::Struct(x) => format!(
            "{}struct<{}{}>{{{}{}}}",
            mattrs(&x.attrs),
            x.qself.as_ref().map(|_| "Q:").unwrap_or(""),
            mt(&x.path),
            x.body
                .fields
                .iter()
                .map(|f| format!(
                    "{}{}{}{}",
                    mattrs(&f.attrs),
                    mt(&f.member),
                    if f.colon_punct.is_some() { ":" } else { "~" },
                    mexpr(&f.expr)
                ))
                .collect::<Vec<_>>()
                .join(","),
            x.body
                .rest
                .as_ref()
                .map(|(_, e)| format!(" ..{}", mexpr(e)))
                .unwrap_or_default()
        ),
        E::Try(x) => format!("{}({}?)", mattrs(&x.attrs), mexpr(&x.expr)),
        E::TryBlock(x) => format!("{}try{}", mattrs(&x.attrs), mblock(&x.block)),
        E::Tuple(x) => format!(
            "{}tup({})",
            mattrs(&x.attrs),
            x.elems.iter().map(mexpr).collect::<Vec<_>>().join(",")
        ),
        // syn reads `-1` as a negative literal in patterns and generic arguments
        E::Unary(x) if matches!(x.op, m::UnOp::Neg(_)) && x.expr.is_lit() && x.attrs.is_empty() => format!("-{}", mexpr(&x.expr)),
        E::Unary(x) => format!("{}({} {})", mattrs(&x.attrs), mt(&x.op), mexpr(&x.expr)),
        E::Unsafe(x) => format!("{}unsafe{}", mattrs(&x.attrs), mblock(&x.block)),
        E::Verbatim(x) => format!("V<{}>", strip(&x.to_string())),
        E::While(x) => format!(
            "{}{}while {} {}",
            mattrs(&x.attrs),
            x.label.as_ref().map(mt).unwrap_or_default(),
            mexpr(&x.cond),
            mblock(&x.body)
        ),
        E::Yield(x) => format!("{}(yield {})", mattrs(&x.attrs), mopt(&x.expr)),
        _ => "UNKNOWN".into(),
    }
}

pub(super) fn sblock(b: &syn::Block) -> String {
    let mut s = String::from("{");
    for stmt in &b.stmts {
        s.push_str(&sstmt(stmt));
        s.push(' ');
    }
    s.push('}');
    s
}

fn sstmt(stmt: &syn::Stmt) -> String {
    match stmt {
        syn::Stmt::Local(l) => {
            let (pat, ty) = match &l.pat {
                syn::Pat::Type(t) => (spat(&t.pat), format!(":{}", st(&t.ty))),
                p => (spat(p), String::new()),
            };
            let mut s = format!("{}let {}{}", sattrs(&l.attrs), pat, ty);
            if let Some(init) = &l.init {
                s.push_str(&format!(" = {}", sexpr(&init.expr)));
                if let Some((_, e)) = &init.diverge {
                    s.push_str(&format!(" else {}", sexpr(e)));
                }
            }
            s.push(';');
            s
        }
        syn::Stmt::Item(i) => format!("item<{}>", st(i)),
        syn::Stmt::Expr(e, semi) => format!("{}{}", sexpr(e), if semi.is_some() { ";" } else { "" }),
        syn::Stmt::Macro(mac) => format!(
            "{}mac<{}>{}",
            sattrs(&mac.attrs),
            st(&mac.mac),
            if mac.semi_token.is_some() { ";" } else { "" }
        ),
    }
}

pub(super) fn spat(p: &syn::Pat) -> String {
    use syn::Pat as P;
    match p {
        P::Wild(_) => "_".into(),
        P::Rest(_) => "..".into(),
        P::Ident(i) => format!(
            "id({}{}{}{})",
            if i.by_ref.is_some() { "ref " } else { "" },
            if i.mutability.is_some() { "mut " } else { "" },
            st(&i.ident),
            i.subpat.as_ref().map(|(_, p)| format!("@{}", spat(p))).unwrap_or_default()
        ),
        P::Path(p) => format!("path<{}>", st(p)),
        P::Tuple(t) => format!("tup({})", t.elems.iter().map(spat).collect::<Vec<_>>().join(",")),
        P::TupleStruct(t) => format!(
            "ts<{}{}>({})",
            t.qself.as_ref().map(|_| "Q:").unwrap_or(""),
            {
                let mut all = st(t);
                let elems = format!("({})", strip(&t.elems.to_token_stream().to_string()));
                if all.ends_with(&elems) {
                    all.truncate(all.len() - elems.len());
                }
                all
            },
            t.elems.iter().map(spat).collect::<Vec<_>>().join(",")
        ),
        P::Struct(s) => format!(
            "st<{}>{{{}{}}}",
            st(&s.path),
            s.fields
                .iter()
                .map(|f| format!("{}{}:{}", sattrs(&f.attrs), st(&f.member), spat(&f.pat)))
                .collect::<Vec<_>>()
                .join(","),
            if s.rest.is_some() { ",.." } else { "" }
        ),
        P::Slice(s) => format!("sl[{}]", s.elems.iter().map(spat).collect::<Vec<_>>().join(",")),
        P::Reference(r) => format!("(&{}{})", if r.mutability.is_some() { "mut " } else { "" }, spat(&r.pat)),
        P::Or(o) => format!(
            "or({}{})",
            if o.leading_vert.is_some() { "|" } else { "" },
            o.cases.iter().map(spat).collect::<Vec<_>>().join("|")
        ),
        P::Lit(l) => format!("lit<{}>", st(l)),
        P::Range(r) => format!("range({} {} {})", sopt(&r.start), st(&r.limits), sopt(&r.end)),
        P::Macro(mac) => format!("mac<{}>", st(mac)),
        P::Type(t) => format!("({}:{})", spat(&t.pat), st(&t.ty)),
        P::Paren(p) => format!("paren({})", spat(&p.pat)),
        P::Guard(g) => format!("{} if {}", spat(&g.pat), sexpr(&g.guard)),
        P::Const(c) => format!("const{}", sblock(&c.block)),
        P::Verbatim(_) => SYN_VERBATIM.into(),
        _ => "UNKNOWN".into(),
    }
}

pub(super) fn sexpr(e: &syn::Expr) -> String {
    use syn::Expr as E;
    match e {
        E::Array(x) => format!(
            "{}arr[{}]",
            sattrs(&x.attrs),
            x.elems.iter().map(sexpr).collect::<Vec<_>>().join(",")
        ),
        E::Assign(x) => format!(
            "{}{}",
            sattrs(&x.attrs),
            grp(format!("{} = {}", sexpr(&x.left), sexpr(&x.right)))
        ),
        E::Async(x) => format!(
            "{}async{}{}",
            sattrs(&x.attrs),
            if x.capture.is_some() { " move" } else { "" },
            sblock(&x.block)
        ),
        E::Await(x) => format!("{}({}.await)", sattrs(&x.attrs), sexpr(&x.base)),
        E::Binary(x) => format!(
            "{}{}",
            sattrs(&x.attrs),
            grp(format!("{} {} {}", sexpr(&x.left), st(&x.op), sexpr(&x.right)))
        ),
        E::Block(x) => format!(
            "{}{}{}",
            sattrs(&x.attrs),
            x.label.as_ref().map(st).unwrap_or_default(),
            sblock(&x.block)
        ),
        E::Break(x) => format!(
            "{}(break {} {})",
            sattrs(&x.attrs),
            x.label.as_ref().map(st).unwrap_or_default(),
            sopt(&x.expr)
        ),
        E::Call(x) => format!(
            "{}call({}; {})",
            sattrs(&x.attrs),
            sexpr(&x.func),
            x.args.iter().map(sexpr).collect::<Vec<_>>().join(",")
        ),
        E::Cast(x) => format!("{}({} as {})", sattrs(&x.attrs), sexpr(&x.expr), st(&x.ty)),
        E::Closure(x) => format!(
            "{}closure[{}{}{}{}{}|{}|{}]{}",
            sattrs(&x.attrs),
            x.lifetimes.as_ref().map(st).unwrap_or_default(),
            if x.constness.is_some() { "const " } else { "" },
            "",
            if x.asyncness.is_some() { "async " } else { "" },
            if x.capture.is_some() { "move " } else { "" },
            x.inputs.iter().map(spat).collect::<Vec<_>>().join(","),
            st(&x.output),
            sexpr(&x.body)
        ),
        E::Const(x) => format!("{}const{}", sattrs(&x.attrs), sblock(&x.block)),
        E::Continue(x) => format!(
            "{}(continue {})",
            sattrs(&x.attrs),
            x.label.as_ref().map(st).unwrap_or_default()
        ),
        E::Field(x) => format!("{}({}.{})", sattrs(&x.attrs), sexpr(&x.base), st(&x.member)),
        E::ForLoop(x) => format!(
            "{}{}for {} in {} {}",
            sattrs(&x.attrs),
            x.label.as_ref().map(st).unwrap_or_default(),
            spat(&x.pat),
            sexpr(&x.expr),
            sblock(&x.body)
        ),
        E::Group(x) => sexpr(&x.expr),
        E::If(x) => format!(
            "{}if {} {}{}",
            sattrs(&x.attrs),
            sexpr(&x.cond),
            sblock(&x.then_branch),
            x.else_branch
                .as_ref()
                .map(|(_, e)| format!(" else {}", sexpr(e)))
                .unwrap_or_default()
        ),
        E::Index(x) => format!("{}idx({}; {})", sattrs(&x.attrs), sexpr(&x.expr), sexpr(&x.index)),
        E::Infer(x) => format!("{}_", sattrs(&x.attrs)),
        E::Let(x) => format!(
            "{}{}",
            sattrs(&x.attrs),
            grp(format!("let {} = {}", spat(&x.pat), sexpr(&x.expr)))
        ),
        E::Lit(x) => format!("{}{}", sattrs(&x.attrs), st(&x.lit)),
        E::Loop(x) => format!(
            "{}{}loop{}",
            sattrs(&x.attrs),
            x.label.as_ref().map(st).unwrap_or_default(),
            sblock(&x.body)
        ),
        E::Macro(x) => format!("{}mac<{}>", sattrs(&x.attrs), st(&x.mac)),
        E::Match(x) => format!(
            "{}match {} {{{}}}",
            sattrs(&x.attrs),
            sexpr(&x.expr),
            x.arms
                .iter()
                .map(|a| format!("{}{} => {}", sattrs(&a.attrs), spat(&a.pat), sexpr(&a.body)))
                .collect::<Vec<_>>()
                .join(", ")
        ),
        E::MethodCall(x) => format!(
            "{}mc({}.{}{}; {})",
            sattrs(&x.attrs),
            sexpr(&x.receiver),
            st(&x.method),
            x.turbofish.as_ref().map(st).unwrap_or_default(),
            x.args.iter().map(sexpr).collect::<Vec<_>>().join(",")
        ),
        E::Paren(x) => format!("{}paren({})", sattrs(&x.attrs), sexpr(&x.expr)),
        E::Path(x) => {
            let mut bare = x.clone();
            bare.attrs.clear();
            format!("{}path<{}>", sattrs(&x.attrs), st(&bare))
        }
        E::Range(x) => format!(
            "{}{}",
            sattrs(&x.attrs),
            grp(format!("{} {} {}", sopt(&x.start), st(&x.limits), sopt(&x.end)))
        ),
        E::RawAddr(x) => format!("{}(&raw {} {})", sattrs(&x.attrs), st(&x.mutability), sexpr(&x.expr)),
        E::Reference(x) => format!(
            "{}(&{}{})",
            sattrs(&x.attrs),
            if x.mutability.is_some() { "mut " } else { "" },
            sexpr(&x.expr)
        ),
        E::Repeat(x) => format!("{}rep[{}; {}]", sattrs(&x.attrs), sexpr(&x.expr), sexpr(&x.len)),
        E::Return(x) => format!("{}(return {})", sattrs(&x.attrs), sopt(&x.expr)),
        E::Struct(x) => format!(
            "{}struct<{}{}>{{{}{}}}",
            sattrs(&x.attrs),
            x.qself.as_ref().map(|_| "Q:").unwrap_or(""),
            st(&x.path),
            x.fields
                .iter()
                .map(|f| format!(
                    "{}{}{}{}",
                    sattrs(&f.attrs),
                    st(&f.member),
                    if f.colon_token.is_some() { ":" } else { "~" },
                    sexpr(&f.expr)
                ))
                .collect::<Vec<_>>()
                .join(","),
            x.rest.as_ref().map(|e| format!(" ..{}", sexpr(e))).unwrap_or_default()
        ),
        E::Try(x) => format!("{}({}?)", sattrs(&x.attrs), sexpr(&x.expr)),
        E::TryBlock(x) => format!("{}try{}", sattrs(&x.attrs), sblock(&x.block)),
        E::Tuple(x) => format!(
            "{}tup({})",
            sattrs(&x.attrs),
            x.elems.iter().map(sexpr).collect::<Vec<_>>().join(",")
        ),
        E::Unary(x) if matches!(x.op, syn::UnOp::Neg(_)) && matches!(*x.expr, syn::Expr::Lit(_)) && x.attrs.is_empty() => {
            format!("-{}", sexpr(&x.expr))
        }
        E::Unary(x) => format!("{}({} {})", sattrs(&x.attrs), st(&x.op), sexpr(&x.expr)),
        E::Unsafe(x) => format!("{}unsafe{}", sattrs(&x.attrs), sblock(&x.block)),
        E::Verbatim(_) => SYN_VERBATIM.into(),
        E::While(x) => format!(
            "{}{}while {} {}",
            sattrs(&x.attrs),
            x.label.as_ref().map(st).unwrap_or_default(),
            sexpr(&x.cond),
            sblock(&x.body)
        ),
        E::Yield(x) => format!("{}(yield {})", sattrs(&x.attrs), sopt(&x.expr)),
        _ => "UNKNOWN".into(),
    }
}

fn mbounds(b: &m::List<m::TypeBound, moxy::Token![+]>) -> String {
    b.iter()
        .map(|b| match b {
            m::TypeBound::Trait(t) => format!(
                "bound<{}{}{}{}>",
                if t.polarity.is_some() { "!" } else { "" },
                if t.modifier.is_some() { "?" } else { "" },
                t.lifetimes.as_ref().map(mt).unwrap_or_default(),
                mpath(&t.path)
            ),
            m::TypeBound::Lifetime(l) => mt(l),
            m::TypeBound::Use(u) => mt(u),
        })
        .collect::<Vec<_>>()
        .join(" + ")
}

fn mret(r: &m::ReturnType) -> String {
    match r {
        m::ReturnType::Default => String::new(),
        m::ReturnType::Type(_, t) => format!(" -> {}", mtype(t)),
    }
}

fn margs(a: &m::PathArguments) -> String {
    match a {
        m::PathArguments::None => String::new(),
        m::PathArguments::AngleBracketed(a) => format!(
            "<{}>",
            a.args
                .iter()
                .map(|g| match g {
                    m::GenericArgument::Lifetime(l) => mt(l),
                    m::GenericArgument::Type(t) => mtype(t),
                    m::GenericArgument::Const(e) => format!("const:{}", mexpr(e)),
                    m::GenericArgument::AssocType(a) => format!(
                        "{}{}={}",
                        mt(&a.ident),
                        a.generics.as_ref().map(mt).unwrap_or_default(),
                        mtype(&a.ty)
                    ),
                    m::GenericArgument::AssocConst(a) => format!("{}=const:{}", mt(&a.ident), mexpr(&a.expr)),
                    m::GenericArgument::Constraint(c) => format!("{}:{}", mt(&c.ident), mbounds(&c.bounds)),
                })
                .collect::<Vec<_>>()
                .join(",")
        ),
        m::PathArguments::Parenthesized(p) => format!(
            "({}){}",
            p.params.iter().map(mtype).collect::<Vec<_>>().join(","),
            mret(&p.output)
        ),
    }
}

fn mpath(p: &m::Path) -> String {
    // Path fields are private: walk segments through the public iterator when available
    p.iter()
        .map(|s| format!("{}{}", mt(&s.ident), margs(&s.args)))
        .collect::<Vec<_>>()
        .join("::")
}

pub(super) fn mtype(t: &m::Type) -> String {
    use m::Type as T;
    match t {
        T::Never(_) => "!".into(),
        T::Infer(_) => "_".into(),
        T::Path(p) => format!(
            "{}{}",
            p.qself
                .as_ref()
                .map(|q| format!("<{} as@{}>", mtype(&q.ty), q.position))
                .unwrap_or_default(),
            mpath(&p.path)
        ),
        T::Tuple(x) => format!("tup({})", x.elems.iter().map(mtype).collect::<Vec<_>>().join(",")),
        T::Array(x) => format!("[{}; {}]", mtype(&x.content.elem), mexpr(&x.content.len)),
        T::Slice(x) => format!("[{}]", mtype(&x.elem)),
        T::Reference(x) => format!(
            "(&{}{}{})",
            x.lifetime.as_ref().map(|l| mt(l) + " ").unwrap_or_default(),
            if x.mutability.is_some() { "mut " } else { "" },
            mtype(&x.elem)
        ),
        T::Pointer(x) => format!("(*{} {})", mt(&x.mutability), mtype(&x.elem)),
        T::BareFn(x) => format!(
            "fn[{}{}{}]({}{}){}",
            x.lifetimes.as_ref().map(mt).unwrap_or_default(),
            if x.unsafety.is_some() { "unsafe " } else { "" },
            x.abi.as_ref().map(mt).unwrap_or_default(),
            x.params.inputs.iter().map(|a| mtype(&a.ty)).collect::<Vec<_>>().join(","),
            if x.params.variadic.is_some() { ",..." } else { "" },
            mret(&x.output)
        ),
        T::ImplTrait(x) => format!("impl({})", mbounds(&x.bounds)),
        T::TraitObject(x) => format!("dyn({})", mbounds(&x.bounds)),
        T::Paren(x) => format!("paren({})", mtype(&x.content)),
        T::Group(x) => mtype(&x.elem),
        T::Macro(x) => format!("mac<{}>", mt(&x.mac)),
        _ => "UNKNOWN".into(),
    }
}

fn sbounds(b: &syn::punctuated::Punctuated<syn::TypeParamBound, syn::Token![+]>) -> String {
    b.iter()
        .map(|b| match b {
            syn::TypeParamBound::Trait(t) => format!(
                "bound<{}{}{}>",
                if t.maybe.is_some() { "?" } else { "" },
                t.lifetimes.as_ref().map(st).unwrap_or_default(),
                spath(&t.path)
            ),
            syn::TypeParamBound::Lifetime(l) => st(l),
            other => st(other),
        })
        .collect::<Vec<_>>()
        .join(" + ")
}

fn sret(r: &syn::ReturnType) -> String {
    match r {
        syn::ReturnType::Default => String::new(),
        syn::ReturnType::Type(_, t) => format!(" -> {}", stype(t)),
    }
}

fn spath(p: &syn::Path) -> String {
    p.segments
        .iter()
        .map(|s| {
            let args = match &s.arguments {
                syn::PathArguments::None => String::new(),
                syn::PathArguments::AngleBracketed(a) => format!(
                    "<{}>",
                    a.args
                        .iter()
                        .map(|g| match g {
                            syn::GenericArgument::Lifetime(l) => st(l),
                            syn::GenericArgument::Type(t) => stype(t),
                            syn::GenericArgument::Const(e) => format!("const:{}", sexpr(e)),
                            syn::GenericArgument::AssocType(a) => format!(
                                "{}{}={}",
                                st(&a.ident),
                                a.generics.as_ref().map(st).unwrap_or_default(),
                                stype(&a.ty)
                            ),
                            syn::GenericArgument::AssocConst(a) => format!("{}=const:{}", st(&a.ident), sexpr(&a.value)),
                            syn::GenericArgument::Constraint(c) => format!("{}:{}", st(&c.ident), sbounds(&c.bounds)),
                            other => st(other),
                        })
                        .collect::<Vec<_>>()
                        .join(",")
                ),
                syn::PathArguments::Parenthesized(p) => format!(
                    "({}){}",
                    p.inputs.iter().map(|a| stype(&a.ty)).collect::<Vec<_>>().join(","),
                    sret(&p.output)
                ),
            };
            format!("{}{}", st(&s.ident), args)
        })
        .collect::<Vec<_>>()
        .join("::")
}

pub(super) fn stype(t: &syn::Type) -> String {
    use syn::Type as T;
    match t {
        T::Never(_) => "!".into(),
        T::Infer(_) => "_".into(),
        T::Path(p) => format!(
            "{}{}",
            p.qself
                .as_ref()
                .map(|q| format!("<{} as@{}>", stype(&q.ty), q.position))
                .unwrap_or_default(),
            spath(&p.path)
        ),
        T::Tuple(x) => format!("tup({})", x.elems.iter().map(stype).collect::<Vec<_>>().join(",")),
        T::Array(x) => format!("[{}; {}]", stype(&x.elem), sexpr(&x.len)),
        T::Slice(x) => format!("[{}]", stype(&x.elem)),
        T::Reference(x) => format!(
            "(&{}{}{})",
            x.lifetime.as_ref().map(|l| st(l) + " ").unwrap_or_default(),
            if x.mutability.is_some() { "mut " } else { "" },
            stype(&x.elem)
        ),
        T::Ptr(x) => format!(
            "(*{} {})",
            if matches!(x.mutability, syn::PointerMutability::Mut(_)) {
                "mut"
            } else {
                "const"
            },
            stype(&x.elem)
        ),
        T::FnPtr(x) => format!(
            "fn[{}{}{}]({}{}){}",
            x.lifetimes.as_ref().map(st).unwrap_or_default(),
            if x.unsafety.is_some() { "unsafe " } else { "" },
            x.abi.as_ref().map(st).unwrap_or_default(),
            x.inputs.iter().map(|a| stype(&a.ty)).collect::<Vec<_>>().join(","),
            if x.variadic.is_some() { ",..." } else { "" },
            sret(&x.output)
        ),
        T::ImplTrait(x) => format!("impl({})", sbounds(&x.bounds)),
        T::TraitObject(x) => format!("dyn({})", sbounds(&x.bounds)),
        T::Paren(x) => format!("paren({})", stype(&x.elem)),
        T::Group(x) => stype(&x.elem),
        T::Macro(x) => format!("mac<{}>", st(&x.mac)),
        T::Verbatim(_) => SYN_VERBATIM.into(),
        _ => "UNKNOWN".into(),
    }
}

fn mgenerics(g: &m::Generics) -> String {
    let params = g
        .params
        .iter()
        .map(|p| match p {
            m::GenericParam::Lifetime(l) => mt(&l.lifetime),
            m::GenericParam::Type(t) => format!(
                "{}{}{}",
                mt(&t.ident),
                if t.bounds.is_empty() {
                    String::new()
                } else {
                    format!(":{}", mbounds(&t.bounds))
                },
                t.default.as_ref().map(|d| format!("={}", mtype(d))).unwrap_or_default()
            ),
            m::GenericParam::Const(c) => format!(
                "const {}:{}{}",
                mt(&c.ident),
                mtype(&c.ty),
                c.default.as_ref().map(|d| format!("={}", mexpr(d))).unwrap_or_default()
            ),
        })
        .collect::<Vec<_>>()
        .join(",");
    let wheres = g
        .where_clause
        .as_ref()
        .map(|w| format!(" where[{}]", mt(&w.predicates)))
        .unwrap_or_default();
    if params.is_empty() {
        wheres
    } else {
        format!("<{params}>{wheres}")
    }
}

fn sgenerics(g: &syn::Generics) -> String {
    let params = g
        .params
        .iter()
        .map(|p| match p {
            syn::GenericParam::Lifetime(l) => st(&l.lifetime),
            syn::GenericParam::Type(t) => format!(
                "{}{}{}",
                st(&t.ident),
                if t.bounds.is_empty() {
                    String::new()
                } else {
                    format!(":{}", sbounds(&t.bounds))
                },
                t.default.as_ref().map(|(_, d)| format!("={}", stype(d))).unwrap_or_default()
            ),
            syn::GenericParam::Const(c) => format!(
                "const {}:{}{}",
                st(&c.ident),
                stype(&c.ty),
                c.default.as_ref().map(|(_, d)| format!("={}", sexpr(d))).unwrap_or_default()
            ),
        })
        .collect::<Vec<_>>()
        .join(",");
    let wheres = g
        .where_clause
        .as_ref()
        .map(|w| format!(" where[{}]", st(&w.predicates)))
        .unwrap_or_default();
    if params.is_empty() {
        wheres
    } else {
        format!("<{params}>{wheres}")
    }
}

fn mfields(f: &m::Fields) -> String {
    let field = |f: &m::Field| {
        format!(
            "{}{}{}{}",
            mattrs(&f.attrs),
            mt(&f.vis),
            f.ident.as_ref().map(|i| format!("{}:", mt(i))).unwrap_or_default(),
            mtype(&f.ty)
        )
    };
    match f {
        m::Fields::Named(n) => format!("{{{}}}", n.fields.inner.iter().map(field).collect::<Vec<_>>().join(",")),
        m::Fields::Unnamed(u) => format!("({})", u.fields.inner.iter().map(field).collect::<Vec<_>>().join(",")),
        m::Fields::Unit => String::new(),
    }
}

fn sfields(f: &syn::Fields) -> String {
    let field = |f: &syn::Field| {
        format!(
            "{}{}{}{}",
            sattrs(&f.attrs),
            st(&f.vis),
            f.ident.as_ref().map(|i| format!("{}:", st(i))).unwrap_or_default(),
            stype(&f.ty)
        )
    };
    match f {
        syn::Fields::Named(n) => format!("{{{}}}", n.named.iter().map(field).collect::<Vec<_>>().join(",")),
        syn::Fields::Unnamed(u) => format!("({})", u.unnamed.iter().map(field).collect::<Vec<_>>().join(",")),
        syn::Fields::Unit => String::new(),
    }
}

fn msig(s: &m::Signature) -> String {
    let inputs = s
        .params
        .inner
        .inputs
        .iter()
        .map(|p| match p {
            m::FnParam::Receiver(r) => format!("self<{}>", mt(r)),
            m::FnParam::Typed(t) => format!("{}:{}", mpat(&t.pat), mtype(&t.ty)),
            m::FnParam::Type(t) => format!("type<{}>", mtype(t)),
        })
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "{}{}{}{}fn {}{}({}{}){}",
        mt(&s.constness),
        mt(&s.asyncness),
        mt(&s.safety),
        mt(&s.abi),
        mt(&s.ident),
        mgenerics(&s.generics),
        inputs,
        if s.params.inner.variadic.is_some() { ",..." } else { "" },
        mret(&s.output)
    )
}

fn ssig(s: &syn::Signature) -> String {
    let inputs = s
        .inputs
        .iter()
        .map(|p| match p {
            syn::FnArg::Receiver(r) => format!("self<{}>", st(r)),
            syn::FnArg::Typed(t) => format!("{}:{}", spat(&t.pat), stype(&t.ty)),
        })
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "{}{}{}{}fn {}{}({}{}){}",
        st(&s.constness),
        st(&s.asyncness),
        st(&s.safety),
        st(&s.abi),
        st(&s.ident),
        sgenerics(&s.generics),
        inputs,
        if s.variadic.is_some() { ",..." } else { "" },
        sret(&s.output)
    )
}

fn mbody(b: &Option<m::StmtBlock>) -> String {
    b.as_ref().map(mblock).unwrap_or_else(|| ";".into())
}

pub(super) fn mitem(i: &m::Item) -> String {
    use m::Item as I;
    match i {
        I::Fn(f) => format!("{}{}{} {}", mattrs(&f.attrs), mt(&f.vis), msig(&f.sig), mbody(&f.body)),
        I::Struct(s) => format!(
            "{}{}struct {}{}{}",
            mattrs(&s.attrs),
            mt(&s.vis),
            mt(&s.ident),
            mgenerics(&s.generics),
            mfields(&s.fields)
        ),
        I::Enum(e) => format!(
            "{}{}enum {}{}{{{}}}",
            mattrs(&e.attrs),
            mt(&e.vis),
            mt(&e.ident),
            mgenerics(&e.generics),
            e.variants
                .inner
                .iter()
                .map(|v| format!(
                    "{}{}{}{}",
                    mattrs(&v.attrs),
                    mt(&v.ident),
                    mfields(&v.fields),
                    v.discriminant.as_ref().map(|d| format!("={}", mexpr(d))).unwrap_or_default()
                ))
                .collect::<Vec<_>>()
                .join(",")
        ),
        I::Union(u) => format!(
            "{}{}union {}{}{}",
            mattrs(&u.attrs),
            mt(&u.vis),
            mt(&u.ident),
            mgenerics(&u.generics),
            mfields(&m::Fields::Named(u.fields.clone()))
        ),
        I::Impl(x) => format!(
            "{}impl{} {}{} {{{}}}",
            mattrs(&x.attrs),
            mgenerics(&x.generics),
            x.trait_ref
                .as_ref()
                .map(|t| format!("{} for ", mt(&t.path)))
                .unwrap_or_default(),
            mtype(&x.self_ty),
            x.items
                .inner
                .iter()
                .map(|it| match it {
                    m::ImplItem::Fn(f) => format!("{}{}{} {}", mattrs(&f.attrs), mt(&f.vis), msig(&f.sig), mblock(&f.body)),
                    other => format!("tokens<{}>", mt(other)),
                })
                .collect::<Vec<_>>()
                .join(" ")
        ),
        I::Trait(t) => format!(
            "{}{}trait {}{}{} {{{}}}",
            mattrs(&t.attrs),
            mt(&t.vis),
            mt(&t.ident),
            mgenerics(&t.generics),
            if t.supertraits.is_empty() {
                String::new()
            } else {
                format!(":{}", mbounds(&t.supertraits))
            },
            t.items
                .inner
                .iter()
                .map(|it| match it {
                    m::TraitItem::Fn(f) => format!("{}{} {}", mattrs(&f.attrs), msig(&f.sig), mbody(&f.body)),
                    other => format!("tokens<{}>", mt(other)),
                })
                .collect::<Vec<_>>()
                .join(" ")
        ),
        I::Mod(x) => format!(
            "{}{}mod {}{}",
            mattrs(&x.attrs),
            mt(&x.vis),
            mt(&x.ident),
            x.content
                .as_ref()
                .map(|c| format!("{{{}}}", c.inner.iter().map(mitem).collect::<Vec<_>>().join(" ")))
                .unwrap_or_else(|| ";".into())
        ),
        I::Const(c) => format!(
            "{}{}const {}{}:{}{}",
            mattrs(&c.attrs),
            mt(&c.vis),
            mt(&c.ident),
            mgenerics(&c.generics),
            mtype(&c.ty),
            c.expr.as_ref().map(|e| format!("={}", mexpr(e))).unwrap_or_default()
        ),
        I::Static(s) => format!(
            "{}{}static {}:{}{}",
            mattrs(&s.attrs),
            mt(&s.vis),
            mt(&s.ident),
            mtype(&s.ty),
            s.expr.as_ref().map(|e| format!("={}", mexpr(e))).unwrap_or_default()
        ),
        I::TypeAlias(t) => format!(
            "{}{}type {}{}{}",
            mattrs(&t.attrs),
            mt(&t.vis),
            mt(&t.ident),
            mgenerics(&t.generics),
            t.ty.as_ref().map(|ty| format!("={}", mtype(ty))).unwrap_or_default()
        ),
        other => format!("tokens<{}>", mt(other)),
    }
}

pub(super) fn sitem(i: &syn::Item) -> String {
    use syn::Item as I;
    match i {
        I::Fn(f) => format!("{}{}{} {}", sattrs(&f.attrs), st(&f.vis), ssig(&f.sig), sblock(&f.block)),
        I::Struct(s) => format!(
            "{}{}struct {}{}{}",
            sattrs(&s.attrs),
            st(&s.vis),
            st(&s.ident),
            sgenerics(&s.generics),
            sfields(&s.fields)
        ),
        I::Enum(e) => format!(
            "{}{}enum {}{}{{{}}}",
            sattrs(&e.attrs),
            st(&e.vis),
            st(&e.ident),
            sgenerics(&e.generics),
            e.variants
                .iter()
                .map(|v| format!(
                    "{}{}{}{}",
                    sattrs(&v.attrs),
                    st(&v.ident),
                    sfields(&v.fields),
                    v.discriminant
                        .as_ref()
                        .map(|(_, d)| format!("={}", sexpr(d)))
                        .unwrap_or_default()
                ))
                .collect::<Vec<_>>()
                .join(",")
        ),
        I::Union(u) => format!(
            "{}{}union {}{}{}",
            sattrs(&u.attrs),
            st(&u.vis),
            st(&u.ident),
            sgenerics(&u.generics),
            sfields(&syn::Fields::Named(u.fields.clone()))
        ),
        I::Impl(x) => format!(
            "{}impl{} {}{} {{{}}}",
            sattrs(&x.attrs),
            sgenerics(&x.generics),
            x.trait_
                .as_ref()
                .map(|(path, _)| format!("{} for ", st(path)))
                .unwrap_or_default(),
            stype(&x.self_ty),
            x.items
                .iter()
                .map(|it| match it {
                    syn::ImplItem::Fn(f) => format!("{}{}{} {}", sattrs(&f.attrs), st(&f.vis), ssig(&f.sig), sblock(&f.block)),
                    other => format!("tokens<{}>", st(other)),
                })
                .collect::<Vec<_>>()
                .join(" ")
        ),
        I::Trait(t) => format!(
            "{}{}trait {}{}{} {{{}}}",
            sattrs(&t.attrs),
            st(&t.vis),
            st(&t.ident),
            sgenerics(&t.generics),
            if t.supertraits.is_empty() {
                String::new()
            } else {
                format!(":{}", sbounds(&t.supertraits))
            },
            t.items
                .iter()
                .map(|it| match it {
                    syn::TraitItem::Fn(f) => format!(
                        "{}{} {}",
                        sattrs(&f.attrs),
                        ssig(&f.sig),
                        f.default.as_ref().map(sblock).unwrap_or_else(|| ";".into())
                    ),
                    other => format!("tokens<{}>", st(other)),
                })
                .collect::<Vec<_>>()
                .join(" ")
        ),
        I::Mod(x) => format!(
            "{}{}mod {}{}",
            sattrs(&x.attrs),
            st(&x.vis),
            st(&x.ident),
            x.content
                .as_ref()
                .map(|(_, c)| format!("{{{}}}", c.iter().map(sitem).collect::<Vec<_>>().join(" ")))
                .unwrap_or_else(|| ";".into())
        ),
        I::Const(c) => format!(
            "{}{}const {}{}:{}={}",
            sattrs(&c.attrs),
            st(&c.vis),
            st(&c.ident),
            sgenerics(&c.generics),
            stype(&c.ty),
            sexpr(&c.expr)
        ),
        I::Static(s) => format!(
            "{}{}static {}:{}={}",
            sattrs(&s.attrs),
            st(&s.vis),
            st(&s.ident),
            stype(&s.ty),
            sexpr(&s.expr)
        ),
        I::Type(t) => format!(
            "{}{}type {}{}={}",
            sattrs(&t.attrs),
            st(&t.vis),
            st(&t.ident),
            sgenerics(&t.generics),
            stype(&t.ty)
        ),
        I::Verbatim(_) => SYN_VERBATIM.into(),
        other => format!("tokens<{}>", st(other)),
    }
}
