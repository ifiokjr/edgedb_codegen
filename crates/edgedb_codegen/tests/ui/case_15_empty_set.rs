fn main() {
	pub mod example {
		#[doc = r" Execute the desired query."]
		#[cfg(feature = "query")]
		pub async fn query(
			client: &edgedb_tokio::Client,
		) -> core::result::Result<Option<Output>, edgedb_errors::Error> {
			client.query_single("select <int64>{}", &()).await
		}
		#[doc = r" Compose the query as part of a larger transaction."]
		#[cfg(feature = "query")]
		pub async fn transaction(
			conn: &mut edgedb_tokio::Transaction,
		) -> core::result::Result<Option<Output>, edgedb_errors::Error> {
			conn.query_single("select <int64>{}", &()).await
		}
		pub type Input = ();
		pub type Output = i64;
	}
}
