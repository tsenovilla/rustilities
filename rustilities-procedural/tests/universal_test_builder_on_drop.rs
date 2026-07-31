// SPDX-License-Identifier: MIT OR Apache-2.0

#![cfg(feature = "universal-test-builder")]

use rustilities::universal_test_builder::Builder;
use rustilities_procedural::universal_test_builder;
use std::sync::atomic::{AtomicU32, Ordering};

static ON_DROP_COUNTER: AtomicU32 = AtomicU32::new(0);

struct CountingBuilder;
impl Builder for CountingBuilder {
	type Args = ();
	type Output = ();

	fn build(_: Self::Args) -> Self::Output {}

	fn on_drop(_output: Self::Output) {
		ON_DROP_COUNTER.fetch_add(1, Ordering::SeqCst);
	}
}

struct NoOpBuilder;
impl Builder for NoOpBuilder {
	type Args = ();
	type Output = String;

	fn build(_: Self::Args) -> Self::Output {
		"hello".to_string()
	}
}

#[universal_test_builder({builder = CountingBuilder}, {builder = NoOpBuilder})]
struct OnDropTestBuilder;

#[test]
fn on_drop_is_called_after_execute() {
	ON_DROP_COUNTER.store(0, Ordering::SeqCst);

	OnDropTestBuilder::default()
		.with_counting_builder(())
		.with_no_op_builder(())
		.build()
		.execute(|context| {
			assert_eq!(ON_DROP_COUNTER.load(Ordering::SeqCst), 0);
			assert_eq!(context.counting_builder, Some(()));
			assert_eq!(context.no_op_builder, Some("hello".to_string()));
		});

	assert_eq!(ON_DROP_COUNTER.load(Ordering::SeqCst), 1);
}

#[test]
fn on_drop_not_called_if_builder_not_used() {
	ON_DROP_COUNTER.store(0, Ordering::SeqCst);

	OnDropTestBuilder::default().with_no_op_builder(()).build().execute(|context| {
		assert_eq!(context.counting_builder, None);
		assert_eq!(context.no_op_builder, Some("hello".to_string()));
	});

	assert_eq!(ON_DROP_COUNTER.load(Ordering::SeqCst), 0);
}

#[test]
fn on_drop_called_even_on_panic() {
	ON_DROP_COUNTER.store(0, Ordering::SeqCst);

	let result = std::panic::catch_unwind(|| {
		OnDropTestBuilder::default()
			.with_counting_builder(())
			.build()
			.execute(|_context| {
				panic!("intentional test panic");
			});
	});

	assert!(result.is_err());
	assert_eq!(ON_DROP_COUNTER.load(Ordering::SeqCst), 1);
}
