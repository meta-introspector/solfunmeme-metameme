mod main_composed;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    main_composed::main().await
}