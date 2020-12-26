use minigrep::Config;
use std::error::Error;

#[test]
fn no_argument() {
    let args = vec![];
    assert!(Config::new(&args).is_err());
}

#[test]
fn it_runs() -> Result<(), Box<dyn Error>> {
    minigrep::run(Config {
        query: "foo".to_string(),
        filename: "/dev/null".to_string(),
        case_sensitive: false,
    })
}
