fn main() {
	pub mod example {
		use ::edgedb_codegen::exports as e;
		#[doc = r" Execute the desired query."]
		#[cfg(feature = "query")]
		pub async fn query(
			client: &e::edgedb_tokio::Client,
		) -> core::result::Result<Output, e::edgedb_errors::Error> {
			client
				.query_required_single(
					"select (fruit := 'Apple', quantity := 3.14, fresh := true)",
					&(),
				)
				.await
		}
		#[doc = r" Compose the query as part of a larger transaction."]
		#[cfg(feature = "query")]
		pub async fn transaction(
			conn: &mut e::edgedb_tokio::Transaction,
		) -> core::result::Result<Output, e::edgedb_errors::Error> {
			conn.query_required_single(
				"select (fruit := 'Apple', quantity := 3.14, fresh := true)",
				&(),
			)
			.await
		}
		pub type Input = ();
		#[derive(Clone, Debug)]
		#[cfg_attr(feature = "serde", derive(e::serde::Serialize, e::serde::Deserialize))]
		#[cfg_attr(feature = "query", derive(e::edgedb_derive::Queryable))]
		#[cfg_attr(feature = "builder", derive(e::typed_builder::TypedBuilder))]
		pub struct Output {
			pub fruit: String,
			pub quantity: f64,
			pub fresh: bool,
		}
	}
}
