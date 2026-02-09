// SPDX-License-Identifier: GPL-3.0

/// Re-export of async_trait to be used with [`AsyncBuilder`]
pub use async_trait::async_trait;

pub trait Builder {
	type Output;
	type Args;

	fn build(args: Self::Args) -> Self::Output;
}

#[async_trait]
pub trait AsyncBuilder {
	type Output;
	type Args;

	async fn async_build(args: Self::Args) -> Self::Output;
}


pub use rustilities_procedural::universal_test_builder;