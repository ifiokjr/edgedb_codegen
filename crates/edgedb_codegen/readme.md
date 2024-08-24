# `edgedb_codegen`

When working with `edgedb` you often need to write queries and also provide the typed for both the input and output. Your code is only checked at runtime which increases the risk of bugs and unforseen errors.

Fortunately, `edgedb` has a query language that is typed and can be converted into types and queried for correctness at compile time.

```rust
use edgedb_codegen::edgedb_query;
use edgedb_errors::Result;

edgedb_query!(get_users, "select User {**}");

async fn runner() -> Result<()> {
	let client = create_client().await?;
	let input = get_users::Input::builder()
		.custom("This is a custom field")
		.build();
	// For queries the following code can be used.
	let output = get_users::query(&client).await?;

	// To use in transactions the following code can be used:
	client
		.transaction(|mut txn| {
			async move {
				let output = get_users::transaction(&mut txn).await?;
				Ok(())
			}
		})
		.await?;

	Ok(())
}
```

This macro will generate the following code:
