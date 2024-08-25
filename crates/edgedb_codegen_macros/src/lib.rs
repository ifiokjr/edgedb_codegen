use edgedb_codegen_core::generate_rust_from_query;
use edgedb_codegen_core::get_descriptor_sync;
use edgedb_codegen_core::resolve_path;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::ToTokens;
use syn::parse::Parse;
use syn::parse_macro_input;
use syn::Token;

/// Generates a query module from a query string.
///
/// It supports inline code:
///
/// ```ignore
/// use edgedb_codegen_macros::edgedb_query_raw;
///
/// edgedb_query_raw!(get_users, query: "select User {**}");
/// ```
///
/// It also supports file-based queries:
///
/// ```ignore
/// use edgedb_codegen_macros::edgedb_query_raw;
///
/// edgedb_query_raw!(insert_user, file: "../edgedb_codegen/queries/insert_user.edgeql");
/// ```
#[proc_macro]
pub fn edgedb_query_raw(input: TokenStream) -> TokenStream {
	parse_macro_input!(input as EdgedbQueryInput)
		.to_token_stream()
		.into()
}

pub(crate) struct EdgedbQueryInput {
	pub(crate) module: syn::Ident,
	pub(crate) query: String,
}

impl Parse for EdgedbQueryInput {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		let module: syn::Ident = input.parse()?;

		let query_content = if input.peek(Token![,]) {
			input.parse::<Token![,]>()?;

			let marker: syn::Ident = input.parse()?;
			input.parse::<Token![:]>()?;
			let raw_content: syn::LitStr = input.parse()?;

			if marker == "file" {
				QueryContent::File(raw_content.value(), raw_content.span())
			} else if marker == "query" {
				QueryContent::Query(raw_content.value())
			} else {
				let message = format!("unexpected marker token: {marker}");
				return Err(syn::Error::new_spanned(marker, message));
			}
		} else {
			QueryContent::File(format!("queries/{module}.edgeql"), module.span())
		};

		let query = query_content.resolve()?;
		Ok(Self { module, query })
	}
}

impl ToTokens for EdgedbQueryInput {
	fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
		let module_name = self.module.to_string();
		let token_stream = get_descriptor_sync(&self.query)
			.and_then(|descriptor| generate_rust_from_query(&descriptor, &module_name, &self.query))
			.unwrap_or_else(|error| syn::Error::from(error).to_compile_error());

		tokens.extend(token_stream);
	}
}

pub(crate) enum QueryContent {
	Query(String),
	File(String, Span),
}

impl QueryContent {
	pub fn resolve(self) -> syn::Result<String> {
		match self {
			QueryContent::Query(query) => Ok(query),
			QueryContent::File(relative_path, span) => {
				let path = resolve_path(relative_path, span)?;

				std::fs::read_to_string(&path)
					.map_err(|error| {
						syn::Error::new(
							span,
							format!("failed to read query file at {}: {}", path.display(), error),
						)
					})
					.map(|value| value.trim().to_string())
			}
		}
	}
}
