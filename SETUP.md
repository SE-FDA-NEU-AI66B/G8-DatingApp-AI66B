## Setup

```bash
git clone <this-repo-url>

```

### Install rust

check https://rust-lang.org/tools/install/
install nightly toolschain
and set it as defalut

```bash
rustup toolchain help
rustup show active-toolchain|python -c"print(input().split()[0])"
```

- check that both .cargo/bin and .rustup/<the above result>/bin is in $PATH

### Install tools

```bash
rustup component add clippy rustc-codegen-cranelift rust-analyzer rust-std-wasm32-unknown-unknown
```

```bash
cargo install cargo-generate cargo-leptos leptosfmt mise
```

or

```bash
cargo install cargo-binstall #for pc with limited space
cargo binstall cargo-generate cargo-leptos leptosfmt mise
```

### Setup Editor

https://book.leptos.dev/getting_started/leptos_dx.html

### Setup Cloudflare tunnel

## Install cloudflared

|on windows:| https://dash.cloudflare.com/tunnels|
| ---------------------------- | ---------------- | ---- |

| os       | method                                  |
| -------- | --------------------------------------- |
| windows: | https://dash.cloudflare.com/tunnels     |
| linux:   | use urown package manager "cloudflared" |

## open tunnel

```bash
cloudflared tunnel --config ./cert/.cloudflared/config.yml run
```
