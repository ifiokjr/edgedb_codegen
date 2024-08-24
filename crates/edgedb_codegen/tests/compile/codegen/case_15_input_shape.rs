fn main() {
	pub mod example {
		use ::edgedb_codegen::exports as e;
		#[doc = r" Execute the desired query."]
		#[cfg(feature = "query")]
		pub async fn query(
			client: &e::edgedb_tokio::Client,
		) -> core::result::Result<Output, e::edgedb_errors::Error> {
			client.query_required_single(QUERY, &()).await
		}
		#[doc = r" Compose the query as part of a larger transaction."]
		#[cfg(feature = "query")]
		pub async fn transaction(
			conn: &mut e::edgedb_tokio::Transaction,
		) -> core::result::Result<Output, e::edgedb_errors::Error> {
			conn.query_required_single(QUERY, &()).await
		}
		pub type Input = ();
		#[derive(Clone, Debug, e :: typed_builder :: TypedBuilder)]
		#[cfg_attr(feature = "serde", derive(e::serde::Serialize, e::serde::Deserialize))]
		#[cfg_attr(feature = "query", derive(e::edgedb_derive::Queryable))]
		pub struct Output {
			#[builder(setter(into))]
			pub my_string: String,
			#[builder(setter(into))]
			pub my_number: i64,
			pub several_numbers: Vec<i64>,
			#[builder(setter(into))]
			pub array: Vec<i64>,
		}
		#[doc = r" The original query string provided to the macro. Can be reused in your codebase."]
		pub const QUERY: &str = "select { my_string := RelationshipType.Follow, my_number := 42, \
		                         several_numbers := {1, 2, 3}, array := [1, 2, 3] };";
	}
}
