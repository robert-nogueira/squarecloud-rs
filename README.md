# squarecloud-rs

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A lightweight, idiomatic, async Rust client for the [SquareCloud](https://squarecloud.app) API. Deploy applications, manage files and environment variables, provision databases, take snapshots, and organize workspaces, with compile-time type safety and zero-cost async through Tokio.

## Installation

```toml
[dependencies]
squarecloud-rs = { git = "https://github.com/robert-nogueira/squarecloud-rs" }
tokio = { version = "1", features = ["full"] }
```

## Configuration

The client reads `API_TOKEN` on first use, either from the process environment or a `.env` file in the working directory.

## Quick start

```rust
use squarecloud_rs::ApiClient;

#[tokio::main]
async fn main() {
    let client = ApiClient::new();

    let me = client.me().await.unwrap();
    println!("Logged in as {} ({})", me.user.name, me.user.email);

    let status = client.app("application_id").status().await.unwrap();
    println!("CPU: {}  RAM: {}", status.cpu, status.ram);
}
```

## Examples

The [`examples/`](examples/) directory contains one file per method, organized by resource:

| Folder | Covers |
|---|---|
| [`apps/`](examples/apps/) | info, status, logs, metrics, start/stop/restart, commit, delete, network, domain |
| [`env/`](examples/env/) | list, upsert, overwrite, delete |
| [`files/`](examples/files/) | list, read, write, move, delete |
| [`snapshots/`](examples/snapshots/) | create, list, restore |
| [`deployments/`](examples/deployments/) | current deploy, history, webhook integration |
| [`database/`](examples/database/) | full lifecycle - create, status, metrics, credentials, snapshots, delete |
| [`workspaces/`](examples/workspaces/) | create, members, permissions, leave, delete |

```sh
cargo run --example me
cargo run --example app_status -- <app_id>
cargo run --example database_info -- <db_id>
```

## License

MIT
