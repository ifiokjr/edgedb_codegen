use edgedb_codegen::edgedb_query;

fn main() {
	edgedb_query!(example, "select <bool>$0");
}
