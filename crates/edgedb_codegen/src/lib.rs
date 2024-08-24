#![doc = document_features::document_features!()]

pub use edgedb_codegen_macros::*;

pub mod exports {
	pub use ::bytes;
	pub use ::edgedb_derive;
	pub use ::edgedb_errors;
	pub use ::edgedb_protocol;
	#[cfg(feature = "query")]
	pub use ::edgedb_tokio;
	#[cfg(feature = "serde")]
	pub use ::serde;
	#[cfg(feature = "builder")]
	pub use ::typed_builder;
	pub use ::uuid;
}
