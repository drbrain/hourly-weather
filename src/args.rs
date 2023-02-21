use rustls::{
    server::AllowAnyAnonymousOrAuthenticatedClient, Certificate, PrivateKey, RootCertStore,
    ServerConfig,
};
use rustls_pemfile::{certs, pkcs8_private_keys};
use std::{fs::File, io::BufReader, path::PathBuf};

/// Hourly weather ActivityPub server
#[derive(clap::Parser, Debug)]
#[command(about, version)]
pub struct Args {
    /// TLS CA certificate
    #[arg(long)]
    ca_certificate: PathBuf,
    /// TLS certificate
    #[arg(long)]
    certificate: PathBuf,
    /// Private key
    #[arg(long)]
    private_key: PathBuf,
}

impl Args {
    pub fn tls_config(&self) -> ServerConfig {
        let ca_certificate = &mut BufReader::new(File::open(&self.ca_certificate).unwrap());
        let ca_certificate = Certificate(certs(ca_certificate).unwrap()[0].clone());

        let mut cert_store = RootCertStore::empty();
        cert_store
            .add(&ca_certificate)
            .expect("failed to add CA cert to store");

        let certificate = &mut BufReader::new(File::open(&self.certificate).unwrap());
        let private_key = &mut BufReader::new(File::open(&self.private_key).unwrap());

        let chain = certs(certificate)
            .unwrap()
            .into_iter()
            .map(Certificate)
            .collect();
        let mut private_keys: Vec<PrivateKey> = pkcs8_private_keys(private_key)
            .unwrap()
            .into_iter()
            .map(PrivateKey)
            .collect();

        ServerConfig::builder()
            .with_safe_defaults()
            .with_client_cert_verifier(AllowAnyAnonymousOrAuthenticatedClient::new(cert_store))
            .with_single_cert(chain, private_keys.remove(0))
            .unwrap()
    }
}
