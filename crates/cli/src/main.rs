mod app;
mod compare;
mod schema;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    app::run().await
}
