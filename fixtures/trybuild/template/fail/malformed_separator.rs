fn main() {
    let args = ["a", "b"];
    let _ = moxy::template! { f(@for arg in &args { {{ arg }} } @sep) };
}
