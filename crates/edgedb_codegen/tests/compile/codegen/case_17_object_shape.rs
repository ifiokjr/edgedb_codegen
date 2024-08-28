fn main() {
    pub mod example {
        use ::edgedb_codegen::exports as e;
        /// Execute the desired query.
        #[cfg(feature = "query")]
        pub async fn query(
            client: &e::edgedb_tokio::Client,
        ) -> core::result::Result<Vec<Output>, e::edgedb_errors::Error> {
            client.query(QUERY, &()).await
        }
        /// Compose the query as part of a larger transaction.
        #[cfg(feature = "query")]
        pub async fn transaction(
            conn: &mut e::edgedb_tokio::Transaction,
        ) -> core::result::Result<Vec<Output>, e::edgedb_errors::Error> {
            conn.query(QUERY, &()).await
        }
        pub type Input = ();
        #[derive(Clone, Debug)]
        #[cfg_attr(feature = "query", derive(e::edgedb_derive::Queryable))]
        #[cfg_attr(
            feature = "serde",
            derive(e::serde::Serialize, e::serde::Deserialize)
        )]
        pub struct OutputWalletsSet {
            pub name: Option<String>,
            pub pubkey: String,
            pub created_at: e::chrono::DateTime<e::chrono::Utc>,
            pub id: e::uuid::Uuid,
            pub updated_at: e::chrono::DateTime<e::chrono::Utc>,
            pub primary: bool,
            pub description: Option<String>,
        }
        #[derive(Clone, Debug)]
        #[cfg_attr(feature = "query", derive(e::edgedb_derive::Queryable))]
        #[cfg_attr(
            feature = "serde",
            derive(e::serde::Serialize, e::serde::Deserialize)
        )]
        pub struct Output {
            pub slug: String,
            pub id: e::uuid::Uuid,
            pub created_at: e::chrono::DateTime<e::chrono::Utc>,
            pub updated_at: e::chrono::DateTime<e::chrono::Utc>,
            pub description: Option<String>,
            pub name: String,
            pub wallets: Vec<OutputWalletsSet>,
        }
        /// The original query string provided to the macro. Can be reused in your codebase.
        pub const QUERY: &str = "select Team {**}";
    }
}
