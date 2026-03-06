#[tokio::main]
async fn main() -> anyhow::Result<()> {
    website_app::run_service().await
}
