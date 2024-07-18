use quinn::{ClientConfig, ServerConfig, TransportConfig};
use rustls::{Certificate, PrivateKey};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use anyhow::*;

pub const SERVER_PORT: u16 = 5000;
pub const SERVER_CERT_PATH: &str = "cert.der";
pub const SERVER_KEY_PATH: &str = "key.der";

pub fn generate_self_signed_cert() -> Result<(Vec<u8>, Vec<u8>)> {
    let cert = rcgen::generate_simple_self_signed(vec!["localhost".into()])?;
    let cert_der = cert.serialize_der()?;
    let priv_key = cert.serialize_private_key_der();
    Ok((cert_der, priv_key))
}

pub fn save_cert_and_key(cert_der: &[u8], key_der: &[u8]) -> Result<()> {
    std::fs::write(SERVER_CERT_PATH, cert_der)?;
    std::fs::write(SERVER_KEY_PATH, key_der)?;
    Ok(())
}

pub fn load_cert_and_key() -> Result<(Vec<u8>, Vec<u8>)> {
    let cert_der = std::fs::read(SERVER_CERT_PATH)?;
    let key_der = std::fs::read(SERVER_KEY_PATH)?;
    Ok((cert_der, key_der))
}

pub fn configure_server(cert_der: Vec<u8>, priv_key: Vec<u8>) -> Result<ServerConfig> {
    let mut server_config = ServerConfig::with_single_cert(
        vec![Certificate(cert_der)],
        PrivateKey(priv_key)
    )?;
    server_config.transport = Arc::new(TransportConfig::default());
    Ok(server_config)
}

pub fn configure_client(cert_der: Vec<u8>) -> Result<ClientConfig> {
    let mut roots = rustls::RootCertStore::empty();
    roots.add(&Certificate(cert_der))?;

    let client_config = ClientConfig::new(Arc::new(
        rustls::ClientConfig::builder()
            .with_safe_defaults()
            .with_root_certificates(roots)
            .with_no_client_auth()
    ));

    Ok(client_config)
}

pub fn get_server_addr() -> SocketAddr {
    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), SERVER_PORT)
}