fn main() {
	pub mod example {
		#[doc = r" Execute the desired query."]
		#[cfg(feature = "query")]
		pub async fn query(
			client: &edgedb_tokio::Client,
		) -> core::result::Result<Output, edgedb_errors::Error> {
			client
				.query_required_single("select <datetime>'1999-03-31T15:17:00Z'", &())
				.await
		}
		#[doc = r" Compose the query as part of a larger transaction."]
		#[cfg(feature = "query")]
		pub async fn transaction(
			conn: &mut edgedb_tokio::Transaction,
		) -> core::result::Result<Output, edgedb_errors::Error> {
			conn.query_required_single("select <datetime>'1999-03-31T15:17:00Z'", &())
				.await
		}
		pub type Input = ();
		pub type Output = edgedb_protocol::model::Datetime;
	}
}
