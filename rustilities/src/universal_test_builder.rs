// SPDX-License-Identifier: MIT OR Apache-2.0

//! Composable test harnesses: implement [`Builder`] (or [`AsyncBuilder`]) once per
//! resource, then let [`universal_test_builder`] assemble any subset of them into a
//! fluent, self-cleaning test context.
//!
//! ```
//! use rustilities::universal_test_builder::{builders::temp_dir::TempDir, universal_test_builder, Builder};
//!
//! #[universal_test_builder({builder = TempDir})]
//! struct MyTestBuilder;
//!
//! MyTestBuilder::default()
//!     .with_temp_dir(()) // request only what the test needs; the rest is never built
//!     .build()
//!     .execute(|context| {
//!         let dir = context.temp_dir(); // generated accessor
//!         assert!(dir.path().is_dir());
//!     });
//! // The context dropped here: every built resource ran its cleanup.
//! ```
//!
//! Builders declared with the `async_builder` flag are driven through
//! `async_build()`/`async_execute()` instead; see [`AsyncBuilder`].

pub mod builders;

/// Re-export of async_trait to be used with [`AsyncBuilder`]
pub use async_trait::async_trait;

/// A unit of test infrastructure: how to build a resource from its `Args`, and how to
/// clean it up once the test is over.
pub trait Builder {
	/// The resource handed to the test through the generated context.
	type Output;
	/// Configuration the test supplies through the generated `with_*` method.
	type Args;

	/// Builds the resource.
	fn build(args: Self::Args) -> Self::Output;

	/// Called via Drop on the context after the test executes.
	/// Override to perform cleanup for resources that do not clean themselves up when
	/// dropped. Default is a no-op.
	fn on_drop(_output: Self::Output) {}
}

/// The async counterpart of [`Builder`], for resources whose construction must await.
/// Only runnable through the generated `async_build()`.
#[async_trait]
pub trait AsyncBuilder {
	/// The resource handed to the test through the generated context.
	type Output;
	/// Configuration the test supplies through the generated `with_*` method.
	type Args;

	/// Builds the resource.
	async fn async_build(args: Self::Args) -> Self::Output;

	/// Called via Drop on the context after the test executes.
	/// Sync even for async builders because Drop cannot be async.
	fn on_drop(_output: Self::Output) {}
}

pub use rustilities_procedural::universal_test_builder;

#[cfg(test)]
use builders::{
	rustup_component::RustupComponent, temp_dir::TempDir, temp_rust_project::TempRustProject,
};

#[cfg(test)]
#[universal_test_builder({builder = TempDir}, {builder = TempRustProject}, {builder = RustupComponent})]
pub struct UniversalTestBuilder;
