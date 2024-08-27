fn main() {
    pub mod example {
        use ::edgedb_codegen::exports as e;
        /// Execute the desired query.
        #[cfg(feature = "query")]
        pub async fn query(
            client: &e::edgedb_tokio::Client,
        ) -> core::result::Result<Output, e::edgedb_errors::Error> {
            client.query_required_single(QUERY, &()).await
        }
        /// Compose the query as part of a larger transaction.
        #[cfg(feature = "query")]
        pub async fn transaction(
            conn: &mut e::edgedb_tokio::Transaction,
        ) -> core::result::Result<Output, e::edgedb_errors::Error> {
            conn.query_required_single(QUERY, &()).await
        }
        pub type Input = ();
        pub type Output = f64;
        /// The original query string provided to the macro. Can be reused in your codebase.
        pub const QUERY: &str = "select 314e-2";
    }
}
