use crate::args::Config;
use std::{env, fmt::Display, process, time::Duration};

pub enum Error {
    RequestError(Box<ureq::Error>),
    InvalidResponseCode(u16),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::RequestError(err) => writeln!(f, "request error: {}", err),
            Error::InvalidResponseCode(code) => writeln!(f, "bad response code: {}", code),
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
    pub fn new() -> THC {
        let config = Config::new();
        let agent = ureq::builder()
            .timeout_connect(Duration::from_secs(config.connect_timeout))
            .timeout(Duration::from_secs(config.request_timeout))
            .build();

        THC { config, agent }
    }

    pub fn exec(self) -> Result<(), Error> {
        if env::args().len() > 1 {
            Config::usage();
            process::exit(1);
        }

        let resp = self.agent.get(&self.config.url()).call()?;

        if resp.status() >= 200 || resp.status() < 300 {
            return Ok(());
        }

        Err(Error::InvalidResponseCode(resp.status()))
    }
}

impl Default for THC {
    fn default() -> Self {
        Self::new()
    }
}
