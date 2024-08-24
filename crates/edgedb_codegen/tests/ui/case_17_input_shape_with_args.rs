fn main() {
	pub mod example {
		#[doc = r" Execute the desired query."]
		#[cfg(feature = "query")]
		pub async fn query(
			client: &edgedb_tokio::Client,
			props: &Input,
		) -> core::result::Result<Output, edgedb_errors::Error> {
			client
				.query_required_single(
					"select {hello := \"world\", custom := <str>$custom }",
					props,
				)
				.await
		}
		#[doc = r" Compose the query as part of a larger transaction."]
		#[cfg(feature = "query")]
		pub async fn transaction(
			conn: &mut edgedb_tokio::Transaction,
			props: &Input,
		) -> core::result::Result<Output, edgedb_errors::Error> {
			conn.query_required_single(
				"select {hello := \"world\", custom := <str>$custom }",
				props,
			)
			.await
		}
		#[derive(Clone, Debug)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
		#[cfg_attr(feature = "query", derive(edgedb_derive::Queryable))]
		#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
		pub struct Input {
			#[cfg_attr(feature = "builder", builder(setter(into)))]
			pub custom: String,
		}
		impl edgedb_protocol::query_arg::QueryArgs for Input {
			fn encode(
				&self,
				encoder: &mut edgedb_protocol::query_arg::Encoder,
			) -> core::result::Result<(), edgedb_errors::Error> {
				let map = edgedb_protocol::named_args! { "custom" => self . custom . clone () , };
				map.encode(encoder)
			}
		}
		#[derive(Clone, Debug)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
		#[cfg_attr(feature = "query", derive(edgedb_derive::Queryable))]
		#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
		pub struct Output {
			#[cfg_attr(feature = "builder", builder(setter(into)))]
			pub hello: String,
			#[cfg_attr(feature = "builder", builder(setter(into)))]
			pub custom: String,
		}
	}
}
