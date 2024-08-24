fn main() {
	pub mod example {
		use ::edgedb_codegen::exports as e;
		#[doc = r" Execute the desired query."]
		#[cfg(feature = "query")]
		pub async fn query(
			client: &e::edgedb_tokio::Client,
			props: &Input,
		) -> core::result::Result<Vec<Output>, e::edgedb_errors::Error> {
			client.query(QUERY, props).await
		}
		#[doc = r" Compose the query as part of a larger transaction."]
		#[cfg(feature = "query")]
		pub async fn transaction(
			conn: &mut e::edgedb_tokio::Transaction,
			props: &Input,
		) -> core::result::Result<Vec<Output>, e::edgedb_errors::Error> {
			conn.query(QUERY, props).await
		}
		#[derive(Clone, Debug, e :: typed_builder :: TypedBuilder)]
		#[cfg_attr(feature = "serde", derive(e::serde::Serialize, e::serde::Deserialize))]
		#[cfg_attr(feature = "query", derive(e::edgedb_derive::Queryable))]
		pub struct Input {
			#[builder(setter(into))]
			pub starts_with: String,
			#[builder(setter(into))]
			pub ends_with: String,
		}
		impl e::edgedb_protocol::query_arg::QueryArgs for Input {
			fn encode(
				&self,
				encoder: &mut e::edgedb_protocol::query_arg::Encoder,
			) -> core::result::Result<(), e::edgedb_errors::Error> {
				let map = e::edgedb_protocol::named_args! { "starts_with" => self . starts_with . clone () , "ends_with" => self . ends_with . clone () , };
				map.encode(encoder)
			}
		}
		#[derive(Clone, Debug, e :: typed_builder :: TypedBuilder)]
		#[cfg_attr(feature = "serde", derive(e::serde::Serialize, e::serde::Deserialize))]
		#[cfg_attr(feature = "query", derive(e::edgedb_derive::Queryable))]
		pub struct OutputWalletsSet {
			# [builder (default , setter (into , strip_option (fallback = name_opt)))]
			pub name: Option<String>,
			#[builder(setter(into))]
			pub pubkey: String,
			#[builder(setter(into))]
			pub created_at: e::chrono::DateTime<e::chrono::Utc>,
			#[builder(setter(into))]
			pub id: e::uuid::Uuid,
			#[builder(setter(into))]
			pub updated_at: e::chrono::DateTime<e::chrono::Utc>,
			#[builder(setter(into))]
			pub primary: bool,
			# [builder (default , setter (into , strip_option (fallback = description_opt)))]
			pub description: Option<String>,
		}
		#[derive(Clone, Debug, e :: typed_builder :: TypedBuilder)]
		#[cfg_attr(feature = "serde", derive(e::serde::Serialize, e::serde::Deserialize))]
		#[cfg_attr(feature = "query", derive(e::edgedb_derive::Queryable))]
		pub struct Output {
			#[builder(setter(into))]
			pub slug: String,
			#[builder(setter(into))]
			pub id: e::uuid::Uuid,
			#[builder(setter(into))]
			pub created_at: e::chrono::DateTime<e::chrono::Utc>,
			#[builder(setter(into))]
			pub updated_at: e::chrono::DateTime<e::chrono::Utc>,
			# [builder (default , setter (into , strip_option (fallback = description_opt)))]
			pub description: Option<String>,
			#[builder(setter(into))]
			pub name: String,
			#[builder(default)]
			pub wallets: Vec<OutputWalletsSet>,
		}
		#[doc = r" The original query string provided to the macro. Can be reused in your codebase."]
		pub const QUERY: &str = "select Team {**} filter .name like <str>$starts_with ++ '%' and \
		                         .description like '%' ++ <str>$ends_with;";
	}
}
