fn main() {
	pub mod example {
		#[doc = r" Execute the desired query."]
		#[cfg(feature = "query")]
		pub async fn query(
			client: &edgedb_tokio::Client,
		) -> core::result::Result<Vec<Output>, edgedb_errors::Error> {
			client
				.query("select {\"set\", \"of\", \"strings\"}", &())
				.await
		}
		#[doc = r" Compose the query as part of a larger transaction."]
		#[cfg(feature = "query")]
		pub async fn transaction(
			conn: &mut edgedb_tokio::Transaction,
		) -> core::result::Result<Vec<Output>, edgedb_errors::Error> {
			conn.query("select {\"set\", \"of\", \"strings\"}", &())
				.await
		}
		pub type Input = ();
		pub type Output = String;
	}
}
