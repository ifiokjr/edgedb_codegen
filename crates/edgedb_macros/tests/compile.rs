use std::thread;

use rstest::fixture;
use rstest::rstest;
use trybuild::TestCases;

#[fixture]
fn t() -> TestCases {
	TestCases::new()
}

#[fixture]
pub fn testname() -> String {
	thread::current().name().unwrap().to_string()
}

#[rstest]
#[case::no_args()]
#[case::one_arg()]
#[case::incorrect_args()]
#[case::invalid_query()]
fn compilation_error(t: TestCases, testname: String) {
	let path = format!("tests/ui/{testname}.rs");
	t.compile_fail(&path);
}
