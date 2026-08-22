use moxy_build as build;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cfg = build::rustc::Config::read()?;

    cfg.min_version("1.31.0")
        .check_cfg("cfg(nightly)")
        .rerun_if_changed("build.rs");

    if cfg.version().channel.is_nightly() {
        cfg.cfg("nightly");
    }

    cfg.emit();
    Ok(())
}
