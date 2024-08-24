use rstest::fixture;
use rstest::rstest;
use trybuild::TestCases;

#[fixture]
fn t() -> TestCases {
	TestCases::new()
}

#[rstest]
#[case::no_args("no_args")]
#[case::one_arg("one_arg")]
#[case::incorrect_args("incorrect_args")]
#[case::invalid_query("invalid_query")]
fn compilation_error(t: TestCases, #[case] name: &str) {
	let path = format!("tests/ui/{name}.rs");
	t.compile_fail(&path);
}
