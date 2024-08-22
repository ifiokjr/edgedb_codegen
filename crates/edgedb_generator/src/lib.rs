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
use edgedb_tokio::create_client;
use edgedb_tokio::raw::Pool;
use edgedb_tokio::raw::PoolState;
use edgedb_tokio::Builder;
use heck::ToKebabCase;
use heck::ToPascalCase;
use heck::ToSnakeCase;

pub mod utils;

const INPUT_NAME: &str = "Input";
const OUTPUT_NAME: &str = "Output";
const QUERY_NAME: &str = "query";

pub async fn generate_rust_from_query(name: &str, query: &str) -> anyhow::Result<String> {
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

	let descriptor = connection.parse(&flags, query, &state).await?;
	let kebab_name = name.to_kebab_case();
	let input = descriptor.input.decode()?;
	let output = descriptor.output.decode()?;
	let mut strings: Vec<String> = vec![format!("pub mod {kebab_name} {{")];

	explore_descriptor(input.root(), &input, INPUT_NAME, true, &mut strings);
	explore_descriptor(output.root(), &output, OUTPUT_NAME, false, &mut strings);

	let (prefix, suffix, query_method) = match descriptor.result_cardinality {
		Cardinality::NoResult | Cardinality::AtMostOne => ("Option<", ">", "query_single"),
		Cardinality::One => ("", "", "query_required_single"),
		Cardinality::Many | Cardinality::AtLeastOne => ("Vec<", ">", "query"),
	};

	let props = input
		.root()
		.map_or(String::new(), |_| format!(", props: &{INPUT_NAME}"));
	let args = input.root().map_or("&()", |_| "props");
	let returns = output
		.root()
		.map_or("()".into(), |_| OUTPUT_NAME.to_string());
	let returns = format!("{prefix}{returns}{suffix}");
	strings.push(format!(
		"pub async fn {QUERY_NAME}(client: &edgedb_tokio::Client{props}) -> \
		 core::result::Result<{returns}, edgedb_errors:Error> \
		 {{\n\tclient.{query_method}(r#\"{query}\"#, {args}).await\n}}"
	));
	strings.push("}".to_string());

	Ok(strings.join("\n"))
}

pub fn explore_descriptor(
	maybe_descriptor: Option<&Descriptor>,
	typedesc: &Typedesc,
	root_name: &str,
	is_input: bool,
	strings: &mut Vec<String>,
) -> Option<String> {
	let descriptor = maybe_descriptor?;

	match descriptor {
		Descriptor::Set(set) => {
			let set_descriptor = typedesc.get(set.type_pos).ok();
			explore_descriptor(set_descriptor, typedesc, root_name, is_input, strings)
		}
		Descriptor::ObjectShape(object) => {
			let mut struct_strings = vec![];
			strings.push(explore_object_shape_descriptor(
				StructElement::from_shape(&object.elements),
				typedesc,
				root_name,
				is_input,
				&mut struct_strings,
			)?);
			strings.push(struct_strings.join("\n"));
			Some(root_name.into())
		}
		Descriptor::BaseScalar(base_scalar) => uuid_to_known_name(&base_scalar.id),
		Descriptor::Scalar(scalar) => {
			explore_descriptor(
				typedesc.get(scalar.base_type_pos).ok(),
				typedesc,
				root_name,
				is_input,
				strings,
			)
		}
		Descriptor::Tuple(tuple) => {
			let mut tuple_strings = vec![];

			for element in &tuple.element_types {
				tuple_strings.push(
					explore_descriptor(
						typedesc.get(*element).ok(),
						typedesc,
						root_name,
						is_input,
						&mut vec![],
					)
					.unwrap(),
				);
			}

			Some(format!("({})", tuple_strings.join(", ")))
		}
		Descriptor::NamedTuple(named_tuple) => {
			let mut struct_strings = vec![];
			strings.push(explore_object_shape_descriptor(
				StructElement::from_named_tuple(&named_tuple.elements),
				typedesc,
				root_name,
				is_input,
				&mut struct_strings,
			)?);
			strings.push(struct_strings.join("\n"));

			Some(root_name.into())
		}
		Descriptor::Array(array) => {
			let mut array_strings = vec![];
			let array_descriptor = typedesc.get(array.type_pos).ok();
			let result = explore_descriptor(
				array_descriptor,
				typedesc,
				root_name,
				is_input,
				&mut array_strings,
			)
			.unwrap();

			Some(format!("Vec<{result}>"))
		}
		Descriptor::Enumeration(enumeration) => {
			log::info!("enumeration: {enumeration:#?}");
			log::info!("enum name: {root_name}");
			Some("String".into())

			// todo!("enumeration in progress")
		}
		Descriptor::InputShape(object) => {
			let mut struct_strings = vec![];
			strings.push(explore_object_shape_descriptor(
				StructElement::from_shape(&object.elements),
				typedesc,
				root_name,
				is_input,
				&mut struct_strings,
			)?);
			strings.push(struct_strings.join("\n"));
			Some(root_name.into())
		}
		Descriptor::Range(_) => todo!("`range` is not public in the `protocol` crate"),
		Descriptor::MultiRange(_) => todo!("`multirange` does not exist in the `protocol` crate"),
		Descriptor::TypeAnnotation(_) => todo!("type annotations are not supported"),
	}
}

pub fn explore_object_shape_descriptor(
	elements: Vec<StructElement<'_>>,
	typedesc: &Typedesc,
	root_name: &str,
	is_input: bool,
	strings: &mut Vec<String>,
) -> Option<String> {
	let mut struct_strings: Vec<String> = vec![format!(
		"#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, \
		 edgedb_derive::Queryable)]\npub struct {root_name} {{"
	)];
	let mut impl_strings: Vec<String> = vec![];

	if is_input {
		impl_strings.push(format!(
			"impl QueryArgs for {root_name} {{\n\tfn encode(&self, encoder: &mut \
			 edgedb_protocol::query_arg::Encoder) -> Result<(), edgedb_errors::Error> {{\n\t\tlet \
			 map = edgedb_protocol::named_args! {{"
		));
	}

	for element in elements {
		let (prefix, suffix) = element.wrapper();
		let descriptor = typedesc.get(element.type_pos()).ok();
		let name = &element.name();
		let safe_name = name.to_snake_case().into_safe();
		let pascal_name = name.to_pascal_case();
		let root_name = format!("{root_name}{pascal_name}");
		let output = explore_descriptor(descriptor, typedesc, &root_name, is_input, strings)?;

		struct_strings.push(format!("\tpub {safe_name}: {prefix}{output}{suffix},"));

		if is_input {
			impl_strings.push(format!("\t\t\t\"{name}\" => self.{safe_name}.clone(),"));
		}
	}

	struct_strings.push("}".into());

	if is_input {
		impl_strings.push("\t\t};\n\t\tmap.encode(encoder)\n\t}\n}".to_string());
		struct_strings.push(impl_strings.join("\n"));
	}

	Some(struct_strings.join("\n"))
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

	pub fn wrapper(&self) -> (String, String) {
		match self {
			StructElement::Shape(shape) => cardinality_wrapper(shape.cardinality),
			StructElement::Tuple(_) => (String::new(), String::new()),
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

fn cardinality_wrapper(cardinality: Option<Cardinality>) -> (String, String) {
	let Some(cardinality) = cardinality else {
		return (String::new(), String::new());
	};

	match cardinality {
		Cardinality::NoResult | Cardinality::AtMostOne => ("Option<".into(), ">".into()),
		Cardinality::One => (String::new(), String::new()),
		Cardinality::Many | Cardinality::AtLeastOne => ("Vec<".into(), ">".into()),
	}
}

fn uuid_to_known_name(uuid: &Uuid) -> Option<String> {
	match *uuid {
		STD_UUID => Some("uuid::Uuid".into()),
		STD_STR => Some("String".into()),
		STD_BYTES => Some("bytes::Bytes".into()),
		STD_INT16 => Some("i16".into()),
		STD_INT32 => Some("i32".into()),
		STD_INT64 => Some("i64".into()),
		STD_FLOAT32 => Some("f32".into()),
		STD_FLOAT64 => Some("f64".into()),
		STD_DECIMAL => Some("edgedb_protocol::model::Decimal".into()),
		STD_BOOL => Some("bool".into()),
		STD_DATETIME => Some("edgedb_protocol::model::DateTime".into()),
		CAL_LOCAL_DATETIME => Some("edgedb_protocol::model::LocalDateTime".into()),
		CAL_LOCAL_DATE => Some("edgedb_protocol::model::LocalDate".into()),
		CAL_LOCAL_TIME => Some("edgedb_protocol::model::LocalTime".into()),
		STD_DURATION => Some("edgedb_protocol::model::Duration".into()),
		CAL_RELATIVE_DURATION => Some("edgedb_protocol::model::RelativeDuration".into()),
		CAL_DATE_DURATION => Some("edgedb_protocol::model::DateDuration".into()),
		STD_JSON => Some("edgedb_protocol::model::Json".into()),
		STD_BIGINT => Some("edgedb_protocol::model::BigInt".into()),
		CFG_MEMORY => Some("edgedb_protocol::model::ConfigMemory".into()),
		PGVECTOR_VECTOR => Some("edgedb_protocol::model::Vector".into()),
		_ => None,
	}
}

const TYPES_QUERY: &str = r#"WITH
  MODULE schema,
  material_scalars := (
    SELECT ScalarType
    FILTER NOT .abstract
       AND NOT EXISTS .enum_values
       AND NOT EXISTS (SELECT .ancestors FILTER NOT .abstract)
  )

	SELECT Type {
	  id,
	  name :=
	    array_join(array_agg([IS ObjectType].union_of.name), ' | ')
	    IF EXISTS [IS ObjectType].union_of
	    ELSE .name,
	  is_abstract := .abstract,

	  kind := 'object' IF Type IS ObjectType ELSE
	          'scalar' IF Type IS ScalarType ELSE
	          'array' IF Type IS Array ELSE
	          'tuple' IF Type IS Tuple ELSE
	          'multirange' IF Type IS MultiRange ELSE 
	          'unknown',

	  [IS ScalarType].enum_values,
	  is_seq := 'std::sequence' in [IS ScalarType].ancestors.name,
	  # for sequence (abstract type that has non-abstract ancestor)
	  single material_id := (
	    SELECT x := Type[IS ScalarType].ancestors
	    FILTER x IN material_scalars
	    LIMIT 1
	  ).id,

	  [IS InheritingObject].bases: {
	    id
	  } ORDER BY @index ASC,

	  [IS ObjectType].union_of,
	  [IS ObjectType].intersection_of,
	  [IS ObjectType].pointers: {
	    card := ('One' IF .required ELSE 'AtMostOne') IF <str>.cardinality = 'One' ELSE ('AtLeastOne' IF .required ELSE 'Many'),
	    name,
	    target_id := .target.id,
	    kind := 'link' IF .__type__.name = 'schema::Link' ELSE 'property',
	    is_exclusive := exists (select .constraints filter .name = 'std::exclusive'),
	    is_computed := len(.computed_fields) != 0,
	    is_readonly := .readonly,
	    has_default := EXISTS .default or ('std::sequence' in .target[IS ScalarType].ancestors.name),
	    [IS Link].pointers: {
	      card := ('One' IF .required ELSE 'AtMostOne') IF <str>.cardinality = "One" ELSE ('AtLeastOne' IF .required ELSE 'Many'),
	      name := '@' ++ .name,
	      target_id := .target.id,
	      kind := 'link' IF .__type__.name = 'schema::Link' ELSE 'property',
	      is_computed := len(.computed_fields) != 0,
	      is_readonly := .readonly
	    } filter .name != '@source' and .name != '@target',
	  } FILTER @is_owned,
	  exclusives := assert_distinct((
	    [is schema::ObjectType].constraints
	    union
	    [is schema::ObjectType].pointers.constraints
	  ) {
	    target := (.subject[is schema::Property].name ?? .subject[is schema::Link].name ?? .subjectexpr)
	  } filter .name = 'std::exclusive'),
	  backlinks := (
	     SELECT DETACHED Link
	     FILTER .target = Type
	       AND NOT EXISTS .source[IS ObjectType].union_of
	    ) {
	    card := 'AtMostOne'
	      IF
	      EXISTS (select .constraints filter .name = 'std::exclusive')
	      ELSE
	      'Many',
	    name := '<' ++ .name ++ '[is ' ++ assert_exists(.source.name) ++ ']',
	    stub := .name,
	    target_id := .source.id,
	    kind := 'link',
	    is_exclusive := (EXISTS (select .constraints filter .name = 'std::exclusive')) AND <str>.cardinality = 'One',
	  },
	  backlink_stubs := array_agg((
	    WITH
	      stubs := DISTINCT (SELECT DETACHED Link FILTER .target = Type).name,
	      baseObjectId := (SELECT DETACHED ObjectType FILTER .name = 'std::BaseObject' LIMIT 1).id
	    FOR stub in { stubs }
	    UNION (
	      SELECT {
	        card := 'Many',
	        name := '<' ++ stub,
	        target_id := baseObjectId,
	        kind := 'link',
	        is_exclusive := false,
	      }
	    )
	  )),
	  array_element_id := [IS Array].element_type.id,

	  tuple_elements := (SELECT [IS Tuple].element_types {
	    target_id := .type.id,
	    name
	  } ORDER BY @index ASC),
		 multirange_element_id := [IS MultiRange].element_type.id,
	}
ORDER BY .name;
"#;

pub async fn get_types() -> anyhow::Result<()> {
	let client = create_client().await?;
	let json = client.query_json(TYPES_QUERY, &()).await?;
	// let json = serde_json::from_str(json.as_ref())?;
	log::info!("{}", json.as_ref());

	Ok(())
}

#[cfg(test)]
mod tests {
	use anyhow::Result;

	use super::*;

	#[tokio::test]
	async fn can_generate_content_without_args() -> Result<()> {
		let content = generate_rust_from_query("example", "select Team {**}").await?;

		insta::assert_snapshot!(content);

		Ok(())
	}

	#[tokio::test]
	async fn can_generate_content_with_args() -> Result<()> {
		let content = generate_rust_from_query(
			"example",
			"select Team {**} filter .name like <str>$starts_with ++ '%' and .description like \
			 '%' ++ <str>$ends_with;",
		)
		.await?;

		insta::assert_snapshot!(content);

		Ok(())
	}

	#[tokio::test]
	async fn can_generate_singular_content() -> Result<()> {
		let content = generate_rust_from_query(
			"example",
			"select Team {**} filter .name like <str>$starts_with ++ '%' and .description like \
			 '%' ++ <str>$ends_with;",
		)
		.await?;

		insta::assert_snapshot!(content);

		Ok(())
	}

	#[test_log::test(tokio::test)]
	async fn each_thing() -> Result<()> {
		let query = "
		select {
		  my_string := RelationshipType.Follow,
		  my_number := 42,
		  several_numbers := {1, 2, 3},
		  array := [1, 2, 3],
		};";
		let content = generate_rust_from_query("example", query).await?;

		insta::assert_snapshot!(content);

		Ok(())
	}

	#[test_log::test(tokio::test)]
	async fn explore() -> Result<()> {
		// get_types().await?;
		// check!(false);
		// let query = "
		// select {
		//   my_string := RelationshipType.Follow,
		//   my_number := 42,
		//   several_numbers := {1, 2, 3},
		//   array := [1, 2, 3],
		// };";
		let content = generate_rust_from_query("types", TYPES_QUERY).await?;

		insta::assert_snapshot!(content);

		Ok(())
	}
}
