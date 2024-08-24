use edgedb_codegen_macros::edgedb_query;

fn main() {
	edgedb_query!(example_module, invalid);
}
