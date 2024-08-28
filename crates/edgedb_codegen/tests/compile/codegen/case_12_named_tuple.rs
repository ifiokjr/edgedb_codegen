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
        #[derive(Clone, Debug)]
        #[cfg_attr(feature = "query", derive(e::edgedb_derive::Queryable))]
        #[cfg_attr(
            feature = "serde",
            derive(e::serde::Serialize, e::serde::Deserialize)
        )]
        pub struct Output {
            pub fruit: String,
            pub quantity: f64,
            pub fresh: bool,
        }
        /// The original query string provided to the macro. Can be reused in your codebase.
        pub const QUERY: &str = "select (fruit := 'Apple', quantity := 3.14, fresh := true)";
    }
}
