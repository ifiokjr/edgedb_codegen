#![cfg(all(feature = "query", feature = "serde"))]

use edgedb_codegen::edgedb_query;
use edgedb_codegen_core::Result;
use edgedb_tokio::create_client;

#[tokio::test]
pub async fn simple_query_with_input() -> Result<()> {
	let client = create_client().await?;
	let input = simple::Input::builder()
		.custom("This is a custom field")
		.build();
	let output = simple::query(&client, &input).await?;

	edgedb_query!(
		simple,
		r#"select {hello := "world", custom := <str>$custom }"#
	);

	insta::assert_ron_snapshot!(output, @r###"
 Output(
   hello: "world",
   custom: "This is a custom field",
 )
 "###);

	Ok(())
}

#[tokio::test]
pub async fn empty_set_query() -> Result<()> {
	let client = create_client().await?;
	let output = empty::query(&client).await?;

	edgedb_query!(empty, r#"select <int64>{}"#);

	insta::assert_ron_snapshot!(output, @"None");

	Ok(())
}
