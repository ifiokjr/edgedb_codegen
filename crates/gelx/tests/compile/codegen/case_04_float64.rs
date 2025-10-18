pub mod example {
    use ::gelx::exports as __g;
    /// Execute the desired query.
    pub async fn query(
        client: impl __g::gel_tokio::QueryExecutor,
    ) -> ::core::result::Result<Output, __g::gel_errors::Error> {
        client.query_required_single(QUERY, &()).await
    }
    pub type Input = ();
    pub type Output = f64;
    /// The original query string provided to the macro. Can be reused in your codebase.
    pub const QUERY: &str = "select 314e-2";
}
fn main() {}
