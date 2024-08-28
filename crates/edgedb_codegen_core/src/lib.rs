use std::sync::Arc;

use check_keyword::CheckKeyword;
use edgedb_protocol::codec::CAL_DATE_DURATION;
use edgedb_protocol::codec::CAL_LOCAL_DATE;
use edgedb_protocol::codec::CAL_LOCAL_DATETIME;
use edgedb_protocol::codec::CAL_LOCAL_TIME;
use edgedb_protocol::codec::CAL_RELATIVE_DURATION;
use edgedb_protocol::codec::CFG_MEMORY;
use edgedb_protocol::codec::PGVECTOR_VECTOR;
use edgedb_protocol::codec::STD_BIGINT;
use edgedb_protocol::codec::STD_BOOL;
use edgedb_protocol::codec::STD_BYTES;
use edgedb_protocol::codec::STD_DATETIME;
use edgedb_protocol::codec::STD_DECIMAL;
use edgedb_protocol::codec::STD_DURATION;
use edgedb_protocol::codec::STD_FLOAT32;
use edgedb_protocol::codec::STD_FLOAT64;
use edgedb_protocol::codec::STD_INT16;
use edgedb_protocol::codec::STD_INT32;
use edgedb_protocol::codec::STD_INT64;
use edgedb_protocol::codec::STD_JSON;
use edgedb_protocol::codec::STD_STR;
use edgedb_protocol::codec::STD_UUID;
use edgedb_protocol::common::Capabilities;
use edgedb_protocol::common::Cardinality;
use edgedb_protocol::common::CompilationOptions;
use edgedb_protocol::common::IoFormat;
use edgedb_protocol::descriptors::Descriptor;
use edgedb_protocol::descriptors::ShapeElement;
use edgedb_protocol::descriptors::TupleElement;
use edgedb_protocol::descriptors::TypePos;
use edgedb_protocol::descriptors::Typedesc;
use edgedb_protocol::model::Uuid;
use edgedb_protocol::server_message::CommandDataDescription1;
use edgedb_tokio::create_client;
use edgedb_tokio::raw::Pool;
use edgedb_tokio::raw::PoolState;
use edgedb_tokio::Builder;
use heck::ToPascalCase;
use heck::ToSnakeCase;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::Token;
use tokio::runtime::Runtime;
use typed_builder::TypedBuilder;

pub use crate::constants::*;
pub use crate::errors::*;
pub use crate::utils::*;

mod constants;
mod errors;
mod utils;

/// Get the descriptor asynchronously.
pub async fn get_descriptor(query: &str) -> Result<CommandDataDescription1> {
	let builder = Builder::new().build_env().await?;
	let state = Arc::new(PoolState::default());
	let pool = Pool::new(&builder);
	let mut pool_connection = pool.acquire().await?;
	let connection = pool_connection.inner();
	let allow_capabilities = Capabilities::MODIFICATIONS | Capabilities::DDL;
	let flags = CompilationOptions {
		implicit_limit: None,
		implicit_typenames: false,
		implicit_typeids: false,
		explicit_objectids: true,
		allow_capabilities,
		io_format: IoFormat::Binary,
		expected_cardinality: Cardinality::Many,
	};

	Ok(connection.parse(&flags, query, &state).await?)
}

/// Get the descriptor synchronously.
pub fn get_descriptor_sync(query: &str) -> Result<CommandDataDescription1> {
	let rt = Runtime::new()?;
	let descriptor = rt.block_on(async { get_descriptor(query).await })?;

	Ok(descriptor)
}

pub fn generate_rust_from_query(
	descriptor: &CommandDataDescription1,
	name: &str,
	query: &str,
) -> Result<TokenStream> {
	let input_ident = format_ident!("{INPUT_NAME}");
	let output_ident = format_ident!("{OUTPUT_NAME}");
	let props_ident = format_ident!("{PROPS_NAME}");
	let query_ident = format_ident!("{QUERY_NAME}");
	let query_prop_ident = format_ident!("{QUERY_PROP_NAME}");
	let transaction_ident = format_ident!("{TRANSACTION_NAME}");
	let transaction_prop_ident = format_ident!("{TRANSACTION_PROP_NAME}");
	let module_name: syn::Ident = format_ident!("{}", name.to_snake_case());
	let input = descriptor.input.decode()?;
	let output = descriptor.output.decode()?;
	let mut tokens: TokenStream = TokenStream::new();

	explore_descriptor(
		ExploreDescriptorProps::builder()
			.typedesc(&input)
			.is_input()
			.is_root()
			.descriptor(input.root())
			.root_name(INPUT_NAME)
			.build(),
		&mut tokens,
	)?;
	explore_descriptor(
		ExploreDescriptorProps::builder()
			.typedesc(&output)
			.is_root()
			.descriptor(output.root())
			.root_name(OUTPUT_NAME)
			.build(),
		&mut tokens,
	)?;

	let query_method = match descriptor.result_cardinality {
		Cardinality::NoResult => quote!(execute),
		Cardinality::AtMostOne => quote!(query_single),
		Cardinality::One => quote!(query_required_single),
		Cardinality::Many | Cardinality::AtLeastOne => quote!(query),
	};

	let mut query_props = vec![quote!(#query_prop_ident: &#EXPORTS_IDENT::edgedb_tokio::Client)];
	let mut transaction_props =
		vec![quote!(#transaction_prop_ident: &mut #EXPORTS_IDENT::edgedb_tokio::Transaction)];
	let args = vec![
		quote!(#QUERY_CONSTANT),
		input.root().map_or(quote!(&()), |_| quote!(#props_ident)),
	];
	let inner_return = output.root().map_or(quote!(()), |_| quote!(#output_ident));
	let returns = wrap_token_with_cardinality(Some(descriptor.result_cardinality), inner_return);

	if input.root().is_some() {
		query_props.push(quote!(#props_ident: &#input_ident));
		transaction_props.push(quote!(#props_ident: &#input_ident));
	}

	let token_stream = quote! {
		pub mod #module_name {
			use ::edgedb_codegen::exports as #EXPORTS_IDENT;

			/// Execute the desired query.
			#[cfg(feature = "query")]
			pub async fn #query_ident(#(#query_props),*) -> core::result::Result<#returns, #EXPORTS_IDENT::edgedb_errors::Error> {
				#query_prop_ident.#query_method(#(#args),*).await
			}

			/// Compose the query as part of a larger transaction.
			#[cfg(feature = "query")]
			pub async fn #transaction_ident(#(#transaction_props),*) -> core::result::Result<#returns, #EXPORTS_IDENT::edgedb_errors::Error> {
				#transaction_prop_ident.#query_method(#(#args),*).await
			}

			#tokens

			/// The original query string provided to the macro. Can be reused in your codebase.
			pub const #QUERY_CONSTANT: &str = #query;
		}
	};

	Ok(token_stream)
}

fn wrap_token_with_cardinality(
	cardinality: Option<Cardinality>,
	token: TokenStream,
) -> TokenStream {
	let Some(cardinality) = cardinality else {
		return token;
	};

	match cardinality {
		Cardinality::NoResult | Cardinality::AtMostOne => quote!(Option<#token>),
		Cardinality::One => token,
		Cardinality::Many | Cardinality::AtLeastOne => quote!(Vec<#token>),
	}
}

#[derive(Debug, TypedBuilder)]
struct ExploreDescriptorProps<'a> {
	typedesc: &'a Typedesc,
	#[builder(setter(strip_bool(fallback = is_input_bool)))]
	is_input: bool,
	#[builder(setter(strip_bool(fallback = is_root_bool)))]
	is_root: bool,
	descriptor: Option<&'a Descriptor>,
	root_name: &'a str,
}

type PartialExploreDescriptorProps<'a> =
	ExploreDescriptorPropsBuilder<'a, ((&'a Typedesc,), (bool,), (bool,), (), ())>;

impl<'a> ExploreDescriptorProps<'a> {
	fn into_props(self) -> PartialExploreDescriptorProps<'a> {
		let Self {
			typedesc, is_input, ..
		} = self;

		Self::builder()
			.typedesc(typedesc)
			.is_input_bool(is_input)
			.is_root_bool(false)
	}
}

fn explore_descriptor(
	props @ ExploreDescriptorProps {
		typedesc,
		is_input,
		is_root,
		descriptor,
		root_name,
	}: ExploreDescriptorProps,
	tokens: &mut TokenStream,
) -> Result<Option<TokenStream>> {
	let root_ident = format_ident!("{root_name}");

	let Some(descriptor) = descriptor else {
		if is_root {
			tokens.extend(quote!(pub type #root_ident = ();));
		}

		return Ok(None);
	};

	match descriptor {
		Descriptor::Set(set) => {
			let set_descriptor = typedesc.get(set.type_pos).ok();
			let sub_root_name = format!("{root_name}Set");
			let props = props
				.into_props()
				.descriptor(set_descriptor)
				.root_name(&sub_root_name)
				.build();
			let result = explore_descriptor(props, tokens)?.map(|result| quote!(Vec<#result>));

			if is_root {
				tokens.extend(quote!(pub type #root_ident = #result;));
				Ok(Some(quote!(#root_ident)))
			} else {
				Ok(result)
			}
		}
		Descriptor::ObjectShape(object) => {
			let result = explore_object_shape_descriptor(
				StructElement::from_shape(&object.elements),
				typedesc,
				root_name,
				is_input,
				tokens,
			)?;

			Ok(result)
		}
		Descriptor::BaseScalar(base_scalar) => {
			let result = uuid_to_token_name(&base_scalar.id);

			if is_root {
				tokens.extend(quote!(pub type #root_ident = #result;));
				Ok(Some(quote!(#root_ident)))
			} else {
				Ok(Some(result))
			}
		}
		Descriptor::Scalar(scalar) => {
			let props = props
				.into_props()
				.descriptor(typedesc.get(scalar.base_type_pos).ok())
				.root_name(root_name)
				.build();

			explore_descriptor(props, tokens)
		}
		Descriptor::Tuple(tuple) => {
			let mut tuple_tokens = Punctuated::<_, Token![,]>::new();

			for (index, element) in tuple.element_types.iter().enumerate() {
				let sub_root_name = format!("{root_name}{index}");
				let result = explore_descriptor(
					ExploreDescriptorProps::builder()
						.typedesc(typedesc)
						.is_input_bool(is_input)
						.descriptor(typedesc.get(*element).ok())
						.root_name(&sub_root_name)
						.build(),
					tokens,
				)?;

				tuple_tokens.push(result);
			}

			let result = quote!((#tuple_tokens));

			if is_root {
				tokens.extend(quote!(pub type #root_ident = #result;));
				Ok(Some(quote!(#root_ident)))
			} else {
				Ok(Some(result))
			}
		}
		Descriptor::NamedTuple(named_tuple) => {
			let result = explore_object_shape_descriptor(
				StructElement::from_named_tuple(&named_tuple.elements),
				typedesc,
				root_name,
				is_input,
				tokens,
			)?;

			Ok(result)
		}
		Descriptor::Array(array) => {
			let array_descriptor = typedesc.get(array.type_pos).ok();
			let sub_root_name = format!("{root_name}Array");
			let props = props
				.into_props()
				.descriptor(array_descriptor)
				.root_name(&sub_root_name)
				.build();
			let result = explore_descriptor(props, tokens)?.map(|result| quote!(Vec<#result>));

			if is_root {
				tokens.extend(quote!(pub type #root_ident = #result;));
				Ok(Some(quote!(#root_ident)))
			} else {
				Ok(result)
			}
		}
		// TODO once `edgedb_protocol` is updated to 2.0 it should be possible to get the enum name.
		Descriptor::Enumeration(_) => {
			let result = Some(quote!(String));

			if is_root {
				tokens.extend(quote!(pub type #root_ident = #result;));
				Ok(Some(quote!(#root_ident)))
			} else {
				Ok(result)
			}
		}
		Descriptor::InputShape(object) => {
			let result = explore_object_shape_descriptor(
				StructElement::from_shape(&object.elements),
				typedesc,
				root_name,
				is_input,
				tokens,
			)?;

			Ok(result)
		}
		Descriptor::Range(range) => {
			let range_descriptor = typedesc.get(range.type_pos).ok();
			let sub_root_name = format!("{root_name}Range");
			let props = props
				.into_props()
				.descriptor(range_descriptor)
				.root_name(&sub_root_name)
				.build();
			let result = explore_descriptor(props, tokens)?
				.map(|result| quote!(#EXPORTS_IDENT::edgedb_protocol::model::Range<#result>));

			if is_root {
				tokens.extend(quote!(pub type #root_ident = #result;));
				Ok(Some(quote!(#root_ident)))
			} else {
				Ok(result)
			}
		}
		Descriptor::MultiRange(_) => todo!("`multirange` not in the `edgedb_protocol` crate"),
		Descriptor::TypeAnnotation(_) => todo!("type annotations are not supported"),
	}
}

pub fn explore_object_shape_descriptor(
	elements: Vec<StructElement<'_>>,
	typedesc: &Typedesc,
	root_name: &str,
	is_input: bool,
	tokens: &mut TokenStream,
) -> Result<Option<TokenStream>> {
	let mut impl_named_args = vec![];
	let mut struct_fields = vec![];
	let root_ident: syn::Ident = syn::parse_str(root_name)?;

	for element in elements {
		let descriptor = typedesc.get(element.type_pos()).ok();
		let name = &element.name();
		let safe_name = name.to_snake_case().into_safe();
		let safe_name_ident = format_ident!("{safe_name}");
		let pascal_name = name.to_pascal_case();
		let sub_root_name = format!("{root_name}{pascal_name}").into_safe();
		let sub_props = ExploreDescriptorProps::builder()
			.typedesc(typedesc)
			.is_input_bool(is_input)
			.descriptor(descriptor)
			.root_name(&sub_root_name)
			.build();
		let output = explore_descriptor(sub_props, tokens)?;
		let output_token = element.wrap(&output);
		let serde_annotation = (&safe_name != name).then_some(quote!(
			#[cfg_attr(feature = "serde", serde(rename = #name))]
		));
		let builder_fields = {
			match element.cardinality() {
				Cardinality::AtMostOne => {
					let fallback_ident = format_ident!("{safe_name_ident}_opt");
					Some(quote!(default, setter(into, strip_option(fallback = #fallback_ident))))
				}
				Cardinality::One => Some(quote!(setter(into))),
				Cardinality::Many => Some(quote!(default)),
				Cardinality::NoResult | Cardinality::AtLeastOne => None,
			}
		};
		let builder_annotation = (is_input && builder_fields.is_some()).then_some(quote!(
			#[cfg_attr(feature = "builder", builder(#builder_fields))]
		));

		struct_fields.push(quote! {
			#serde_annotation
			#builder_annotation
			pub #safe_name_ident: #output_token,
		});

		if is_input {
			impl_named_args.push(quote!(#name => self.#safe_name_ident.clone(),));
		}
	}

	let impl_tokens = is_input.then_some(quote! {
		impl #EXPORTS_IDENT::edgedb_protocol::query_arg::QueryArgs for #root_ident {
			fn encode(&self, encoder: &mut #EXPORTS_IDENT::edgedb_protocol::query_arg::Encoder) -> core::result::Result<(), #EXPORTS_IDENT::edgedb_errors::Error> {
				let map = #EXPORTS_IDENT::edgedb_protocol::named_args! {
					#(#impl_named_args)*
				};

				map.encode(encoder)
			}
		}
	});
	let typed_builder_tokens = is_input.then_some(
		quote!(#[cfg_attr(feature = "builder", derive(#EXPORTS_IDENT::typed_builder::TypedBuilder))]),
	);
	let struct_tokens = quote! {
		#[derive(Clone, Debug)]
		#typed_builder_tokens
		#[cfg_attr(feature = "query", derive(#EXPORTS_IDENT::edgedb_derive::Queryable))]
		#[cfg_attr(feature = "serde", derive(#EXPORTS_IDENT::serde::Serialize, #EXPORTS_IDENT::serde::Deserialize))]
		pub struct #root_ident {
			#(#struct_fields)*
		}

		#impl_tokens
	};

	tokens.extend(struct_tokens);

	Ok(Some(quote!(#root_ident)))
}

pub enum StructElement<'a> {
	Shape(&'a ShapeElement),
	Tuple(&'a TupleElement),
}

impl<'a> StructElement<'a> {
	pub fn from_shape(elements: &'a [ShapeElement]) -> Vec<StructElement<'a>> {
		elements.iter().map(From::from).collect::<Vec<_>>()
	}

	pub fn from_named_tuple(elements: &'a [TupleElement]) -> Vec<StructElement<'a>> {
		elements.iter().map(From::from).collect::<Vec<_>>()
	}

	pub fn name(&self) -> String {
		match self {
			StructElement::Shape(shape) => shape.name.clone(),
			StructElement::Tuple(tuple) => tuple.name.clone(),
		}
	}

	pub fn type_pos(&self) -> TypePos {
		match self {
			StructElement::Shape(shape) => shape.type_pos,
			StructElement::Tuple(tuple) => tuple.type_pos,
		}
	}

	pub fn wrap(&self, token: &Option<TokenStream>) -> TokenStream {
		if let Cardinality::AtMostOne = self.cardinality() {
			quote!(Option<#token>)
		} else {
			quote!(#token)
		}
	}

	pub fn cardinality(&self) -> Cardinality {
		match self {
			StructElement::Shape(shape) => shape.cardinality.unwrap_or(Cardinality::NoResult),
			StructElement::Tuple(_) => Cardinality::NoResult,
		}
	}
}

impl<'a> From<&'a ShapeElement> for StructElement<'a> {
	fn from(value: &'a ShapeElement) -> Self {
		StructElement::Shape(value)
	}
}

impl<'a> From<&'a TupleElement> for StructElement<'a> {
	fn from(value: &'a TupleElement) -> Self {
		StructElement::Tuple(value)
	}
}

fn uuid_to_token_name(uuid: &Uuid) -> TokenStream {
	match *uuid {
		STD_UUID => quote!(#EXPORTS_IDENT::uuid::Uuid),
		STD_STR => quote!(String),
		STD_BYTES => quote!(#EXPORTS_IDENT::bytes::Bytes),
		STD_INT16 => quote!(i16),
		STD_INT32 => quote!(i32),
		STD_INT64 => quote!(i64),
		STD_FLOAT32 => quote!(f32),
		STD_FLOAT64 => quote!(f64),
		#[cfg(not(feature = "with_bigdecimal"))]
		STD_DECIMAL => quote!(#EXPORTS_IDENT::edgedb_protocol::model::Decimal),
		#[cfg(feature = "with_bigdecimal")]
		STD_DECIMAL => quote!(#EXPORTS_IDENT::bigdecimal::BigDecimal),
		STD_BOOL => quote!(bool),
		#[cfg(not(feature = "with_chrono"))]
		STD_DATETIME => quote!(#EXPORTS_IDENT::edgedb_protocol::model::Datetime),
		#[cfg(feature = "with_chrono")]
		STD_DATETIME => quote!(#EXPORTS_IDENT::chrono::DateTime<#EXPORTS_IDENT::chrono::Utc>),
		#[cfg(not(feature = "with_chrono"))]
		CAL_LOCAL_DATETIME => quote!(#EXPORTS_IDENT::edgedb_protocol::model::LocalDatetime),
		#[cfg(feature = "with_chrono")]
		CAL_LOCAL_DATETIME => quote!(#EXPORTS_IDENT::chrono::NaiveDateTime),
		#[cfg(not(feature = "with_chrono"))]
		CAL_LOCAL_DATE => quote!(#EXPORTS_IDENT::edgedb_protocol::model::LocalDate),
		#[cfg(feature = "with_chrono")]
		CAL_LOCAL_DATE => quote!(#EXPORTS_IDENT::chrono::NaiveDate),
		#[cfg(not(feature = "with_chrono"))]
		CAL_LOCAL_TIME => quote!(#EXPORTS_IDENT::edgedb_protocol::model::LocalTime),
		#[cfg(feature = "with_chrono")]
		CAL_LOCAL_TIME => quote!(#EXPORTS_IDENT::chrono::NaiveTime),
		STD_DURATION => quote!(#EXPORTS_IDENT::edgedb_protocol::model::Duration),
		CAL_RELATIVE_DURATION => quote!(#EXPORTS_IDENT::edgedb_protocol::model::RelativeDuration),
		CAL_DATE_DURATION => quote!(#EXPORTS_IDENT::edgedb_protocol::model::DateDuration),
		STD_JSON => quote!(#EXPORTS_IDENT::edgedb_protocol::model::Json),
		#[cfg(not(feature = "with_bigint"))]
		STD_BIGINT => quote!(#EXPORTS_IDENT::edgedb_protocol::model::BigInt),
		#[cfg(feature = "with_bigint")]
		STD_BIGINT => quote!(#EXPORTS_IDENT::num_bigint::BigInt),
		CFG_MEMORY => quote!(#EXPORTS_IDENT::edgedb_protocol::model::ConfigMemory),
		PGVECTOR_VECTOR => quote!(#EXPORTS_IDENT::edgedb_protocol::model::Vector),
		_ => quote!(()),
	}
}

pub async fn get_types() -> Result<()> {
	let client = create_client().await?;
	let json = client.query_json(TYPES_QUERY, &()).await?;
	log::debug!("{}", json.as_ref());

	Ok(())
}
