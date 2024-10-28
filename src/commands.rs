use crate::args::Config;
use std::{fmt::Display, time::Duration};

pub enum Error {
    RequestError(Box<ureq::Error>),
    InvalidResponseCode(u16),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::RequestError(err) => writeln!(f, "request error: {err}"),
            Error::InvalidResponseCode(code) => writeln!(f, "bad response code: {code}"),
        }
    }
}

impl From<ureq::Error> for Error {
    fn from(err: ureq::Error) -> Self {
        Error::RequestError(Box::new(err))
    }
}

pub struct THC {
    config: Config,
    agent: ureq::Agent,
}

impl THC {
    pub fn new(args: &[String]) -> THC {
        let config = Config::new(args);
        let agent = ureq::builder()
            .timeout_connect(Duration::from_secs(config.connect_timeout))
            .timeout(Duration::from_secs(config.request_timeout))
            .build();

        THC { config, agent }
    }

    pub fn exec(self) -> Result<(), Error> {
        let result = self.agent.get(&self.config.url()).call();
        match result {
            Err(ureq::Error::Status(status, _)) => Err(Error::InvalidResponseCode(status)),
            Err(err) => Err(Error::RequestError(Box::new(err))),
            Ok(resp) => {
                if resp.status() >= 200 && resp.status() < 300 {
                    return Ok(());
                }
                Err(Error::InvalidResponseCode(resp.status()))
            }
        }
    }
}

impl Default for THC {
    fn default() -> Self {
        Self::new(&[])
    }
}

#[cfg(test)]
mod tests {
    use httpmock::{prelude::*, Mock};

    use crate::Error;

    use super::THC;

    fn mock_server<T>(closure: T)
    where
        T: Fn(&MockServer),
    {
        let server = MockServer::start();
        temp_env::with_var("THC_PORT", Some(server.port().to_string()), || {
            closure(&server);
        })
    }

    fn mock_response<'a>(server: &'a MockServer, path: &'a str, status: u16) -> Mock<'a> {
        server.mock(|when, then| {
            when.method(GET).path(path);
            then.status(status)
                .header("content-type", "text/plain")
                .body("OK");
        })
    }

    #[test]
    pub fn it_gets_200_response() {
        mock_server(|server| {
            let mock = mock_response(&server, "/", 200);

            let thc = THC::new(&[]);

            let result = thc.exec();

            assert!(result.is_ok(), "request failed {}", result.unwrap_err());
            mock.assert();
        });
    }

    #[test]
    pub fn it_gets_200_response_on_custom_path() {
        mock_server(|server| {
            let mock = mock_response(server, "/custom", 200);

            temp_env::with_var("THC_PATH", Some("/custom"), || {
                let thc = THC::new(&[]);

                let result = thc.exec();

                assert!(result.is_ok(), "request failed {}", result.unwrap_err());
                mock.assert();
            })
        });
    }

    #[test]
    pub fn it_gets_302_response() {
        mock_server(|server| {
            let mock = mock_response(&server, "/", 302);

            let thc = THC::new(&[]);

            let result = thc.exec();

            assert!(result.is_err(), "request was ok");

            match result.unwrap_err() {
                Error::InvalidResponseCode(code) => assert_eq!(code, 302),
                Error::RequestError(err) => assert!(false, "unexpected ureq err -> {}", err),
            }

            mock.assert();
        });
    }

    #[test]
    pub fn it_gets_404_response() {
        mock_server(|server| {
            let mock = mock_response(&server, "/", 404);

            let thc = THC::new(&[]);

            let result = thc.exec();

            assert!(result.is_err(), "request was ok");

            match result.unwrap_err() {
                Error::InvalidResponseCode(code) => assert_eq!(code, 404),
                Error::RequestError(err) => assert!(false, "unexpected ureq err -> {}", err),
            }

            mock.assert();
        });
    }
}
