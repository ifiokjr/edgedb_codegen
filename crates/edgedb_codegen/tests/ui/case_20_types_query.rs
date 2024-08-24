fn main() {
	pub mod example {
		#[doc = r" Execute the desired query."]
		#[cfg(feature = "query")]
		pub async fn query(
			client: &edgedb_tokio::Client,
		) -> core::result::Result<Vec<Output>, edgedb_errors::Error> {
			client
				.query(
					"WITH\n  MODULE schema,\n  material_scalars := (\n    SELECT ScalarType\n    \
					 FILTER NOT .abstract\n       AND NOT EXISTS .enum_values\n       AND NOT \
					 EXISTS (SELECT .ancestors FILTER NOT .abstract)\n  )\n\n\tSELECT Type {\n\t  \
					 id,\n\t  name :=\n\t    array_join(array_agg([IS ObjectType].union_of.name), \
					 ' | ')\n\t    IF EXISTS [IS ObjectType].union_of\n\t    ELSE .name,\n\t  \
					 is_abstract := .abstract,\n\n\t  kind := 'object' IF Type IS ObjectType \
					 ELSE\n\t          'scalar' IF Type IS ScalarType ELSE\n\t          'array' \
					 IF Type IS Array ELSE\n\t          'tuple' IF Type IS Tuple ELSE\n\t          \
					 'multirange' IF Type IS MultiRange ELSE\n\t          'unknown',\n\n\t  [IS \
					 ScalarType].enum_values,\n\t  is_seq := 'std::sequence' in [IS \
					 ScalarType].ancestors.name,\n\t  # for sequence (abstract type that has \
					 non-abstract ancestor)\n\t  single material_id := (\n\t    SELECT x := \
					 Type[IS ScalarType].ancestors\n\t    FILTER x IN material_scalars\n\t    \
					 LIMIT 1\n\t  ).id,\n\n\t  [IS InheritingObject].bases: {\n\t    id\n\t  } \
					 ORDER BY @index ASC,\n\n\t  [IS ObjectType].union_of,\n\t  [IS \
					 ObjectType].intersection_of,\n\t  [IS ObjectType].pointers: {\n\t    card := \
					 ('One' IF .required ELSE 'AtMostOne') IF <str>.cardinality = 'One' ELSE \
					 ('AtLeastOne' IF .required ELSE 'Many'),\n\t    name,\n\t    target_id := \
					 .target.id,\n\t    kind := 'link' IF .__type__.name = 'schema::Link' ELSE \
					 'property',\n\t    is_exclusive := exists (select .constraints filter .name \
					 = 'std::exclusive'),\n\t    is_computed := len(.computed_fields) != 0,\n\t    \
					 is_readonly := .readonly,\n\t    has_default := EXISTS .default or \
					 ('std::sequence' in .target[IS ScalarType].ancestors.name),\n\t    [IS \
					 Link].pointers: {\n\t      card := ('One' IF .required ELSE 'AtMostOne') IF \
					 <str>.cardinality = \"One\" ELSE ('AtLeastOne' IF .required ELSE \
					 'Many'),\n\t      name := '@' ++ .name,\n\t      target_id := \
					 .target.id,\n\t      kind := 'link' IF .__type__.name = 'schema::Link' ELSE \
					 'property',\n\t      is_computed := len(.computed_fields) != 0,\n\t      \
					 is_readonly := .readonly\n\t    } filter .name != '@source' and .name != \
					 '@target',\n\t  } FILTER @is_owned,\n\t  exclusives := assert_distinct((\n\t    \
					 [is schema::ObjectType].constraints\n\t    union\n\t    [is \
					 schema::ObjectType].pointers.constraints\n\t  ) {\n\t    target := \
					 (.subject[is schema::Property].name ?? .subject[is schema::Link].name ?? \
					 .subjectexpr)\n\t  } filter .name = 'std::exclusive'),\n\t  backlinks := \
					 (\n\t     SELECT DETACHED Link\n\t     FILTER .target = Type\n\t       AND \
					 NOT EXISTS .source[IS ObjectType].union_of\n\t    ) {\n\t    card := \
					 'AtMostOne'\n\t      IF\n\t      EXISTS (select .constraints filter .name = \
					 'std::exclusive')\n\t      ELSE\n\t      'Many',\n\t    name := '<' ++ .name \
					 ++ '[is ' ++ assert_exists(.source.name) ++ ']',\n\t    stub := .name,\n\t    \
					 target_id := .source.id,\n\t    kind := 'link',\n\t    is_exclusive := \
					 (EXISTS (select .constraints filter .name = 'std::exclusive')) AND \
					 <str>.cardinality = 'One',\n\t  },\n\t  backlink_stubs := array_agg((\n\t    \
					 WITH\n\t      stubs := DISTINCT (SELECT DETACHED Link FILTER .target = \
					 Type).name,\n\t      baseObjectId := (SELECT DETACHED ObjectType FILTER \
					 .name = 'std::BaseObject' LIMIT 1).id\n\t    FOR stub in { stubs }\n\t    \
					 UNION (\n\t      SELECT {\n\t        card := 'Many',\n\t        name := '<' \
					 ++ stub,\n\t        target_id := baseObjectId,\n\t        kind := \
					 'link',\n\t        is_exclusive := false,\n\t      }\n\t    )\n\t  )),\n\t  \
					 array_element_id := [IS Array].element_type.id,\n\n\t  tuple_elements := \
					 (SELECT [IS Tuple].element_types {\n\t    target_id := .type.id,\n\t    \
					 name\n\t  } ORDER BY @index ASC),\n\t\t multirange_element_id := [IS \
					 MultiRange].element_type.id,\n\t}\nORDER BY .name;\n",
					&(),
				)
				.await
		}
		#[doc = r" Compose the query as part of a larger transaction."]
		#[cfg(feature = "query")]
		pub async fn transaction(
			conn: &mut edgedb_tokio::Transaction,
		) -> core::result::Result<Vec<Output>, edgedb_errors::Error> {
			conn.query(
				"WITH\n  MODULE schema,\n  material_scalars := (\n    SELECT ScalarType\n    \
				 FILTER NOT .abstract\n       AND NOT EXISTS .enum_values\n       AND NOT EXISTS \
				 (SELECT .ancestors FILTER NOT .abstract)\n  )\n\n\tSELECT Type {\n\t  id,\n\t  \
				 name :=\n\t    array_join(array_agg([IS ObjectType].union_of.name), ' | ')\n\t    \
				 IF EXISTS [IS ObjectType].union_of\n\t    ELSE .name,\n\t  is_abstract := \
				 .abstract,\n\n\t  kind := 'object' IF Type IS ObjectType ELSE\n\t          \
				 'scalar' IF Type IS ScalarType ELSE\n\t          'array' IF Type IS Array \
				 ELSE\n\t          'tuple' IF Type IS Tuple ELSE\n\t          'multirange' IF \
				 Type IS MultiRange ELSE\n\t          'unknown',\n\n\t  [IS \
				 ScalarType].enum_values,\n\t  is_seq := 'std::sequence' in [IS \
				 ScalarType].ancestors.name,\n\t  # for sequence (abstract type that has \
				 non-abstract ancestor)\n\t  single material_id := (\n\t    SELECT x := Type[IS \
				 ScalarType].ancestors\n\t    FILTER x IN material_scalars\n\t    LIMIT 1\n\t  \
				 ).id,\n\n\t  [IS InheritingObject].bases: {\n\t    id\n\t  } ORDER BY @index \
				 ASC,\n\n\t  [IS ObjectType].union_of,\n\t  [IS ObjectType].intersection_of,\n\t  \
				 [IS ObjectType].pointers: {\n\t    card := ('One' IF .required ELSE 'AtMostOne') \
				 IF <str>.cardinality = 'One' ELSE ('AtLeastOne' IF .required ELSE 'Many'),\n\t    \
				 name,\n\t    target_id := .target.id,\n\t    kind := 'link' IF .__type__.name = \
				 'schema::Link' ELSE 'property',\n\t    is_exclusive := exists (select \
				 .constraints filter .name = 'std::exclusive'),\n\t    is_computed := \
				 len(.computed_fields) != 0,\n\t    is_readonly := .readonly,\n\t    has_default \
				 := EXISTS .default or ('std::sequence' in .target[IS \
				 ScalarType].ancestors.name),\n\t    [IS Link].pointers: {\n\t      card := \
				 ('One' IF .required ELSE 'AtMostOne') IF <str>.cardinality = \"One\" ELSE \
				 ('AtLeastOne' IF .required ELSE 'Many'),\n\t      name := '@' ++ .name,\n\t      \
				 target_id := .target.id,\n\t      kind := 'link' IF .__type__.name = \
				 'schema::Link' ELSE 'property',\n\t      is_computed := len(.computed_fields) != \
				 0,\n\t      is_readonly := .readonly\n\t    } filter .name != '@source' and \
				 .name != '@target',\n\t  } FILTER @is_owned,\n\t  exclusives := \
				 assert_distinct((\n\t    [is schema::ObjectType].constraints\n\t    union\n\t    \
				 [is schema::ObjectType].pointers.constraints\n\t  ) {\n\t    target := \
				 (.subject[is schema::Property].name ?? .subject[is schema::Link].name ?? \
				 .subjectexpr)\n\t  } filter .name = 'std::exclusive'),\n\t  backlinks := (\n\t     \
				 SELECT DETACHED Link\n\t     FILTER .target = Type\n\t       AND NOT EXISTS \
				 .source[IS ObjectType].union_of\n\t    ) {\n\t    card := 'AtMostOne'\n\t      \
				 IF\n\t      EXISTS (select .constraints filter .name = 'std::exclusive')\n\t      \
				 ELSE\n\t      'Many',\n\t    name := '<' ++ .name ++ '[is ' ++ \
				 assert_exists(.source.name) ++ ']',\n\t    stub := .name,\n\t    target_id := \
				 .source.id,\n\t    kind := 'link',\n\t    is_exclusive := (EXISTS (select \
				 .constraints filter .name = 'std::exclusive')) AND <str>.cardinality = \
				 'One',\n\t  },\n\t  backlink_stubs := array_agg((\n\t    WITH\n\t      stubs := \
				 DISTINCT (SELECT DETACHED Link FILTER .target = Type).name,\n\t      \
				 baseObjectId := (SELECT DETACHED ObjectType FILTER .name = 'std::BaseObject' \
				 LIMIT 1).id\n\t    FOR stub in { stubs }\n\t    UNION (\n\t      SELECT {\n\t        \
				 card := 'Many',\n\t        name := '<' ++ stub,\n\t        target_id := \
				 baseObjectId,\n\t        kind := 'link',\n\t        is_exclusive := false,\n\t      \
				 }\n\t    )\n\t  )),\n\t  array_element_id := [IS Array].element_type.id,\n\n\t  \
				 tuple_elements := (SELECT [IS Tuple].element_types {\n\t    target_id := \
				 .type.id,\n\t    name\n\t  } ORDER BY @index ASC),\n\t\t multirange_element_id \
				 := [IS MultiRange].element_type.id,\n\t}\nORDER BY .name;\n",
				&(),
			)
			.await
		}
		pub type Input = ();
		#[derive(Clone, Debug)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
		#[cfg_attr(feature = "query", derive(edgedb_derive::Queryable))]
		#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
		pub struct OutputBasesSet {
			#[cfg_attr(feature = "builder", builder(setter(into)))]
			pub id: uuid::Uuid,
		}
		#[derive(Clone, Debug)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
		#[cfg_attr(feature = "query", derive(edgedb_derive::Queryable))]
		#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
		pub struct OutputUnionOfSet {
			#[cfg_attr(feature = "builder", builder(setter(into)))]
			pub id: uuid::Uuid,
		}
		#[derive(Clone, Debug)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
		#[cfg_attr(feature = "query", derive(edgedb_derive::Queryable))]
		#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
		pub struct OutputIntersectionOfSet {
			#[cfg_attr(feature = "builder", builder(setter(into)))]
			pub id: uuid::Uuid,
		}
		#[derive(Clone, Debug)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
		#[cfg_attr(feature = "query", derive(edgedb_derive::Queryable))]
		#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
		pub struct OutputPointersSetPointersSet {
			# [cfg_attr (feature = "builder" , builder (default , setter (into , strip_option = (fallback = card_opt))))]
			pub card: Option<String>,
			#[cfg_attr(feature = "builder", builder(setter(into)))]
			pub name: String,
			# [cfg_attr (feature = "builder" , builder (default , setter (into , strip_option = (fallback = target_id_opt))))]
			pub target_id: Option<uuid::Uuid>,
			#[cfg_attr(feature = "builder", builder(setter(into)))]
			pub kind: String,
			# [cfg_attr (feature = "builder" , builder (default , setter (into , strip_option = (fallback = is_computed_opt))))]
			pub is_computed: Option<bool>,
			# [cfg_attr (feature = "builder" , builder (default , setter (into , strip_option = (fallback = is_readonly_opt))))]
			pub is_readonly: Option<bool>,
		}
		#[derive(Clone, Debug)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
		#[cfg_attr(feature = "query", derive(edgedb_derive::Queryable))]
		#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
		pub struct OutputPointersSet {
			# [cfg_attr (feature = "builder" , builder (default , setter (into , strip_option = (fallback = card_opt))))]
			pub card: Option<String>,
			#[cfg_attr(feature = "builder", builder(setter(into)))]
			pub name: String,
			# [cfg_attr (feature = "builder" , builder (default , setter (into , strip_option = (fallback = target_id_opt))))]
			pub target_id: Option<uuid::Uuid>,
			#[cfg_attr(feature = "builder", builder(setter(into)))]
			pub kind: String,
			#[cfg_attr(feature = "builder", builder(setter(into)))]
			pub is_exclusive: bool,
			# [cfg_attr (feature = "builder" , builder (default , setter (into , strip_option = (fallback = is_computed_opt))))]
			pub is_computed: Option<bool>,
			# [cfg_attr (feature = "builder" , builder (default , setter (into , strip_option = (fallback = is_readonly_opt))))]
			pub is_readonly: Option<bool>,
			#[cfg_attr(feature = "builder", builder(setter(into)))]
			pub has_default: bool,
			#[cfg_attr(feature = "builder", builder(default))]
			pub pointers: Vec<OutputPointersSetPointersSet>,
		}
		#[derive(Clone, Debug)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
		#[cfg_attr(feature = "query", derive(edgedb_derive::Queryable))]
		#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
		pub struct OutputExclusivesSet {
			# [cfg_attr (feature = "builder" , builder (default , setter (into , strip_option = (fallback = target_opt))))]
			pub target: Option<String>,
		}
		#[derive(Clone, Debug)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
		#[cfg_attr(feature = "query", derive(edgedb_derive::Queryable))]
		#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
		pub struct OutputBacklinksSet {
			#[cfg_attr(feature = "builder", builder(setter(into)))]
			pub card: String,
			#[cfg_attr(feature = "builder", builder(setter(into)))]
			pub name: String,
			#[cfg_attr(feature = "builder", builder(setter(into)))]
			pub stub: String,
			# [cfg_attr (feature = "builder" , builder (default , setter (into , strip_option = (fallback = target_id_opt))))]
			pub target_id: Option<uuid::Uuid>,
			#[cfg_attr(feature = "builder", builder(setter(into)))]
			pub kind: String,
			# [cfg_attr (feature = "builder" , builder (default , setter (into , strip_option = (fallback = is_exclusive_opt))))]
			pub is_exclusive: Option<bool>,
		}
		#[derive(Clone, Debug)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
		#[cfg_attr(feature = "query", derive(edgedb_derive::Queryable))]
		#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
		pub struct OutputBacklinkStubsArray {
			#[cfg_attr(feature = "builder", builder(setter(into)))]
			pub card: String,
			#[cfg_attr(feature = "builder", builder(setter(into)))]
			pub name: String,
			# [cfg_attr (feature = "builder" , builder (default , setter (into , strip_option = (fallback = target_id_opt))))]
			pub target_id: Option<uuid::Uuid>,
			#[cfg_attr(feature = "builder", builder(setter(into)))]
			pub kind: String,
			#[cfg_attr(feature = "builder", builder(setter(into)))]
			pub is_exclusive: bool,
		}
		#[derive(Clone, Debug)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
		#[cfg_attr(feature = "query", derive(edgedb_derive::Queryable))]
		#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
		pub struct OutputTupleElementsSet {
			#[cfg_attr(feature = "builder", builder(setter(into)))]
			pub target_id: uuid::Uuid,
			# [cfg_attr (feature = "builder" , builder (default , setter (into , strip_option = (fallback = name_opt))))]
			pub name: Option<String>,
		}
		#[derive(Clone, Debug)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
		#[cfg_attr(feature = "query", derive(edgedb_derive::Queryable))]
		#[cfg_attr(feature = "builder", derive(typed_builder::TypedBuilder))]
		pub struct Output {
			#[cfg_attr(feature = "builder", builder(setter(into)))]
			pub id: uuid::Uuid,
			#[cfg_attr(feature = "builder", builder(setter(into)))]
			pub name: String,
			# [cfg_attr (feature = "builder" , builder (default , setter (into , strip_option = (fallback = is_abstract_opt))))]
			pub is_abstract: Option<bool>,
			#[cfg_attr(feature = "builder", builder(setter(into)))]
			pub kind: String,
			# [cfg_attr (feature = "builder" , builder (default , setter (into , strip_option = (fallback = enum_values_opt))))]
			pub enum_values: Option<Vec<String>>,
			#[cfg_attr(feature = "builder", builder(setter(into)))]
			pub is_seq: bool,
			# [cfg_attr (feature = "builder" , builder (default , setter (into , strip_option = (fallback = material_id_opt))))]
			pub material_id: Option<uuid::Uuid>,
			#[cfg_attr(feature = "builder", builder(default))]
			pub bases: Vec<OutputBasesSet>,
			#[cfg_attr(feature = "builder", builder(default))]
			pub union_of: Vec<OutputUnionOfSet>,
			#[cfg_attr(feature = "builder", builder(default))]
			pub intersection_of: Vec<OutputIntersectionOfSet>,
			#[cfg_attr(feature = "builder", builder(default))]
			pub pointers: Vec<OutputPointersSet>,
			#[cfg_attr(feature = "builder", builder(default))]
			pub exclusives: Vec<OutputExclusivesSet>,
			#[cfg_attr(feature = "builder", builder(default))]
			pub backlinks: Vec<OutputBacklinksSet>,
			#[cfg_attr(feature = "builder", builder(setter(into)))]
			pub backlink_stubs: Vec<OutputBacklinkStubsArray>,
			# [cfg_attr (feature = "builder" , builder (default , setter (into , strip_option = (fallback = array_element_id_opt))))]
			pub array_element_id: Option<uuid::Uuid>,
			#[cfg_attr(feature = "builder", builder(default))]
			pub tuple_elements: Vec<OutputTupleElementsSet>,
			# [cfg_attr (feature = "builder" , builder (default , setter (into , strip_option = (fallback = multirange_element_id_opt))))]
			pub multirange_element_id: Option<uuid::Uuid>,
		}
	}
}
