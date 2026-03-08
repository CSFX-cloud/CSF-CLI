pub async fn run(watch: bool) -> Result<(), Box<dyn std::error::Error>> {
    crate::nodes::metrics::run(watch).await
}
