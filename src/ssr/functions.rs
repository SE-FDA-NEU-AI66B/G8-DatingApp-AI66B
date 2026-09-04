pub fn get_tls_config() -> rustls::ServerConfig {
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .unwrap();
    use std::{fs::File, io::BufReader};

    let mut certs_file = BufReader::new(File::open("cert/rsa_4096.crt").unwrap());
    let mut key_file = BufReader::new(File::open("cert/rsa_4096.key").unwrap());
    let mut certs_file =
        BufReader::new(File::open("cert/.lego/certificates/rsa_4096.pem").unwrap());
    let mut key_file = BufReader::new(File::open("cert/.lego/certificates/rsa_4096.pem").unwrap());
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
