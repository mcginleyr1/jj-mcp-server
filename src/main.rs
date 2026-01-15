mod tools;

use rmcp::{ServiceExt, transport::stdio};
use tools::JjService;
use tracing_subscriber::{self, EnvFilter};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::DEBUG.into()))
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    let service = JjService::new().serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}
