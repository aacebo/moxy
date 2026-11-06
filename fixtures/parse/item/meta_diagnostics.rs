#[bar]
#[foo(min = 1)]
struct MissingRequired {}

#[bar]
#[foo]
struct BareAttribute {}

#[bar]
#[foo = 1]
struct ValueAttribute {}

#[bar]
#[foo(min = 1, max = 2, nope)]
struct UnknownBare {}

#[bar]
#[foo(min = 1, max = 2, nope = 3)]
struct UnknownValue {}

#[bar]
#[foo(min = 1, max = 2, nope(1))]
struct UnknownList {}
