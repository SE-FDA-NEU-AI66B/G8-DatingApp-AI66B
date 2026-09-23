## Setup

```bash
git clone https://github.com/SE-FDA-NEU-AI66B/G8-DatingApp-AI66B.git
cd G8-DatingApp-AI66B
```

### Install Rust

Check https://rust-lang.org/tools/install

Install nightly toolchain and set it as default:

```bash
rustup toolchain install nightly
rustup default nightly
```

Verify:

```bash
rustup show active-toolchain
```

- Check that both `.cargo/bin` and `.rustup/<toolchain>/bin` are in `$PATH`.

### Install tools

```bash
rustup target add wasm32-unknown-unknown
rustup component add clippy rust-analyzer
```

```bash
cargo install cargo-generate cargo-leptos leptosfmt mise
```

Or, for PCs with limited space:

```bash
cargo install cargo-binstall
cargo binstall cargo-generate cargo-leptos leptosfmt mise
```

#### Other tools

- [uv](https://docs.astral.sh/uv/) — Python package manager

### Setup Editor

https://book.leptos.dev/getting_started/leptos_dx.html

### Setup Cloudflare tunnel

#### Install cloudflared

| OS      | Method                                      |
| ------- | ------------------------------------------- |
| Windows | https://dash.cloudflare.com/tunnels         |
| Linux   | Use your package manager: `cloudflared`     |

#### Open tunnel

```bash
cloudflared tunnel --config ./cert/.cloudflared/config.yml run
```

## Misc

### Create Python env

```bash
uv add -r requirements.txt
```
