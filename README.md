# squarecloud-rs

[![Crates.io](https://img.shields.io/crates/v/squarecloud.svg)](https://crates.io/crates/squarecloud)
[![Docs.rs](https://docs.rs/squarecloud/badge.svg)](https://docs.rs/squarecloud)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![CI](https://github.com/robert-nogueira/squarecloud-rs/actions/workflows/general.yaml/badge.svg)](https://github.com/robert-nogueira/squarecloud-rs/actions/workflows/general.yaml)

A lightweight, idiomatic, async Rust client for the [SquareCloud](https://squarecloud.app) API. Deploy applications, manage files and environment variables, provision databases, take snapshots, organize workspaces, and store objects in Blob Storage, with compile-time type safety and zero-cost async through Tokio.

## Installation

```toml
[dependencies]
squarecloud = "0.2"
tokio = { version = "1", features = ["full"] }
```

## Configuration

`Client::new` takes your API token directly — the crate never reads the environment or any file on its own. Source the token however you like, e.g. from an environment variable:

```env
API_TOKEN=your_squarecloud_api_token
```

## Quick start

```rust
use squarecloud::Client;

#[tokio::main]
async fn main() {
    let client = Client::new(std::env::var("API_TOKEN").expect("API_TOKEN not set"));

    let me = client.me().await.unwrap();
    println!("Logged in as {} ({})", me.user.name, me.user.email);

    let status = client.app("application_id").status().await.unwrap();
    println!("CPU: {}  RAM: {}", status.cpu, status.ram);
}
```

Resource factories (`app`, `database`, `workspace`, `blob`) borrow the client and clone it internally; the underlying connection pool is shared, so clones are cheap and the client stays usable.

## Error handling

Every method returns `ApiError<C>`, where `C` is an error-code enum scoped to that group of routes: you only ever match codes your call can actually return. Codes the catalogue does not know yet are preserved in an `Unknown` variant with the raw wire string.

```rust
use squarecloud::{ApiError, Client, errors::AppErrorCode};

match client.app("application_id").start().await {
    Ok(_) => println!("app started"),
    Err(ApiError::Transport(e)) => eprintln!("network problem: {e}"),
    Err(ApiError::Service { code }) => match code {
        AppErrorCode::AppNotFound => eprintln!("no such app"),
        AppErrorCode::Unknown(raw) => eprintln!("uncatalogued code: {raw}"),
        other => eprintln!("start rejected: {other:?}"),
    },
}
```

Functions that compose calls across domains can return the type-erased default `ApiError` and propagate everything with `?`; the erased code still compares against typed variants and strings:

```rust
use squarecloud::{ApiError, Client, errors::AppErrorCode};

async fn ensure_running(client: &Client, id: &str) -> Result<(), ApiError> {
    let app = client.app(id);
    app.list_envs().await?; // ApiError<EnvErrorCode>
    app.start().await?;     // ApiError<AppErrorCode>
    Ok(())
}
```

The catalogue is kept in sync with the live OpenAPI spec by a contract test, so upstream code renames surface as test failures instead of silent `Unknown`s.

## Realtime log streaming

`AppResource::realtime()` returns a live SSE stream of log and system events:

```rust
use futures_util::StreamExt;
use squarecloud::{Client, RealtimeEvent};

#[tokio::main]
async fn main() {
    let client = Client::new(std::env::var("API_TOKEN").expect("API_TOKEN not set"));
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
| [`apps/`](examples/apps/) | info, status, logs, realtime stream, metrics, start/stop/restart, commit, delete, network, domains, load balancers |
| [`env/`](examples/env/) | list, upsert, overwrite, delete |
| [`files/`](examples/files/) | list, read, write, move, delete |
| [`snapshots/`](examples/snapshots/) | create, list, restore |
| [`deployments/`](examples/deployments/) | current deploy, history, webhook integration |
| [`database/`](examples/database/) | full lifecycle: create, status, metrics, credentials, snapshots, delete |
| [`workspaces/`](examples/workspaces/) | create, members, permissions, leave, delete |
| [`blob/`](examples/blob/) | upload, list, delete, account stats |

```sh
cargo run --example me
cargo run --example error_handling -- <app_id>
cargo run --example app_status -- <app_id>
cargo run --example app_realtime -- <app_id>
cargo run --example database_info -- <db_id>
```

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

MIT
