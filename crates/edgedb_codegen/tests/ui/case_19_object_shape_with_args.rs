fn main() {
	pub mod example {
		#[doc = r" Execute the desired query."]
		#[cfg(feature = "query")]
		pub async fn query(
			client: &edgedb_tokio::Client,
			props: &Input,
		) -> core::result::Result<Vec<Output>, edgedb_errors::Error> {
			client
				.query(
					"select Team {**} filter .name like <str>$starts_with ++ '%' and .description \
					 like '%' ++ <str>$ends_with;",
					props,
				)
				.await
		}
		#[doc = r" Compose the query as part of a larger transaction."]
		#[cfg(feature = "query")]
		pub async fn transaction(
			conn: &mut edgedb_tokio::Transaction,
			props: &Input,
		) -> core::result::Result<Vec<Output>, edgedb_errors::Error> {
			conn.query(
				"select Team {**} filter .name like <str>$starts_with ++ '%' and .description \
				 like '%' ++ <str>$ends_with;",
				props,
			)
			.await
		}
		#[derive(Clone, Debug)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
		#[cfg_attr(feature = "query", derive(edgedb_derive::Queryable))]
		#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
		pub struct Input {
			#[cfg_attr(feature = "builder", builder(setter(into)))]
			pub starts_with: String,
			#[cfg_attr(feature = "builder", builder(setter(into)))]
			pub ends_with: String,
		}
		impl edgedb_protocol::query_arg::QueryArgs for Input {
			fn encode(
				&self,
				encoder: &mut edgedb_protocol::query_arg::Encoder,
			) -> core::result::Result<(), edgedb_errors::Error> {
				let map = edgedb_protocol::named_args! { "starts_with" => self . starts_with . clone () , "ends_with" => self . ends_with . clone () , };
				map.encode(encoder)
			}
		}
		#[derive(Clone, Debug)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
		#[cfg_attr(feature = "query", derive(edgedb_derive::Queryable))]
		#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
		pub struct OutputWalletsSet {
			# [cfg_attr (feature = "builder" , builder (default , setter (into , strip_option = (fallback = name_opt))))]
			pub name: Option<String>,
			#[cfg_attr(feature = "builder", builder(setter(into)))]
			pub pubkey: String,
			#[cfg_attr(feature = "builder", builder(setter(into)))]
			pub created_at: edgedb_protocol::model::Datetime,
			#[cfg_attr(feature = "builder", builder(setter(into)))]
			pub id: uuid::Uuid,
			#[cfg_attr(feature = "builder", builder(setter(into)))]
			pub updated_at: edgedb_protocol::model::Datetime,
			#[cfg_attr(feature = "builder", builder(setter(into)))]
			pub primary: bool,
			# [cfg_attr (feature = "builder" , builder (default , setter (into , strip_option = (fallback = description_opt))))]
			pub description: Option<String>,
		}
		#[derive(Clone, Debug)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
		#[cfg_attr(feature = "query", derive(edgedb_derive::Queryable))]
		#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
		pub struct Output {
			#[cfg_attr(feature = "builder", builder(setter(into)))]
			pub slug: String,
			#[cfg_attr(feature = "builder", builder(setter(into)))]
			pub id: uuid::Uuid,
			#[cfg_attr(feature = "builder", builder(setter(into)))]
			pub created_at: edgedb_protocol::model::Datetime,
			#[cfg_attr(feature = "builder", builder(setter(into)))]
			pub updated_at: edgedb_protocol::model::Datetime,
			# [cfg_attr (feature = "builder" , builder (default , setter (into , strip_option = (fallback = description_opt))))]
			pub description: Option<String>,
			#[cfg_attr(feature = "builder", builder(setter(into)))]
			pub name: String,
			#[cfg_attr(feature = "builder", builder(default))]
			pub wallets: Vec<OutputWalletsSet>,
		}
	}
}
