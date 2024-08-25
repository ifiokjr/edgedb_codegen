use edgedb_codegen_macros::edgedb_query_raw;

fn main() {
	edgedb_query_raw!(example, query: "select bool", "more stuff", that is, "not allowed");
}
