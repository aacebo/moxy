extern crate version_check as rustc;

fn main() {
    // hack to use rustc::version_check to check if diagnostics are supported
    if let Some((version, channel, _)) = rustc::triple() {
        if version.at_least("1.31.0") && channel.supports_features() {
            println!("cargo::rustc-check-cfg=cfg(nightly)");
            println!("cargo::rustc-cfg=nightly");
        }
    }
}
