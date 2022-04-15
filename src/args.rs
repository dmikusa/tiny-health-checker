use anyhow::{bail, Result};

pub struct Parser {
    args: Vec<String>,
}

impl Parser {
    pub fn new(args: Vec<String>) -> Parser {
        Parser { args }
    }

    pub fn validate(self) -> Result<Self> {
        if self.args.iter().any(|a| a == "-h") {
            self.usage();
            bail!("")
        }

        if self.args.len() > 3 {
            eprintln!("Too many arguments");
            self.usage();
            bail!("too many arguments")
        }

        Ok(self)
    }

    pub fn parse(self) -> Result<String> {
        match self.args.len() {
            1 => Ok(String::from("http://localhost:8080/")),
            2 => Ok(format!("http://localhost:{}/", self.args.get(1).unwrap())),
            3 => Ok(format!(
                "http://localhost:{}{}",
                self.args.get(1).unwrap(),
                self.args
                    .get(2)
                    .map(|p| if p.starts_with('/') {
                        p.into()
                    } else {
                        format!("/{}", p)
                    })
                    .unwrap(),
            )),
            _ => bail!("too many arguments"),
        }
    }

    fn usage(&self) {
        eprintln!("USAGE:");
        eprintln!("\tthc [port] [path]");
        eprintln!();
        eprintln!("ARGS:");
        eprintln!();
        eprintln!("\tport is the port to which a connection will be made, default: 8080");
        eprintln!("\tpath is the path to which a connection will be made, default: /");
        eprintln!();
        eprintln!("ENV:");
        eprintln!();
        eprintln!("\tCONN_TIMEOUT sets the connection timeout, default: 10");
        eprintln!("\tREQ_TIMEOUT sets the request timeout, defaults: 15");
        eprintln!();
        eprintln!();
        eprintln!("\t**NOTE** Host is not configurable and will always be localhost");
        eprintln!();
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;

    #[test]
    fn it_parses_default_url() {
        assert_eq!(
            Parser::new(vec![String::from("thc")])
                .validate()
                .unwrap()
                .parse()
                .unwrap(),
            "http://localhost:8080/"
        );
    }

    #[test]
    fn it_parses_port_with_default_path() {
        assert_eq!(
            Parser::new(vec![String::from("thc"), String::from("8081")])
                .validate()
                .unwrap()
                .parse()
                .unwrap(),
            "http://localhost:8081/"
        );
    }

    #[test]
    fn it_parses_port_and_path() {
        assert_eq!(
            Parser::new(vec![
                String::from("thc"),
                String::from("8081"),
                String::from("/foo")
            ])
            .validate()
            .unwrap()
            .parse()
            .unwrap(),
            "http://localhost:8081/foo"
        );
    }

    #[test]
    fn it_parses_port_and_path_with_no_leading_slash() {
        assert_eq!(
            Parser::new(vec![
                String::from("thc"),
                String::from("8081"),
                String::from("foo")
            ])
            .validate()
            .unwrap()
            .parse()
            .unwrap(),
            "http://localhost:8081/foo"
        );
    }

    #[test]
    fn it_validates_no_args_ok() {
        assert!(Parser::new(vec![String::from("thc")]).validate().is_ok());
    }

    #[test]
    fn it_validates_help_in_either_arg() {
        assert_eq!(
            Parser::new(vec![String::from("thc"), String::from("-h")])
                .validate()
                .err()
                .unwrap()
                .to_string(),
            ""
        );

        assert_eq!(
            Parser::new(vec![
                String::from("thc"),
                String::from("4"),
                String::from("-h")
            ])
            .validate()
            .err()
            .unwrap()
            .to_string(),
            ""
        );
    }

    #[test]
    fn it_validates_too_many_args() {
        assert_eq!(
            Parser::new(vec![
                String::from("thc"),
                String::from("foo"),
                String::from("bar"),
                String::from("baz")
            ])
            .validate()
            .err()
            .unwrap()
            .to_string(),
            "too many arguments"
        );
    }
}
