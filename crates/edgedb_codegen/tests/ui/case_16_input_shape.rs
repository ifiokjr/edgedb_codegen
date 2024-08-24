fn main() {
	pub mod example {
		#[doc = r" Execute the desired query."]
		#[cfg(feature = "query")]
		pub async fn query(
			client: &edgedb_tokio::Client,
		) -> core::result::Result<Output, edgedb_errors::Error> {
			client
				.query_required_single(
					"select { my_string := RelationshipType.Follow, my_number := 42, \
					 several_numbers := {1, 2, 3}, array := [1, 2, 3] };",
					&(),
				)
				.await
		}
		#[doc = r" Compose the query as part of a larger transaction."]
		#[cfg(feature = "query")]
		pub async fn transaction(
			conn: &mut edgedb_tokio::Transaction,
		) -> core::result::Result<Output, edgedb_errors::Error> {
			conn.query_required_single(
				"select { my_string := RelationshipType.Follow, my_number := 42, several_numbers \
				 := {1, 2, 3}, array := [1, 2, 3] };",
				&(),
			)
			.await
		}
		pub type Input = ();
		#[derive(Clone, Debug)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
		#[cfg_attr(feature = "query", derive(edgedb_derive::Queryable))]
		#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
		pub struct Output {
			#[cfg_attr(feature = "builder", builder(setter(into)))]
			pub my_string: String,
			#[cfg_attr(feature = "builder", builder(setter(into)))]
			pub my_number: i64,
			pub several_numbers: Vec<i64>,
			#[cfg_attr(feature = "builder", builder(setter(into)))]
			pub array: Vec<i64>,
		}
	}
}
