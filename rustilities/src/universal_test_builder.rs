// SPDX-License-Identifier: GPL-3.0

pub mod builders;

/// Re-export of async_trait to be used with [`AsyncBuilder`]
pub use async_trait::async_trait;

pub trait Builder {
	type Output;
	type Args;

	fn build(args: Self::Args) -> Self::Output;

	/// Called via Drop on the context after the test executes.
	/// Override to perform cleanup. Default is a no-op.
	fn on_drop(_output: Self::Output) {}
}

#[async_trait]
pub trait AsyncBuilder {
	type Output;
	type Args;

	async fn async_build(args: Self::Args) -> Self::Output;

	/// Called via Drop on the context after the test executes.
	/// Sync even for async builders because Drop cannot be async.
	fn on_drop(_output: Self::Output) {}
}

pub use rustilities_procedural::universal_test_builder;

use builders::{RustupComponent, TempDir, TempRustProject};

#[universal_test_builder({builder = TempDir}, {builder = TempRustProject}, {builder = RustupComponent})]
pub struct UniversalTestBuilder;
