use anyhow::{bail, Result};

use crate::args;
use std::env;

pub struct THC {
    agent: ureq::Agent,
}

impl THC {
    pub fn new() -> Result<THC> {
        Ok(THC {
            agent: THC::configure_agent()?,
        })
    }

    pub fn exec(self) -> Result<()> {
        let parser = args::Parser::new(env::args().collect());
        let url = parser.validate()?.parse()?;

        let resp = self.agent.get(&url).call()?;
        if resp.status() >= 200 || resp.status() < 300 {
            return Ok(());
        }

        bail!("invalid response code {}", resp.status())
    }

    fn configure_agent() -> Result<ureq::Agent> {
        let conn_timeout: u64 = std::env::var("CONN_TIMEOUT")
            .unwrap_or_else(|_| String::from("10"))
            .parse()?;

        let timeout: u64 = std::env::var("REQ_TIMEOUT")
            .unwrap_or_else(|_| String::from("15"))
            .parse()?;

        Ok(ureq::builder()
            .timeout_connect(std::time::Duration::from_secs(conn_timeout))
            .timeout(std::time::Duration::from_secs(timeout))
            .build())
    }
}
