use std::{env, process};

pub struct Config {
    port: u32,
    path: String,
    pub connect_timeout: u64,
    pub request_timeout: u64,
}

impl Config {
    pub fn new(args: &[String]) -> Config {
        if args.len() > 1 {
            Config::usage();
            process::exit(0);
        }

        Config {
            port: env::var("THC_PORT")
                .unwrap_or_else(|_| "8080".into())
                .parse()
                .expect("invalid port in THC_PORT"),
            path: env::var("THC_PATH")
                .map(|p| {
                    if p.starts_with('/') {
                        p
                    } else {
                        format!("/{p}")
                    }
                })
                .unwrap_or_else(|_| "/".into()),
            connect_timeout: env::var("THC_CONN_TIMEOUT")
                .unwrap_or_else(|_| "10".into())
                .parse()
                .expect("invalid connection timeout"),
            request_timeout: env::var("THC_REQ_TIMEOUT")
                .unwrap_or_else(|_| "15".into())
                .parse()
                .expect("invalid request timeout"),
        }
    }

    pub fn url(&self) -> String {
        format!("http://localhost:{}{}", self.port, self.path)
    }

    pub fn usage() {
        println!("USAGE:");
        println!("\tthc");
        println!();
        println!("ENV:");
        println!();
        println!("\tTHC_PORT sets the port to which a connection will be made, default: 8080");
        println!("\tTHC_PATH sets the path to which a connection will be made, default `/`");
        println!("\tCONN_TIMEOUT sets the connection timeout, default: 10");
        println!("\tREQ_TIMEOUT sets the request timeout, defaults: 15");
        println!();
        println!("\t**NOTE** Host is not configurable and will always be localhost");
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::Config;

    #[test]
    fn it_parses_default_url() {
        temp_env::with_vars_unset(vec!["THC_PORT", "THC_PATH"], || {
            assert_eq!(Config::new(&[]).url(), "http://localhost:8080/");
        });
    }

    #[test]
    fn it_parses_port_with_default_path() {
        temp_env::with_vars(
            vec![("THC_PORT", Some("8081")), ("THC_PATH", None::<&str>)],
            || {
                assert_eq!(Config::new(&[]).url(), "http://localhost:8081/");
            },
        );
    }

    #[test]
    fn it_parses_port_and_path() {
        temp_env::with_vars(
            vec![("THC_PORT", Some("8081")), ("THC_PATH", Some("/foo"))],
            || {
                assert_eq!(Config::new(&[]).url(), "http://localhost:8081/foo");
            },
        );
    }

    #[test]
    fn it_parses_port_and_path_with_no_leading_slash() {
        temp_env::with_vars(
            vec![("THC_PORT", Some("8081")), ("THC_PATH", Some("foo"))],
            || {
                assert_eq!(Config::new(&[]).url(), "http://localhost:8081/foo");
            },
        );
    }
}
