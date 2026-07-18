use squarecloud::{ApiError, Client, errors::AppErrorCode};

async fn ensure_running(
    client: &Client,
    app_id: &str,
) -> Result<(), ApiError> {
    let app = client.app(app_id);
    let envs = app.list_envs().await?;
    println!("{} env vars set", envs.len());
    app.start().await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let client =
        Client::new(std::env::var("API_TOKEN").expect("set API_TOKEN"));
    let app_id = std::env::args()
        .nth(1)
        .expect("usage: cargo run --example error_handling -- <app_id>");

    match client.app(&app_id).start().await {
        Ok(_) => println!("app started"),
        Err(ApiError::Transport(e)) => eprintln!("network problem: {e}"),
        Err(ApiError::Service { code }) => match code {
            AppErrorCode::AppNotFound => eprintln!("no such app: {app_id}"),
            AppErrorCode::ContainerAlreadyStarted => {
                println!("already running")
            }
            AppErrorCode::Unknown(raw) => {
                eprintln!("uncatalogued API code: {raw}")
            }
            other => eprintln!("start rejected: {other:?}"),
        },
    }

    if let Err(ApiError::Service { code }) =
        ensure_running(&client, &app_id).await
    {
        if code == AppErrorCode::AppNotFound {
            eprintln!("composed flow failed: app is gone");
        } else {
            eprintln!("composed flow failed with code {code}");
        }
    }
}
