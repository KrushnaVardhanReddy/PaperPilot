use std::env;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let port: u16 = env::var("PORT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(7823);

    let bind = env::var("BIND").unwrap_or_else(|_| "127.0.0.1".to_string());

    paperpilot_gateway::server::start(port, &bind, std::future::pending()).await
}
