fn main() {
	pub mod example {
		#[doc = r" Execute the desired query."]
		#[cfg(feature = "query")]
		pub async fn query(
			client: &edgedb_tokio::Client,
		) -> core::result::Result<Output, edgedb_errors::Error> {
			client
				.query_required_single("select <uuid>'a5ea6360-75bd-4c20-b69c-8f317b0d2857'", &())
				.await
		}
		#[doc = r" Compose the query as part of a larger transaction."]
		#[cfg(feature = "query")]
		pub async fn transaction(
			conn: &mut edgedb_tokio::Transaction,
		) -> core::result::Result<Output, edgedb_errors::Error> {
			conn.query_required_single("select <uuid>'a5ea6360-75bd-4c20-b69c-8f317b0d2857'", &())
				.await
		}
		pub type Input = ();
		pub type Output = uuid::Uuid;
	}
}
