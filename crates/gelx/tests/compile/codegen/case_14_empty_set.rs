pub mod example {
    use ::gelx::exports as __g;
    /// Execute the desired query.
    pub async fn query(
        client: impl __g::gel_tokio::QueryExecutor,
    ) -> ::core::result::Result<Option<Output>, __g::gel_errors::Error> {
        client.query_single(QUERY, &()).await
    }
    pub type Input = ();
    pub type Output = i64;
    /// The original query string provided to the macro. Can be reused in your codebase.
    pub const QUERY: &str = "select <int64>{}";
}
fn main() {}
