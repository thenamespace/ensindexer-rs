mod app;
mod compare;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    app::run().await
}
