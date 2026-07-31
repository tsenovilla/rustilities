// SPDX-License-Identifier: MIT OR Apache-2.0

#![cfg(feature = "universal-test-builder")]

use rustilities::universal_test_builder::{AsyncBuilder, Builder, async_trait};
use rustilities_procedural::universal_test_builder;
use std::time::Duration;

struct Builder1;
impl Builder for Builder1 {
	type Args = ();
	type Output = u8;

	fn build(_: Self::Args) -> Self::Output {
		4
	}
}

struct Builder2;
#[async_trait]
impl AsyncBuilder for Builder2 {
	type Args = ();
	type Output = String;

	async fn async_build(_: Self::Args) -> Self::Output {
		tokio::time::sleep(Duration::from_secs(2)).await;
		"test".to_string()
	}
}

#[universal_test_builder({builder = Builder1}, {builder = Builder2, async_builder, and other random stuff})]
struct UniversalBuilder;

fn main(){}