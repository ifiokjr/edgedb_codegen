use edgedb_codegen_macros::edgedb_query_raw;

fn main() {
	edgedb_query_raw!(insert_user, file: "/absolute/path/to/queries/insert_user.edgeql");
}
