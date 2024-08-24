use std::thread;

use rstest::fixture;
use trybuild::TestCases;

#[fixture]
fn t() -> TestCases {
	TestCases::new()
}

#[fixture]
pub fn testname() -> String {
	thread::current()
		.name()
		.unwrap()
		.split("::")
		.last()
		.unwrap()
		.to_string()
}

#[rustversion::attr(not(nightly), ignore = "requires nightly")]
#[cfg_attr(miri, ignore = "incompatible with miri")]
#[test]
fn macro_compilation_errors() {
	let t = TestCases::new();
	t.compile_fail("tests/macros/*.rs");
}

#[rustversion::attr(not(nightly), ignore = "requires nightly")]
#[cfg_attr(miri, ignore = "incompatible with miri")]
#[test]
fn codegen_compilation_pass() {
	let t = TestCases::new();
	t.pass("tests/codegen/*.rs");
}
