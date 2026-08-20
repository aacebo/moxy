use std::error::Error;
use std::str::FromStr;

use moxy_build::rustc::Config;
use moxy_build::rustc::instruction::{Cfg, Instruction, Link, Rerun};
use moxy_build::rustc::version::{Channel, ParseVersionError, Version};

#[test]
fn every_instruction_family_has_the_expected_cargo_protocol_text() {
    let cases = [
        (
            Instruction::Cfg(Cfg::Set("feature=\"fast\"".into())),
            "cargo::rustc-cfg=feature=\"fast\"",
        ),
        (
            Instruction::Cfg(Cfg::Check("cfg(feature, values(\"fast\"))".into())),
            "cargo::rustc-check-cfg=cfg(feature, values(\"fast\"))",
        ),
        (
            Instruction::Link(Link::Arg("-Wl,-rpath".into())),
            "cargo::rustc-link-arg=-Wl,-rpath",
        ),
        (
            Instruction::Link(Link::ArgBin("tool".into(), "-static".into())),
            "cargo::rustc-link-arg-bin=tool=-static",
        ),
        (
            Instruction::Link(Link::ArgBins("-static".into())),
            "cargo::rustc-link-arg-bins=-static",
        ),
        (
            Instruction::Link(Link::ArgTests("-pthread".into())),
            "cargo::rustc-link-arg-tests=-pthread",
        ),
        (
            Instruction::Link(Link::ArgExamples("-pthread".into())),
            "cargo::rustc-link-arg-examples=-pthread",
        ),
        (
            Instruction::Link(Link::ArgBenches("-pthread".into())),
            "cargo::rustc-link-arg-benches=-pthread",
        ),
        (
            Instruction::Link(Link::ArgCdylib("-undefined".into())),
            "cargo::rustc-link-arg-cdylib=-undefined",
        ),
        (
            Instruction::Link(Link::Lib("static=ssl".into())),
            "cargo::rustc-link-lib=static=ssl",
        ),
        (
            Instruction::Link(Link::Search("native=/opt/lib".into())),
            "cargo::rustc-link-search=native=/opt/lib",
        ),
        (
            Instruction::Rerun(Rerun::IfChanged("schema.json".into())),
            "cargo::rerun-if-changed=schema.json",
        ),
        (
            Instruction::Rerun(Rerun::IfEnvChanged("SDKROOT".into())),
            "cargo::rerun-if-env-changed=SDKROOT",
        ),
        (
            Instruction::Flags("-C target-cpu=native".into()),
            "cargo::rustc-flags=-C target-cpu=native",
        ),
        (Instruction::Env("KEY".into(), "VALUE".into()), "cargo::rustc-env=KEY=VALUE"),
        (Instruction::Warning("careful".into()), "cargo::warning=careful"),
        (Instruction::Error("stopped".into()), "cargo::error=stopped"),
        (Instruction::Metadata("ABI".into(), "v2".into()), "cargo::metadata=ABI=v2"),
    ];

    for (instruction, expected) in cases {
        assert_eq!(instruction.to_string(), expected);
        assert!(expected.starts_with(&format!("cargo::{}", instruction.as_str())));
    }
}

#[test]
fn config_helpers_preserve_a_complete_build_script_workflow_in_order() {
    let version = Version::parse("rustc 1.96.0-nightly (hash date)").unwrap();
    let mut config = Config::with_version(version);
    config
        .cfg("nightly")
        .check_cfg("cfg(nightly)")
        .env("GENERATED", "yes")
        .link_lib("static=moxy")
        .link_search("native=target/lib")
        .link_arg("-pthread")
        .rerun_if_changed("build.rs")
        .rerun_if_env_changed("RUSTC")
        .warning("using nightly")
        .error("example error")
        .push(Instruction::Metadata("KEY".into(), "VALUE".into()));

    assert_eq!(config.version(), Some(&version));
    assert_eq!(config.instructions().len(), 11);
    let lines: Vec<_> = config.instructions().iter().map(ToString::to_string).collect();
    assert_eq!(lines[0], "cargo::rustc-cfg=nightly");
    assert_eq!(lines[5], "cargo::rustc-link-arg=-pthread");
    assert_eq!(lines[10], "cargo::metadata=KEY=VALUE");
    config.emit();
}

#[test]
fn version_parsing_channels_ordering_display_and_errors_cover_boundaries() {
    for (source, triple, channel, display) in [
        ("1.0.0-dev", (1, 0, 0), Channel::Dev, "1.0.0-dev"),
        ("1.80.0-beta", (1, 80, 0), Channel::Beta, "1.80.0-beta"),
        ("1.96.0-nightly", (1, 96, 0), Channel::Nightly, "1.96.0-nightly"),
        ("1.90.1", (1, 90, 1), Channel::Stable, "1.90.1"),
    ] {
        let version = Version::parse(source).unwrap();
        assert_eq!(version.triple(), triple);
        assert_eq!(version.channel, channel);
        assert_eq!(version.to_string(), display);
        assert_eq!(Channel::parse(channel.as_str()), Some(channel));
        assert_eq!(channel.to_string(), channel.as_str());
    }

    let verbose = "rustc 1.96.0-nightly\nbinary: rustc\nrelease: 1.96.0-nightly\nLLVM version: 22";
    assert_eq!(Version::parse_verbose(verbose).unwrap().channel, Channel::Nightly);
    assert!(Version::parse_verbose("rustc 1.80.0").is_none());
    assert!(Version::parse("").is_none());
    assert!(Version::parse("1.x.0").is_none());
    assert!(Channel::parse("1.0.0-preview").is_none());
    assert!(Version::parse("1.90.0").unwrap().at_least(&Version::parse("1.89.9").unwrap()));
    assert!(!Version::parse("1.89.9").unwrap().at_least(&Version::parse("1.90.0").unwrap()));

    let error = Version::from_str("not-a-version").unwrap_err();
    assert_eq!(error, ParseVersionError);
    assert_eq!(error.to_string(), "could not parse rustc version");
    assert!(error.source().is_none());
}

#[test]
fn config_detects_the_current_rustc_and_handles_version_requirements() {
    let detected = Config::new();
    assert!(detected.version().is_some());

    let mut current = Config::with_version(Version::parse("1.80.0").unwrap());
    assert!(current.at_least("1.80"));
    assert!(!current.at_least("1.81.0"));
    assert!(!current.at_least("invalid"));
    current.require_version("1.80.0");
    assert!(current.instructions().is_empty());
    current.require_version("1.81.0");
    assert!(matches!(current.instructions(), [Instruction::Error(message)] if message.contains("found 1.80.0")));

    let mut missing = Config::default();
    missing.require_version("1.0.0").require_version("invalid");
    assert!(matches!(
        missing.instructions(),
        [Instruction::Error(_), Instruction::Error(_)]
    ));
}
