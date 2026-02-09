// SPDX-License-Identifier: GPL-3.0

#[cfg(feature = "universal-test-builder")]
mod universal_test_builder;

#[allow(unused_imports)]
use proc_macro::TokenStream;

/// NOTE: The sync `build()` method cannot run async builders — they will produce `None` in the
/// context. Use `async_build()` when async builders are involved.
#[cfg(feature = "universal-test-builder")]
#[proc_macro_attribute]
pub fn universal_test_builder(attrs: TokenStream, item: TokenStream) -> TokenStream {
	universal_test_builder::universal_test_builder(attrs, item)
}
