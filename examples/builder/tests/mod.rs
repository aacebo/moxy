use builder::builder;

#[test]
fn success() {
    #[builder]
    pub struct Config {
        host: String,
        port: u16,
        verbose: bool,
    }

    let config = Config::builder().host("localhost".into()).port(8080).verbose(true).build();
    assert_eq!(&config.host, "localhost");
    assert_eq!(config.port, 8080);
    assert!(config.verbose);
}
