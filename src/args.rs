use std::{env, process};

pub struct Config {
    port: u32,
    path: String,
    pub connect_timeout: u64,
    pub request_timeout: u64,
    use_loopback_addr: bool,
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
            use_loopback_addr: env::var("THC_USE_LOOPBACK_ADDRESS")
                .unwrap_or_else(|_| "false".into())
                .parse()
                .expect("THC_USE_LOOPBACK_ADDRESS must be 'true' or 'false' if specified"), 

        }
    }

    pub fn url(&self) -> String {
        let host = if self.use_loopback_addr { "127.0.0.1" } else { "localhost" };
        format!("http://{}:{}{}", host, self.port, self.path)
    }

    pub fn usage() {
        println!("USAGE:");
        println!("\tthc");
        println!();
        println!("ENV:");
        println!();
        println!("\tTHC_PORT sets the port to which a connection will be made, default: 8080");
        println!("\tTHC_PATH sets the path to which a connection will be made, default: `/`");
        println!("\tTHC_CONN_TIMEOUT sets the connection timeout, default: 10");
        println!("\tTHC_REQ_TIMEOUT sets the request timeout, default: 15");
        println!("\tTHC_USE_LOOPBACK_ADDRESS 'true' to use 127.0.0.1 in place of 'localhost', default: `false`");
        println!();
        println!("\t**NOTE** Host is not configurable and will always be localhost (or 127.0.0.1)");
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

    #[test]
    fn it_parses_loopback_adress_override() {
        temp_env::with_vars(
            vec![("THC_USE_LOOPBACK_ADDRESS", Some("true"))],
            || {
                assert_eq!(Config::new(&[]).url(), "http://127.0.0.1:8080/");
            },
        );
    }

}
