use moxy::ast::{ClosurePipes, Expr, RangeLimits};
use moxy::token::{ToTokenStream, TokenStream};

// The expression with every operation in parentheses, so two parses compare by shape
fn parenthesize(expression: &Expr) -> String {
    match expression {
        Expr::Binary(v) => format!("({} {} {})", parenthesize(&v.left), compact(&v.op), parenthesize(&v.right)),
        Expr::Assign(v) => format!("({} = {})", parenthesize(&v.left), parenthesize(&v.right)),
        Expr::Unary(v) => format!("({}{})", compact(&v.op), parenthesize(&v.expr)),
        Expr::Reference(v) => {
            let mutability = if v.mutability.is_some() { "mut " } else { "" };
            format!("(&{mutability}{})", parenthesize(&v.expr))
        }
        Expr::Cast(v) => format!("({} as {})", parenthesize(&v.expr), compact(&v.ty)),
        Expr::Range(v) => {
            let limits = match v.limits {
                RangeLimits::HalfOpen(_) => "..",
                RangeLimits::Closed(_) => "..=",
            };
            let start = v.start.as_deref().map(parenthesize).unwrap_or_default();
            let end = v.end.as_deref().map(parenthesize).unwrap_or_default();
            format!("({start}{limits}{end})")
        }
        Expr::MethodCall(v) => format!("({}.{}({}))", parenthesize(&v.receiver), v.method, list(v.args.inner.iter())),
        Expr::Field(v) => format!("({}.{})", parenthesize(&v.base), compact(&v.member)),
        Expr::Call(v) => format!("({}({}))", parenthesize(&v.func), list(v.args.inner.iter())),
        Expr::Index(v) => format!("({}[{}])", parenthesize(&v.base), parenthesize(&v.index.inner)),
        Expr::Try(v) => format!("({}?)", parenthesize(&v.expr)),
        Expr::Await(v) => format!("({}.await)", parenthesize(&v.base)),
        Expr::Paren(v) => format!("({})", parenthesize(&v.content.inner)),
        Expr::Let(v) => format!("(let {} = {})", compact(&v.pat), parenthesize(&v.expr)),
        Expr::Closure(v) => {
            let inputs = match v.pipes {
                ClosurePipes::Empty(_) => String::new(),
                ClosurePipes::Params(..) => compact(&v.inputs),
            };
            format!("(|{inputs}| {})", parenthesize(&v.body))
        }
        Expr::Return(v) => match &v.expr {
            Some(value) => format!("(return {})", parenthesize(value)),
            None => "(return)".to_string(),
        },
        Expr::Break(v) => match &v.expr {
            Some(value) => format!("(break {})", parenthesize(value)),
            None => "(break)".to_string(),
        },
        Expr::If(v) => format!("(if {} {{..}})", parenthesize(&v.cond)),
        Expr::While(v) => format!("(while {} {{..}})", parenthesize(&v.cond)),
        Expr::Match(v) => format!("(match {} {{..}})", parenthesize(&v.expr)),
        other => compact(other),
    }
}

fn list<'a>(values: impl Iterator<Item = &'a Expr>) -> String {
    values.map(parenthesize).collect::<Vec<_>>().join(", ")
}

fn compact(value: &impl ToTokenStream) -> String {
    value.to_token_stream().to_string().replace(' ', "")
}

fn assert_shapes(cases: &[(&str, &str)]) {
    for &(source, expected) in cases {
        let expression: Expr = moxy::parse!(source).unwrap_or_else(|error| panic!("`{source}`: {error}"));
        assert_eq!(parenthesize(&expression), expected, "`{source}`");

        // The tree emits the source tokens unchanged, with no inserted groups
        let lexed: TokenStream = source.parse().unwrap();
        assert_eq!(expression.to_token_stream().to_string(), lexed.to_string(), "`{source}`");
    }
}

#[test]
fn every_adjacent_pair_of_precedence_levels_nests_the_stronger_operator() {
    assert_shapes(&[
        // Postfix operators bind tighter than unary ones
        ("-a.b", "(-(a.b))"),
        ("-a?", "(-(a?))"),
        ("!a()", "(!(a()))"),
        ("*a[0]", "(*(a[0]))"),
        ("&a.b()", "(&(a.b()))"),
        ("-a.await", "(-(a.await))"),
        // Unary operators bind tighter than `as`
        ("-a as u8", "((-a) as u8)"),
        ("&a as *const u8", "((&a) as *constu8)"),
        ("!a as u8", "((!a) as u8)"),
        // `as` binds tighter than `*`, `/`, and `%`
        ("a * b as u8", "(a * (b as u8))"),
        ("a as u8 % b", "((a as u8) % b)"),
        // `*`, `/`, and `%` bind tighter than `+` and `-`
        ("a + b * c", "(a + (b * c))"),
        ("a / b - c", "((a / b) - c)"),
        // `+` and `-` bind tighter than shifts
        ("a << b + c", "(a << (b + c))"),
        ("a - b >> c", "((a - b) >> c)"),
        // Shifts bind tighter than `&`
        ("a & b << c", "(a & (b << c))"),
        ("a >> b & c", "((a >> b) & c)"),
        // `&` binds tighter than `^`
        ("a ^ b & c", "(a ^ (b & c))"),
        ("a & b ^ c", "((a & b) ^ c)"),
        // `^` binds tighter than `|`
        ("a | b ^ c", "(a | (b ^ c))"),
        ("a ^ b | c", "((a ^ b) | c)"),
        // `|` binds tighter than comparisons
        ("a == b | c", "(a == (b | c))"),
        ("a | b < c", "((a | b) < c)"),
        // Comparisons bind tighter than `&&`
        ("a && b != c", "(a && (b != c))"),
        ("a >= b && c", "((a >= b) && c)"),
        // `&&` binds tighter than `||`
        ("a || b && c", "(a || (b && c))"),
        ("a && b || c", "((a && b) || c)"),
        // `||` binds tighter than ranges
        ("a..b || c", "(a..(b || c))"),
        ("a || b..=c", "((a || b)..=c)"),
        // Ranges bind tighter than assignment
        ("a = b..c", "(a = (b..c))"),
        ("a += ..c", "(a += (..c))"),
        // Assignment binds tighter than `return` and closure bodies
        ("return a = b", "(return (a = b))"),
        ("|x| a += x", "(|x| (a += x))"),
    ]);
}

#[test]
fn operators_of_one_precedence_level_associate_as_rust_defines() {
    assert_shapes(&[
        ("a - b - c", "((a - b) - c)"),
        ("a / b * c % d", "(((a / b) * c) % d)"),
        ("a << b >> c", "((a << b) >> c)"),
        ("a & b & c", "((a & b) & c)"),
        ("a ^ b ^ c", "((a ^ b) ^ c)"),
        ("a | b | c", "((a | b) | c)"),
        ("a && b && c", "((a && b) && c)"),
        ("a || b || c", "((a || b) || c)"),
        ("a as u8 as u16", "((a as u8) as u16)"),
        ("--a", "(-(-a))"),
        ("a.b.c()?", "(((a.b).c())?)"),
        // Assignment and compound assignment associate to the right
        ("a = b = c", "(a = (b = c))"),
        ("a += b -= c", "(a += (b -= c))"),
        ("a = b *= c", "(a = (b *= c))"),
        ("x += y + z", "(x += (y + z))"),
        ("a <<= b || c", "(a <<= (b || c))"),
    ]);
}

#[test]
fn let_chains_end_the_scrutinee_before_a_logical_operator() {
    assert_shapes(&[
        ("if let Some(x) = a && b {}", "(if ((let Some(x) = a) && b) {..})"),
        (
            "if let Some(x) = a && let Some(y) = x {}",
            "(if ((let Some(x) = a) && (let Some(y) = x)) {..})",
        ),
        ("if a && let Some(y) = x && b {}", "(if ((a && (let Some(y) = x)) && b) {..})"),
        ("if let Some(x) = a || b {}", "(if ((let Some(x) = a) || b) {..})"),
        ("if let x = a == b {}", "(if (let x = (a == b)) {..})"),
        ("if let x = a | b {}", "(if (let x = (a | b)) {..})"),
        ("while let Some(x) = a && b {}", "(while ((let Some(x) = a) && b) {..})"),
    ]);
}
