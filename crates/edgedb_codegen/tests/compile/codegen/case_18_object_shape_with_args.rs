fn main() {
    pub mod example {
        use ::edgedb_codegen::exports as e;
        /// Execute the desired query.
        #[cfg(feature = "query")]
        pub async fn query(
            client: &e::edgedb_tokio::Client,
            props: &Input,
        ) -> core::result::Result<Vec<Output>, e::edgedb_errors::Error> {
            client.query(QUERY, props).await
        }
        /// Compose the query as part of a larger transaction.
        #[cfg(feature = "query")]
        pub async fn transaction(
            conn: &mut e::edgedb_tokio::Transaction,
            props: &Input,
        ) -> core::result::Result<Vec<Output>, e::edgedb_errors::Error> {
            conn.query(QUERY, props).await
        }
        #[derive(Clone, Debug)]
        #[cfg_attr(feature = "builder", derive(e::typed_builder::TypedBuilder))]
        #[cfg_attr(feature = "query", derive(e::edgedb_derive::Queryable))]
        #[cfg_attr(
            feature = "serde",
            derive(e::serde::Serialize, e::serde::Deserialize)
        )]
        pub struct Input {
            #[cfg_attr(feature = "builder", builder(setter(into)))]
            pub starts_with: String,
            #[cfg_attr(feature = "builder", builder(setter(into)))]
            pub ends_with: String,
        }
        impl e::edgedb_protocol::query_arg::QueryArgs for Input {
            fn encode(
                &self,
                encoder: &mut e::edgedb_protocol::query_arg::Encoder,
            ) -> core::result::Result<(), e::edgedb_errors::Error> {
                let map = e::edgedb_protocol::named_args! {
                    "starts_with" => self.starts_with.clone(), "ends_with" => self
                    .ends_with.clone(),
                };
                map.encode(encoder)
            }
        }
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
        pub const QUERY: &str = "select Team {**} filter .name like <str>$starts_with ++ '%' and .description like '%' ++ <str>$ends_with;";
    }
}
