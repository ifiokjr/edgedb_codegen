use edgedb_codegen::generate_rust_from_query;
use edgedb_codegen::get_descriptor_sync;
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse::Parse;
use syn::parse_macro_input;
use syn::Token;

/// Generates a query module from a query string.
///
/// ```rust
/// use edgedb_macros::edgedb_query;
///
/// edgedb_query!(get_users, "select User {**}");
/// ```
#[proc_macro]
pub fn edgedb_query(input: TokenStream) -> TokenStream {
	// 1. Use syn to parse the input tokens into a syntax tree.
	// 2. Use quote to generate new tokens based on what we parsed.
	// 3. Return the generated tokens.

	parse_macro_input!(input as EdgedbQueryInput)
		.to_token_stream()
		.into()
}

pub(crate) struct EdgedbQueryInput {
	pub(crate) module_name: syn::Ident,
	pub(crate) query: syn::LitStr,
}

impl Parse for EdgedbQueryInput {
	fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
		let module_name: syn::Ident = input.parse()?;
		input.parse::<Token![,]>()?;
		let query: syn::LitStr = input.parse()?;

		Ok(Self { module_name, query })
	}
}

impl ToTokens for EdgedbQueryInput {
	fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
		let query = self.query.value();
		let module_name = self.module_name.to_string();
		let token_stream = get_descriptor_sync(&query)
			.and_then(|descriptor| generate_rust_from_query(&descriptor, &module_name, &query))
			.unwrap_or_else(|error| syn::Error::from(error).to_compile_error());

		tokens.extend(token_stream);
	}
}
