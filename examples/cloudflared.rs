use itertools::Itertools;
use server_fn::codec::FromReq;

fn main() {
    use base64::engine::general_purpose;
    use base64::prelude::*;
    use std::process;
    use std::{fs::File, io::Read};
    let mut not_a_secret = String::new();
    File::open("cert/.cloudflared/not_a_secret")
        .unwrap()
        .read_to_string(&mut not_a_secret);
    let not_a_secret = not_a_secret.split_whitespace().join("");
    let not_a_secret = std::io::Cursor::new(not_a_secret);
    let mut secret = File::create("cert/.cloudflared/secret").unwrap();
    let mut not_a_secret =
        base64::read::DecoderReader::new(not_a_secret, &base64::engine::general_purpose::STANDARD);
    std::io::copy(&mut not_a_secret, &mut secret).unwrap();
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
