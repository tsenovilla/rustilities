// SPDX-License-Identifier: MIT OR Apache-2.0

#![cfg(feature = "universal-test-builder")]

use rustilities::universal_test_builder::Builder;
use rustilities_procedural::universal_test_builder;

struct Builder1;
impl Builder for Builder1 {
	type Args = ();
	type Output = u8;

	fn build(_: Self::Args) -> Self::Output {
		4
	}
}

#[universal_test_builder({builder = Builder1}, {builder = 43, async_builder})]
struct UniversalBuilder;

fn main(){}