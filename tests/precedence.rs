use moxy::ast::{ClosurePipes, Expr, Postfix, RangeLimits};
use moxy::token::{ToTokenStream, TokenStream};

// The expression with every operation in parentheses, so two parses compare by shape
fn parenthesize(expression: &Expr) -> String {
    match expression {
        // A chain stands for its operations nested to the left
        Expr::BinaryChain(v) => {
            let mut pairs = v.operands.pairs();
            let first = pairs.next().unwrap();
            let mut rendered = parenthesize(first.value());
            let mut op = first.punct().map(compact);

            for pair in pairs {
                rendered = format!("({rendered} {} {})", op.unwrap(), parenthesize(pair.value()));
                op = pair.punct().map(compact);
            }

            rendered
        }
        Expr::Assign(v) => format!("({} = {})", parenthesize(&v.left), parenthesize(&v.right)),
        Expr::Unary(v) => format!("({}{})", compact(&v.op), parenthesize(&v.expr)),
        Expr::Reference(v) => {
            let mutability = if v.mutability.is_some() { "mut " } else { "" };
            format!("(&{mutability}{})", parenthesize(&v.expr))
        }
        Expr::CastChain(v) => v.casts.iter().fold(parenthesize(&v.expr), |rendered, (_, ty)| {
            format!("({rendered} as {})", compact(ty))
        }),
        Expr::Range(v) => {
            let limits = match v.limits {
                RangeLimits::HalfOpen(_) => "..",
                RangeLimits::Closed(_) => "..=",
            };
            let start = v.start.as_deref().map(parenthesize).unwrap_or_default();
            let end = v.end.as_deref().map(parenthesize).unwrap_or_default();
            format!("({start}{limits}{end})")
        }
        // A chain stands for its operations nested to the left
        Expr::Postfix(v) => {
            let mut rendered = parenthesize(&v.base);

            for op in &v.ops {
                rendered = match op {
                    Postfix::MethodCall(c) => format!("({rendered}.{}({}))", c.method, list(c.args.inner.iter())),
                    Postfix::Field(f) => format!("({rendered}.{})", compact(&f.member)),
                    Postfix::Call(c) => format!("({rendered}({}))", list(c.args.inner.iter())),
                    Postfix::Index(i) => format!("({rendered}[{}])", parenthesize(&i.index.inner)),
                    Postfix::Try(_) => format!("({rendered}?)"),
                    Postfix::Await(_) => format!("({rendered}.await)"),
                };
            }

            rendered
        }
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
        Expr::IfChain(v) => format!("(if {} {{..}})", parenthesize(&v.cond)),
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

#[test]
fn nothing_continues_a_range_at_its_own_level() {
    assert_shapes(&[
        ("a.. ..b", "(a..(..b))"),
        ("a .. ..=b", "(a..(..=b))"),
        ("a = b..c = d", "((a = (b..c)) = d)"),
        ("a += b..c = d", "((a += (b..c)) = d)"),
        ("a = b = c..d = e", "(a = ((b = (c..d)) = e))"),
    ]);

    for source in [
        "a..b = c",
        "a .. = b",
        "a..b = c..d",
        "a..b..c",
        "..a..b",
        "a..=b..c",
        "....",
        "a....b",
    ] {
        assert!(moxy::parse!(source as Expr).is_err(), "`{source}`");
    }

    let range: Expr = moxy::parse!("a.. ..b").unwrap();
    assert_eq!(moxy::fmt!(&range).unwrap(), "a.. ..b");
}

#[test]
fn an_operand_may_be_a_range_without_a_start() {
    assert_shapes(&[
        ("f(&..)", "(f((&(..))))"),
        ("&..b", "(&(..b))"),
        ("&mut ..", "(&mut (..))"),
        ("a + ..b", "(a + (..b))"),
        ("&..b + c", "(&(..(b + c)))"),
        ("-..a", "(-(..a))"),
    ]);
}
