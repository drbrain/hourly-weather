use axum_server::tls_rustls::RustlsConfig;
use std::path::PathBuf;

/// Hourly weather ActivityPub server
#[derive(clap::Parser, Debug)]
#[command(about, version)]
pub struct Args {
    /// TLS certificate
    #[arg(long)]
    certificate: PathBuf,
    /// Private key
    #[arg(long)]
    private_key: PathBuf,
    /// Images directory
    #[arg(long)]
    images_dir: PathBuf,
    /// sky.jpeg root
    #[arg(long)]
    sky_jpeg: PathBuf,
}

impl Args {
    pub fn images_dir(&self) -> PathBuf {
        self.images_dir.clone()
    }

    pub async fn tls_config(&self) -> RustlsConfig {
        RustlsConfig::from_pem_file(self.certificate.clone(), self.private_key.clone())
            .await
            .unwrap()
    }

    pub fn sky_jpeg(&self) -> PathBuf {
        self.sky_jpeg.clone()
    }
}
