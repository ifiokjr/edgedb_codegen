use edgedb_macros::edgedb_query;

fn main() {
	edgedb_query!(example_module, "invalid");
}
