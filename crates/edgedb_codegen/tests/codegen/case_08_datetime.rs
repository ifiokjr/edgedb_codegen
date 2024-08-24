fn main() {
	pub mod example {
		use ::edgedb_codegen::exports as e;
		#[doc = r" Execute the desired query."]
		#[cfg(feature = "query")]
		pub async fn query(
			client: &e::edgedb_tokio::Client,
		) -> core::result::Result<Output, e::edgedb_errors::Error> {
			client
				.query_required_single("select <datetime>'1999-03-31T15:17:00Z'", &())
				.await
		}
		#[doc = r" Compose the query as part of a larger transaction."]
		#[cfg(feature = "query")]
		pub async fn transaction(
			conn: &mut e::edgedb_tokio::Transaction,
		) -> core::result::Result<Output, e::edgedb_errors::Error> {
			conn.query_required_single("select <datetime>'1999-03-31T15:17:00Z'", &())
				.await
		}
		pub type Input = ();
		pub type Output = e::edgedb_protocol::model::Datetime;
	}
}
