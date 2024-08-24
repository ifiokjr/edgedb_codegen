#![doc = document_features::document_features!()]

/// Generates a query module from a query string.
///
/// ```rust
/// use edgedb_codegen::edgedb_query;
///
/// edgedb_query!(get_users, "select User {**}");
/// ```
#[macro_export]
macro_rules! edgedb_query {
	($module:ident, $query:literal) => {
		$crate::exports::edgedb_codegen_macros::edgedb_query!($module, $query);
	};
}

/// Generates a query module from a query string relative to the root of the
/// crate this is defined in.
///
/// ```rust
/// use edgedb_codegen::edgedb_query_file;
///
/// edgedb_query_file!(get_users, "queries/get_users.edgeql");
/// ```
#[macro_export]
macro_rules! edgedb_query_file {
	($module:ident, $path:literal) => {
		$crate::exports::edgedb_codegen_macros::edgedb_query!($module, include_str!($path));
	};
}

pub mod exports {
	#[cfg(feature = "with_bigdecimal")]
	pub use bigdecimal;
	pub use bytes;
	#[cfg(feature = "with_chrono")]
	pub use chrono;
	pub use edgedb_codegen_macros;
	pub use edgedb_derive;
	pub use edgedb_errors;
	pub use edgedb_protocol;
	#[cfg(feature = "query")]
	pub use edgedb_tokio;
	#[cfg(any(feature = "with_bigdecimal", feature = "with_bigint"))]
	pub use num_bigint;
	#[cfg(any(feature = "with_bigdecimal", feature = "with_bigint"))]
	pub use num_traits;
	#[cfg(feature = "serde")]
	pub use serde;
	pub use typed_builder;
	pub use uuid;
}
