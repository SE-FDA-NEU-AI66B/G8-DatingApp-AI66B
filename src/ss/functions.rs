pub fn get_db_uri() -> String {
    std::env::var("DATABASE_URL").expect("DATABASE_URL must point to PostgreSQL")
}

pub fn get_tls_config() -> rustls::ServerConfig {
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .unwrap();

    use std::{fs::File, io::BufReader};

    let cert_path = std::env::var("TLS_CERT_PATH")
        .expect("TLS_CERT_PATH must be set when TLS is enabled");
    let key_path = std::env::var("TLS_KEY_PATH")
        .expect("TLS_KEY_PATH must be set when TLS is enabled");
    let mut certs_file = BufReader::new(File::open(cert_path).expect("cannot open TLS certificate"));
    let mut key_file = BufReader::new(File::open(key_path).expect("cannot open TLS private key"));
    let tls_certs = rustls_pemfile::certs(&mut certs_file)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let tls_key = rustls_pemfile::pkcs8_private_keys(&mut key_file)
        .next()
        .unwrap()
        .unwrap();

    rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(tls_certs, rustls::pki_types::PrivateKeyDer::Pkcs8(tls_key))
        .unwrap()
}
