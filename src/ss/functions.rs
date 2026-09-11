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
pub fn get_db_uri() -> String {
    use std::env;
    format!(
        "postgres://postgres:{}@localhost/userdb",
        env::var_os("PGPASS")
            .map(|i| i.into_string().unwrap())
            .unwrap_or("".to_string())
    )
}
#[allow(dead_code)]
pub async fn connect_database() -> Result<(Client, Connection<Socket, NoTlsStream>), Error> {
    use std::env;
    use tokio_postgres::{connect, NoTls};
    // tokio_postgres::;
    let s = format!(
        "host={} user=postgres password={} dbname=userdb",
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
#[allow(dead_code)]
pub async fn connect_database2() {}
extern crate test;
// #[cfg(test)]
#[allow(unused_imports)]
pub mod tests {
    use super::*;
    use itertools::Itertools;
    extern crate test;
    use test::Bencher;
    use tokio::runtime;
    pub fn database_speed(n: usize, m: usize) {
        // 583,250,930.10 dev
        // 403,411,090.60 release
        let rt = runtime::Builder::new_current_thread()
            .enable_all()
            .build_local(runtime::LocalOptions::default())
            .unwrap();
        rt.block_on(async {
            let mut v = Vec::new();
            let mut v2 = Vec::new();
            for _ in (0..n) {
                let (client, connection) = connect_database().await.unwrap();
                v2.push(rt.spawn_local(async move { if let Err(e) = connection.await {} }));
                v.push(rt.spawn_local(async move {
                    for _ in 0..m {
                        let a = client
                            .query("SELECT * FROM public.cookielogin Limit 5", &[])
                            .await;
                    }
                    std::time::Instant::now()
                }));
            }
            for i in v {
                let r = i.await;
            }
            for i in v2 {
                i.await.unwrap();
            }
        });
    }
    #[ignore]
    #[bench]
    pub fn bench_database_speed(b: &mut Bencher) {
        // 583,250,930.10 dev
        // 403,411,090.60 release
        let (n, m) = (10, 1000);
        println!("{:?}", (n * m));
        b.iter(|| database_speed(n, m));
    }
    use sqlx::Row;
    use std::rc::Rc;
    #[ignore]
    #[bench]
    fn database_speed2(b: &mut Bencher) {
        // 627,045,562.00 dev
        // 489,410,657.00 release
        let (n, m) = (2, 5000);
        println!("{:?}", (n * m));
        b.iter(|| {
            let rt = runtime::Builder::new_current_thread()
                .enable_all()
                .build_local(runtime::LocalOptions::default())
                .unwrap();
            rt.block_on(async {
                use sqlx::postgres::PgPoolOptions;
                let pool = Rc::new(
                    PgPoolOptions::new()
                        .max_connections(n)
                        .connect(&get_db_uri())
                        .await
                        .unwrap(),
                );
                let row = (0..n * m)
                    .map(|_| {
                        let pool = pool.clone();
                        rt.spawn_local(async move {
                            sqlx::query_as("SELECT * FROM public.cookielogin Limit 5")
                                .fetch_all(&*pool)
                                .await
                                .unwrap()
                        })
                    })
                    .collect_vec();
                for i in row {
                    let r: Vec<(Vec<u8>, String, Option<time::PlainDateTime>)> = i.await.unwrap();
                    // let r = &i.await.unwrap()[0];
                    // println!("{:?}", r[0]);
                }
            });
        });
    }
    // #[tokio::main]
    pub async fn database_speed3(n: usize, m: usize) {
        // 583,250,930.10 dev
        // 403,411,090.60 release
        let mut v = Vec::new();
        let mut v2 = Vec::new();
        for _ in (0..n) {
            let (client, connection) = connect_database().await.unwrap();
            v2.push(actix::spawn(
                async move { if let Err(e) = connection.await {} },
            ));
            v.push(actix::spawn(async move {
                for _ in 0..m {
                    let a = client
                        .query("SELECT * FROM public.cookielogin Limit 5", &[])
                        .await;
                }
                std::time::Instant::now()
            }));
        }
        for i in v {
            let r = i.await;
        }
        for i in v2 {
            i.await.unwrap();
        }
    }
    pub async fn database_speed4(n: usize, m: usize) {
        // 583,250,930.10 dev
        // 403,411,090.60 release
        let mut v = Vec::new();
        for _ in (0..n) {
            use clickhouse::Client;

            let client = Client::default()
                .with_url("http://localhost:8123")
                .with_user("username")
                .with_password("password")
                .with_database("default");
            v.push(actix::spawn(async move {
                for _ in 0..m {
                    use clickhouse::Row;
                    use serde::{Deserialize, Serialize};
                    use time::OffsetDateTime;

                    #[derive(Row, Serialize, Deserialize, Debug)]
                    struct MyRow {
                        #[serde(with = "serde_bytes")]
                        cookie: Vec<u8>,
                        userid: u64,
                        //
                        #[serde(with = "clickhouse::serde::time::datetime64::nanos")]
                        start: OffsetDateTime,
                    }
                    // cookie,
                    let a: Vec<MyRow> = client
                        .query("SELECT  cookie,userid,start  FROM cookielogin2 Limit 5")
                        .fetch_all()
                        .await
                        .unwrap();
                    // println!("{:?}", a);
                }
                std::time::Instant::now()
            }));
        }
        for i in v {
            let r = i.await;
        }
    }
}
