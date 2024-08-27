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
        #[derive(Clone, Debug, e::typed_builder::TypedBuilder)]
        #[cfg_attr(
            feature = "serde",
            derive(e::serde::Serialize, e::serde::Deserialize)
        )]
        #[cfg_attr(feature = "query", derive(e::edgedb_derive::Queryable))]
        pub struct OutputWalletsSet {
            #[builder(default, setter(into, strip_option(fallback = name_opt)))]
            pub name: Option<String>,
            #[builder(setter(into))]
            pub pubkey: String,
            #[builder(setter(into))]
            pub created_at: e::chrono::DateTime<e::chrono::Utc>,
            #[builder(setter(into))]
            pub id: e::uuid::Uuid,
            #[builder(setter(into))]
            pub updated_at: e::chrono::DateTime<e::chrono::Utc>,
            #[builder(setter(into))]
            pub primary: bool,
            #[builder(default, setter(into, strip_option(fallback = description_opt)))]
            pub description: Option<String>,
        }
        #[derive(Clone, Debug, e::typed_builder::TypedBuilder)]
        #[cfg_attr(
            feature = "serde",
            derive(e::serde::Serialize, e::serde::Deserialize)
        )]
        #[cfg_attr(feature = "query", derive(e::edgedb_derive::Queryable))]
        pub struct Output {
            #[builder(setter(into))]
            pub slug: String,
            #[builder(setter(into))]
            pub id: e::uuid::Uuid,
            #[builder(setter(into))]
            pub created_at: e::chrono::DateTime<e::chrono::Utc>,
            #[builder(setter(into))]
            pub updated_at: e::chrono::DateTime<e::chrono::Utc>,
            #[builder(default, setter(into, strip_option(fallback = description_opt)))]
            pub description: Option<String>,
            #[builder(setter(into))]
            pub name: String,
            #[builder(default)]
            pub wallets: Vec<OutputWalletsSet>,
        }
        /// The original query string provided to the macro. Can be reused in your codebase.
        pub const QUERY: &str = "select Team {**}";
    }
}
