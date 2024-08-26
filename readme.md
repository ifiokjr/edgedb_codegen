# `edgedb_codegen`

<br />

> Generate fully typed rust code from your EdgeDB schema and inline queries.

<br />

[![Crate][crate-image]][crate-link] [![Docs][docs-image]][docs-link] [![Status][ci-status-image]][ci-status-link] [![Unlicense][unlicense-image]][unlicense-link]

## Installation

To install the `edgedb_codegen` crate you can use the following command.

```bash
cargo add edgedb_codegen
```

Or directly add the following to your `Cargo.toml` file.

```toml
edgedb_codegen = "0.1.0" # replace with the latest version
```

Follow the [Quickstart Guide](https://docs.edgedb.com/get-started/quickstart) to make sure your edgedb instance is running. The macro relies on the running `edgedb` instance to parse the output of the provided query string.

## Usage

When working with `edgedb` you often need to write queries and also provide the typed for both the input and output. Your code is only checked at runtime which increases the risk of bugs and errors.

Fortunately, `edgedb` has a query language that is typed and can be converted into types and queried for correctness at compile time.

### Inline Queries

```rust
use edgedb_codegen::edgedb_query;
use edgedb_errors::Error;
use edgedb_tokio::create_client;

// Creates a module called `simple` with a function called `query` and structs
// for the `Input` and `Output`.
edgedb_query!(
	simple,
	"select { hello := \"world\", custom := <str>$custom }"
);

#[tokio::main]
async fn main() -> Result<(), Error> {
	let client = create_client().await?;
	let input = simple::Input::builder().custom("custom").build();

	// For queries the following code can be used.
	let output = simple::query(&client, &input).await?;

	Ok(())
}
```

The macro above generates the following code:

```rust
pub mod simple {
	use ::edgedb_codegen::exports as e;
	#[doc = r" Execute the desired query."]
	#[cfg(feature = "query")]
	pub async fn query(
		client: &e::edgedb_tokio::Client,
		props: &Input,
	) -> core::result::Result<Output, e::edgedb_errors::Error> {
		client.query_required_single(QUERY, props).await
	}
	#[doc = r" Compose the query as part of a larger transaction."]
	#[cfg(feature = "query")]
	pub async fn transaction(
		conn: &mut e::edgedb_tokio::Transaction,
		props: &Input,
	) -> core::result::Result<Output, e::edgedb_errors::Error> {
		conn.query_required_single(QUERY, props).await
	}
	#[derive(Clone, Debug, e :: typed_builder :: TypedBuilder)]
	#[cfg_attr(feature = "serde", derive(e::serde::Serialize, e::serde::Deserialize))]
	#[cfg_attr(feature = "query", derive(e::edgedb_derive::Queryable))]
	pub struct Input {
		#[builder(setter(into))]
		pub custom: String,
	}
	impl e::edgedb_protocol::query_arg::QueryArgs for Input {
		fn encode(
			&self,
			encoder: &mut e::edgedb_protocol::query_arg::Encoder,
		) -> core::result::Result<(), e::edgedb_errors::Error> {
			let map = e::edgedb_protocol::named_args! { "custom" => self . custom . clone () , };
			map.encode(encoder)
		}
	}
	#[derive(Clone, Debug, e :: typed_builder :: TypedBuilder)]
	#[cfg_attr(feature = "serde", derive(e::serde::Serialize, e::serde::Deserialize))]
	#[cfg_attr(feature = "query", derive(e::edgedb_derive::Queryable))]
	pub struct Output {
		#[builder(setter(into))]
		pub hello: String,
		#[builder(setter(into))]
		pub custom: String,
	}
	#[doc = r" The original query string provided to the macro. Can be reused in your codebase."]
	pub const QUERY: &str = "select { hello := \"world\", custom := <str>$custom }";
}
```

### Query Files

Define a query file in the `queries` directory of your crate called `select_user.edgeql`.

```edgeql
# queries/select_user.edgeql

select User {
  name,
  bio,
  slug,
} filter .slug = <str>$slug;
```

Then use the `edgedb_query` macro to import the query.

```rust
use edgedb_codegen::edgedb_query;
use edgedb_errors::Error;
use edgedb_tokio::create_client;

// Creates a module called `select_user` with public functions `transaction` and
// `query` as well as structs for the `Input` and `Output`.
edgedb_query!(select_user);

#[tokio::main]
async fn main() -> Result<(), Error> {
	let client = create_client().await?;

	// Generated code can be run inside a transaction.
	let result = client
		.transaction(|mut txn| {
			async move {
				let input = select_user::Input::builder().slug("test").build();
				let output = select_user::transaction(&mut txn, &input).await?;
				Ok(output)
			}
		})
		.await?;

	Ok(())
}
```

## Future Work

This crate is still in early development and there are several features that are not yet implemented.

### Missing Types

Currently the following types are not supported:

- `enum` - currently all enums are represented as strings.
- `MultiRange` - The macro will panic if a multirange is used.

#### `enum`

Currently all enums are represented as strings.

In order to support full enum generation the `edgedb-protocol` crate needs to be [updated](https://github.com/edgedb/edgedb-rust/issues/336) to use the [binary protocol 2.0](https://docs.edgedb.com/database/reference/protocol/typedesc#enumeration-type-descriptor). In the current 1.0 version the enum descriptors are returned without the name property.

Once [this is](https://github.com/edgedb/edgedb-rust/issues/336) implemented the macro will be able to generate the correct code.

However end users probably don't want multiple enums for each generated query module as this would break sharing. To get around this, there should be a macro for generating the shared types used by all other.

```rust,ignore
// lib.rs
use edgedb_codegen::generate_shared_types;

generate_shared_types!(); // exports the shared types to the `edb` module.
```

#### `MultiRange`

These are not currently exported by the `edgedb-protocol` so should be added in a PR to the `edgedb-protocol` crate, if they are still supported in the new protocol.

### Configuration

Currently everything is hardcoded and the macro is not configurable.

The following configuration options should be added:

- Name of input struct (optional) - `Input` by default.
- Name of output struct (optional) - `Output` by default.
- Name of query function (optional) - `query` by default.
- Name of transaction function (optional) - `transaction`by default.
- Default location of queries (optional) - `queries` by default.
- Default crate export name for shared types (optional) - `edb` by default.
- Default `edgedb` instance (optional) - `$EDGEDB_INSTANCE` by default.
- Default `edgedb` branch (optional) - `$EDGEDB_BRANCH` by default.

Probably these should be read from the `Cargo.toml` file and parsed manually to prevent slowdowns from parsing the file.

### LSP parsing

Currently the macro depends on having a running edgedb instance to parse the query string.

Once an LSP is created for edgedb it would make sense to switch from using string to using inline edgedb queries.

```rust,ignore
use edgedb_codegen::edgedb_query;

edgedb_query!(
	example,
	select User {**}
);
```

### CLI

Create a `edgedb_codegen_cli` crate which supports generating the typed code into rust files rather than inline queries. This is useful for larger projects to prevent constantly compiling the queries on every change / build.

## Contributing

[`devenv`](https://devenv.sh/) is used to provide a reproducible development environment for this project. Follow the [getting started instructions](https://devenv.sh/getting-started/).

To automatically load the environment you should [install direnv](https://devenv.sh/automatic-shell-activation/) and then load the `direnv`.

```bash
# The security mechanism didn't allow to load the `.envrc`.
# Since we trust it, let's allow it execution.
direnv allow .
```

At this point you should see the `nix` commands available in your terminal.

Run the following commands to install all the required dependencies.

```bash
install:all
```

This installs all the cargo binaries locally so you don't need to worry about polluting your global namespace.

### Upgrading `devenv`

If you have an outdated version of `devenv` you can update it by running the following commands. If you have an easier way, please create a PR and I'll update these docs.

```bash
nix profile list # find the index of the nxi package
nix profile remove <index>
nix profile install --accept-flake-config github:cachix/devenv/<version>
```

### Editor Setup

To setup recommended configuration for your favorite editor run the following commands.

```bash
setup:vscode # Setup vscode
```

## License

Unlicense, see the [license](./license) file.

[crate-image]: https://img.shields.io/crates/v/edgedb_codegen.svg
[crate-link]: https://crates.io/crates/edgedb_codegen
[docs-image]: https://docs.rs/edgedb_codegen/badge.svg
[docs-link]: https://docs.rs/edgedb_codegen/
[ci-status-image]: https://github.com/ifiokjr/edgedb_codegen/workflows/ci/badge.svg
[ci-status-link]: https://github.com/ifiokjr/edgedb_codegen/actions?query=workflow:ci
[unlicense-image]: https://img.shields.io/badge/license-Unlicence-blue.svg
[unlicense-link]: https://opensource.org/license/unlicense
