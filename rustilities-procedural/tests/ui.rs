// SPDX-License-Identifier: MIT OR Apache-2.0

use trybuild::TestCases;

#[test]
fn ui() {
	let t = TestCases::new();
	t.compile_fail("tests/ui/**/*.rs");
}
