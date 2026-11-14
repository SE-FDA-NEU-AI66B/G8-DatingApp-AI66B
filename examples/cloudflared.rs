use itertools::Itertools;
// use server_fn::codec::FromReq;

fn main() {
    use base64::engine::general_purpose;
    use base64::prelude::*;
    use std::process;
    use std::{fs::File, io::Read};
    if let Ok(not_a_secret) = File::open("cert/.cloudflared/not_a_secret") {
        let mut secret = File::create("cert/.cloudflared/secret").unwrap();
        let mut not_a_secret = base64::read::DecoderReader::new(
            not_a_secret,
            &base64::engine::general_purpose::STANDARD,
        );
        std::io::copy(&mut not_a_secret, &mut secret).unwrap();
    }
    {
        let not_a_secret = File::create("cert/.cloudflared/not_a_secret").unwrap();
        let mut not_a_secret = base64::write::EncoderWriter::new(
            not_a_secret,
            &base64::engine::general_purpose::STANDARD,
        );
        let mut secret = File::open("cert/.cloudflared/secret").unwrap();
        std::io::copy(&mut secret, &mut not_a_secret).unwrap();
    }
    // cloudflared = "cloudflared "

    // #[cfg(windows)]
    let mut command = process::Command::new("cloudflared");
    command.args([
        "tunnel",
        "--config",
        "./cert/.cloudflared/config.yml",
        "run",
    ]);

    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        command.exec();
    }
    #[cfg(not(unix))]
    {
        command.stdin(process::Stdio::inherit());
        command.stdout(process::Stdio::inherit());
        command.spawn();
    }
}
