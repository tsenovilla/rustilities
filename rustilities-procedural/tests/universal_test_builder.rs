// SPDX-License-Identifier: GPL-3.0

#![cfg(feature = "universal-test-builder")]

use rustilities::universal_test_builder::Builder;
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
impl Builder for Builder2 {
	type Args = ();
	type Output = String;

	fn build(_: Self::Args) -> Self::Output {
		"test".to_string()
	}
}

#[universal_test_builder({builder = Builder1}, {builder = Builder2})]
struct UniversalBuilder;

#[test]
fn universal_test_builder_works() {
	UniversalBuilder::default()
		.with_builder1(())
		.with_builder2(())
		.build()
		.execute(|context| {
			assert_eq!(context.builder1, Some(4));
			assert_eq!(context.builder2, Some("test".to_string()));
		});

	UniversalBuilder::default().with_builder1(()).build().execute(|context| {
		assert_eq!(context.builder1, Some(4));
		assert_eq!(context.builder2, None);
	});

	UniversalBuilder::default().with_builder2(()).build().execute(|context| {
		assert_eq!(context.builder1, None);
		assert_eq!(context.builder2, Some("test".to_string()));
	});

	UniversalBuilder::default().build().execute(|context| {
		assert_eq!(context.builder1, None);
		assert_eq!(context.builder2, None);
	});
}

#[tokio::test]
async fn universal_test_builder_works_with_async_execute() {
	UniversalBuilder::default()
		.with_builder1(())
		.with_builder2(())
		.build()
		.async_execute(|context| async move {
			tokio::time::sleep(Duration::from_secs(2)).await;
			assert_eq!(context.builder1, Some(4));
			assert_eq!(context.builder2, Some("test".to_string()));
		})
		.await;

	UniversalBuilder::default()
		.with_builder1(())
		.build()
		.async_execute(|context| async move {
			tokio::time::sleep(Duration::from_secs(2)).await;
			assert_eq!(context.builder1, Some(4));
			assert_eq!(context.builder2, None);
		})
		.await;

	UniversalBuilder::default()
		.with_builder2(())
		.build()
		.async_execute(|context| async move {
			tokio::time::sleep(Duration::from_secs(2)).await;
			assert_eq!(context.builder1, None);
			assert_eq!(context.builder2, Some("test".to_string()));
		})
		.await;

	UniversalBuilder::default()
		.build()
		.async_execute(|context| async move {
			tokio::time::sleep(Duration::from_secs(2)).await;
			assert_eq!(context.builder1, None);
			assert_eq!(context.builder2, None);
		})
		.await;
}
