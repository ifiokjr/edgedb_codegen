fn main() {
    pub mod example {
        use ::edgedb_codegen::exports as e;
        /// Execute the desired query.
        #[cfg(feature = "query")]
        pub async fn query(
            client: &e::edgedb_tokio::Client,
            props: &Input,
        ) -> core::result::Result<Output, e::edgedb_errors::Error> {
            client.query_required_single(QUERY, props).await
        }
        /// Compose the query as part of a larger transaction.
        #[cfg(feature = "query")]
        pub async fn transaction(
            conn: &mut e::edgedb_tokio::Transaction,
            props: &Input,
        ) -> core::result::Result<Output, e::edgedb_errors::Error> {
            conn.query_required_single(QUERY, props).await
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
            pub custom: String,
        }
        impl e::edgedb_protocol::query_arg::QueryArgs for Input {
            fn encode(
                &self,
                encoder: &mut e::edgedb_protocol::query_arg::Encoder,
            ) -> core::result::Result<(), e::edgedb_errors::Error> {
                let map = e::edgedb_protocol::named_args! {
                    "custom" => self.custom.clone(),
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
        pub struct Output {
            pub hello: String,
            pub custom: String,
        }
        /// The original query string provided to the macro. Can be reused in your codebase.
        pub const QUERY: &str = "select { hello := \"world\", custom := <str>$custom }";
    }
}
