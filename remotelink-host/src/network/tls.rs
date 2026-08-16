use rcgen::{Certificate, CertificateParams, KeyPair, PKCS_ECDSA_P256_SHA256};
use rustls::pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer};
use rustls::ServerConfig;
use std::error::Error;

pub fn generate_self_signed_cert() -> Result<(CertificateDer<'static>, PrivateKeyDer<'static>), Box<dyn Error>> {
    println!("Generating self-signed RSA cert...");
    
    let params = CertificateParams::new(vec!["localhost".to_string()]);
    let cert = Certificate::from_params(params)?;
    
    let cert_der = cert.serialize_der()?;
    let key_der = cert.serialize_private_key_der();

    let cert = CertificateDer::from(cert_der);
    let key = PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(key_der));
    
    Ok((cert, key))
}

pub fn create_server_config(
    cert: CertificateDer<'static>,
    key: PrivateKeyDer<'static>,
) -> Result<ServerConfig, Box<dyn Error>> {
    let mut config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(vec![cert], key)?;
    config.alpn_protocols = vec![b"remotelink".to_vec()];
    Ok(config)
}
