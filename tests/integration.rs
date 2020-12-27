use minigrep::Config;
use std::error::Error;

#[test]
fn no_argument() {
    assert!(Config::new(std::env::args()).is_err());
}

#[test]
fn it_runs() -> Result<(), Box<dyn Error>> {
    minigrep::run(Config {
        query: "foo".to_string(),
        filename: "/dev/null".to_string(),
        case_sensitive: false,
    })
}
