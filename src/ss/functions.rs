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
use tokio_postgres::{tls::NoTlsStream, Client, Connection, Error, Socket};
#[allow(dead_code)]
pub async fn connect_database() -> Result<(Client, Connection<Socket, NoTlsStream>), Error> {
    use std::env;
    use tokio_postgres::{connect, NoTls};
    let s = format!(
        "host={} user=postgres password={}",
        "localhost",
        env::var_os("PGPASS")
            .map(|i| i.into_string().unwrap())
            .unwrap_or("".to_string())
    );
    // let (client, connection) = tokio_postgres::connect(&s, NoTls).await?;
    // println!("{:?}", client);
    // let connection = connection.await;
    // // connection;
    // println!("{:?}", connection);
    // // println!("{:?}", );
    // Ok(())
    connect(&s, NoTls).await
}
extern crate test;
#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use actix::prelude::*;
    extern crate test;
    use test::Bencher;
    struct MySyncActor;
    impl Actor for MySyncActor {
        type Context = SyncContext<Self>;
    }
    use actix::dev::MessageResponse;
    use std::time::{self, Duration, Instant};
    #[derive(Message)]
    #[rtype(result = "Instant")]
    struct Job(String);
    impl MessageResponse<MySyncActor, Job> for Instant {
        fn handle(
            self,
            ctx: &mut <MySyncActor as Actor>::Context,
            tx: Option<dev::OneshotSender<<Job as Message>::Result>>,
        ) {
            if let Some(tx) = tx {
                tx.send(self).unwrap();
            }
        }
    }
    impl Handler<Job> for MySyncActor {
        type Result = Instant;

        fn handle(&mut self, msg: Job, ctx: &mut Self::Context) -> Self::Result {
            Instant::now()
        }
    }

    #[bench]
    #[actix::main]
    async fn database_speed2(b: &mut Bencher) {
        use super::*;
        // b:
        let addr = SyncArbiter::start(2, || MySyncActor);
        println!("{:?}", addr.send(Job(String::from("afds"))).await);
        let (client, connection) = connect_database().await.unwrap();
        println!(
            "{:?}",
            client.query("SELECT $1::TEXT", &[&"whatsapp"]).await
        );
        // let n = test::black_box(1000);
    }
}
