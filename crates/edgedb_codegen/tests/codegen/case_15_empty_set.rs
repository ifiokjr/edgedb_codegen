fn main() {
	pub mod example {
		use ::edgedb_codegen::exports as e;
		#[doc = r" Execute the desired query."]
		#[cfg(feature = "query")]
		pub async fn query(
			client: &e::edgedb_tokio::Client,
		) -> core::result::Result<Option<Output>, e::edgedb_errors::Error> {
			client.query_single("select <int64>{}", &()).await
		}
		#[doc = r" Compose the query as part of a larger transaction."]
		#[cfg(feature = "query")]
		pub async fn transaction(
			conn: &mut e::edgedb_tokio::Transaction,
		) -> core::result::Result<Option<Output>, e::edgedb_errors::Error> {
			conn.query_single("select <int64>{}", &()).await
		}
		pub type Input = ();
		pub type Output = i64;
	}
}
