use edgedb_macros::edgedb_query;
use edgedb_tokio::create_client;

#[tokio::test]
pub async fn simple_query() -> anyhow::Result<()> {
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
pub async fn empty_query() -> anyhow::Result<()> {
	let client = create_client().await?;
	let output = empty::query(&client).await?;

	edgedb_query!(empty, r#"select <uuid>"100""#);

	insta::assert_ron_snapshot!(output, @"");

	Ok(())
}
