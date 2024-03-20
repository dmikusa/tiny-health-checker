use std::env;

pub struct Config {
    port: u32,
    path: String,
    pub connect_timeout: u64,
    pub request_timeout: u64,
    scheme: String,
}

impl Config {
    pub fn new() -> Config {
        let scheme: String = match env::var("THC_SCHEME") {
            Ok(s) if s == "http" || s == "https" => s,
            Ok(_) => panic!("invalid THC_SCHEME. It should be either 'http' or 'https'."),
            Err(_) => "http".into(), // Default to http if not set
        };

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
            scheme,
        }
    }

    pub fn url(&self) -> String {
        format!("{}://localhost:{}{}", self.scheme, self.port, self.path)
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
        println!("\tTHC_SCHEME sets the protocol scheme of the url ('http' or 'https'), default: http");
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
            assert_eq!(Config::new().url(), "http://localhost:8080/");
        });
    }

    #[test]
    fn it_parses_port_with_default_path() {
        temp_env::with_vars(
            vec![("THC_PORT", Some("8081")), ("THC_PATH", None::<&str>)],
            || {
                assert_eq!(Config::new().url(), "http://localhost:8081/");
            },
        );
    }

    #[test]
    fn it_parses_port_and_path() {
        temp_env::with_vars(
            vec![("THC_PORT", Some("8081")), ("THC_PATH", Some("/foo"))],
            || {
                assert_eq!(Config::new().url(), "http://localhost:8081/foo");
            },
        );
    }

    #[test]
    fn it_parses_port_and_path_with_no_leading_slash() {
        temp_env::with_vars(
            vec![("THC_PORT", Some("8081")), ("THC_PATH", Some("foo"))],
            || {
                assert_eq!(Config::new().url(), "http://localhost:8081/foo");
            },
        );
    }

    #[test]
    fn it_parses_default_scheme() {
        temp_env::with_vars_unset(vec!["THC_SCHEME", "THC_PORT", "THC_PATH"], || {
            assert_eq!(Config::new().url(), "http://localhost:8080/");
        });
    }

    #[test]
    fn it_parses_http_scheme() {
        temp_env::with_vars(
            vec![
                ("THC_PORT", Some("8081")),
                ("THC_PATH", Some("foo")),
                ("THC_SCHEME", Some("http")),
            ],
            || {
                assert_eq!(Config::new().url(), "http://localhost:8081/foo");
            },
        );
    }

    #[test]
    fn it_parses_https_scheme() {
        temp_env::with_vars(vec![("THC_PORT", Some("8081")),
        ("THC_PATH", Some("foo")),
        ("THC_SCHEME", Some("https")),], || {
            assert_eq!(Config::new().url(), "https://localhost:8081/foo");
        });
    }

    #[test]
    #[should_panic(
        expected = "invalid THC_SCHEME. It should be either 'http' or 'https'."
    )]
    fn it_panics_on_invalid_scheme() {
        temp_env::with_vars(
            vec![
                ("THC_PORT", Some("8081")),
                ("THC_PATH", Some("foo")),
                ("THC_SCHEME", Some("ftp")),
            ],
            || {
                Config::new();
            },
        );
    }
}
