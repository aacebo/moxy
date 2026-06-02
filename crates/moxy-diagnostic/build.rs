extern crate moxy_build as build;

fn main() {
    build::rustc::Config::new()
        .require_version("1.31.0")
        .check_cfg("cfg(nightly)")
        .cfg("nightly")
        .emit();
}
