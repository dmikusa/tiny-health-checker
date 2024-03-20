use crate::args::Config;
use rustls::{client::danger::{HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier}, pki_types::{CertificateDer, ServerName, UnixTime}, ClientConfig, DigitallySignedStruct, SignatureScheme};
use std::{env, fmt::Display, process, sync::Arc, time::Duration};

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
    pub fn new() -> THC {
        let config = Config::new();

        let client_config = ClientConfig::builder()
        .dangerous().with_custom_certificate_verifier(Arc::new(NoopServerVerifier {}))
        .with_no_client_auth();

        let agent = ureq::AgentBuilder::new()
            .timeout_connect(Duration::from_secs(config.connect_timeout))
            .timeout(Duration::from_secs(config.request_timeout))
            .tls_config(Arc::new(client_config))
            .build();

        THC { config, agent }
    }

    pub fn exec(self) -> Result<(), Error> {
        if env::args().len() > 1 {
            Config::usage();
            process::exit(0);
        }

        let resp = self.agent.get(&self.config.url()).call()?;

        if resp.status() >= 200 || resp.status() < 300 {
            return Ok(());
        }

        Err(Error::InvalidResponseCode(resp.status()))
    }
}

#[derive(Debug)]
struct NoopServerVerifier;

impl ServerCertVerifier for NoopServerVerifier {
    fn verify_server_cert(
        &self,
        _end_entity: &CertificateDer<'_>,
        _intermediates: &[CertificateDer<'_>],
        _server_name: &ServerName,
        _ocsp_response: &[u8],
        _now: UnixTime,
    ) -> Result<ServerCertVerified, rustls::Error> {
        Ok(ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        _message: &[u8],
        _cert: &CertificateDer<'_>,
        _dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        Ok(HandshakeSignatureValid::assertion())
    }

    fn verify_tls13_signature(
        &self,
        _message: &[u8],
        _cert: &CertificateDer<'_>,
        _dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        Ok(HandshakeSignatureValid::assertion())
    }

    fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
        vec![
            SignatureScheme::RSA_PKCS1_SHA1,
            SignatureScheme::ECDSA_SHA1_Legacy,
            SignatureScheme::RSA_PKCS1_SHA256,
            SignatureScheme::ECDSA_NISTP256_SHA256,
            SignatureScheme::RSA_PKCS1_SHA384,
            SignatureScheme::ECDSA_NISTP384_SHA384,
            SignatureScheme::RSA_PKCS1_SHA512,
            SignatureScheme::ECDSA_NISTP521_SHA512,
            SignatureScheme::RSA_PSS_SHA256,
            SignatureScheme::RSA_PSS_SHA384,
            SignatureScheme::RSA_PSS_SHA512,
            SignatureScheme::ED25519,
            SignatureScheme::ED448,
        ]
    }
}

impl Default for THC {
    fn default() -> Self {
        Self::new()
    }
}
