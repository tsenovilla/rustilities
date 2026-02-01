// SPDX-License-Identifier: GPL-3.0

mod parse;

use proc_macro::TokenStream;

pub(crate) fn universal_test_builder(attrs: TokenStream, item: TokenStream) -> TokenStream {
	TokenStream::new()
}
