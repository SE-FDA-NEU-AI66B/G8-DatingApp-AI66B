use std::ffi::OsString;

pub fn get_tls_config() -> rustls::ServerConfig {
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .unwrap();
    use std::{fs::File, io::BufReader};

    let mut certs_file =
        BufReader::new(File::open("cert/.lego/certificates/rsa_4096.pem").expect("wrong dir"));
    let mut key_file =
        BufReader::new(File::open("cert/.lego/certificates/rsa_4096.pem").expect("wrong dir"));
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
pub async fn connect_database() -> Result<(), tokio_postgres::error::Error> {
    use std::env;
    use tokio_postgres::{connect, NoTls};
    let s = format!(
        "host={} user=postgres password={}",
        "localhost",
        env::var_os("PGPASS")
            .map(|i| i.into_string().unwrap())
            .unwrap_or("".to_string())
    );
    let (client, connection) = tokio_postgres::connect(&s, NoTls).await?;
    println!("{:?}", client);
    let connection = connection.await;
    // connection;
    println!("{:?}", connection);
    // println!("{:?}", );
    Ok(())
}
