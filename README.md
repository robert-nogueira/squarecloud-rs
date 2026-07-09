# squarecloud-rs

[![Crates.io](https://img.shields.io/crates/v/squarecloud.svg)](https://crates.io/crates/squarecloud)
[![Docs.rs](https://docs.rs/squarecloud/badge.svg)](https://docs.rs/squarecloud)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![CI](https://github.com/robert-nogueira/squarecloud-rs/actions/workflows/general.yaml/badge.svg)](https://github.com/robert-nogueira/squarecloud-rs/actions/workflows/general.yaml)

A lightweight, idiomatic, async Rust client for the [SquareCloud](https://squarecloud.app) API. Deploy applications, manage files and environment variables, provision databases, take snapshots, and organize workspaces, with compile-time type safety and zero-cost async through Tokio.

## Installation

```toml
[dependencies]
squarecloud = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Configuration

The client reads `API_TOKEN` on first use, either from the process environment or a `.env` file in the working directory.

```env
API_TOKEN=your_squarecloud_api_token
```

## Quick start

```rust
use squarecloud::ApiClient;

#[tokio::main]
async fn main() {
    let client = ApiClient::new();

    let me = client.me().await.unwrap();
    println!("Logged in as {} ({})", me.user.name, me.user.email);

    let status = client.app("application_id").status().await.unwrap();
    println!("CPU: {}  RAM: {}", status.cpu, status.ram);
}
```

## Realtime log streaming

`AppResource::realtime()` returns a live SSE stream of log and system events:

```rust
use futures_util::StreamExt;
use squarecloud::{ApiClient, RealtimeEvent};

#[tokio::main]
async fn main() {
    let client = ApiClient::new();
    let mut stream = client.app("application_id").realtime();
    while let Some(event) = stream.next().await {
        match event.unwrap() {
            RealtimeEvent::Log(msg) => println!("[log]    {msg}"),
            RealtimeEvent::System(msg) => println!("[system] {msg}"),
        }
    }
}
```

## Examples

The [`examples/`](examples/) directory contains one file per method, organized by resource:

| Folder | Covers |
|---|---|
| [`apps/`](examples/apps/) | info, status, logs, realtime stream, metrics, start/stop/restart, commit, delete, network, domain |
| [`env/`](examples/env/) | list, upsert, overwrite, delete |
| [`files/`](examples/files/) | list, read, write, move, delete |
| [`snapshots/`](examples/snapshots/) | create, list, restore |
| [`deployments/`](examples/deployments/) | current deploy, history, webhook integration |
| [`database/`](examples/database/) | full lifecycle: create, status, metrics, credentials, snapshots, delete |
| [`workspaces/`](examples/workspaces/) | create, members, permissions, leave, delete |

```sh
cargo run --example me
cargo run --example app_status -- <app_id>
cargo run --example app_realtime -- <app_id>
cargo run --example database_info -- <db_id>
```

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

MIT
