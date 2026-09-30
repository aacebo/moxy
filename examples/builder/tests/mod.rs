use builder::Builder;

#[test]
fn success() {
    #[derive(Builder)]
    pub struct Config {
        host: String,
        port: u16,
        #[build(rename = "enable_verbose")]
        verbose: bool,
        #[build(default)]
        retries: usize,
    }

    let config = Config::builder()
        .host("localhost".into())
        .port(8080)
        .enable_verbose(true)
        .build();
    assert_eq!(&config.host, "localhost");
    assert_eq!(config.port, 8080);
    assert!(config.verbose);
    assert_eq!(config.retries, 0);

    let configured = Config::builder()
        .host("localhost".into())
        .port(8080)
        .enable_verbose(true)
        .retries(3)
        .build();
    assert_eq!(configured.retries, 3);
}
