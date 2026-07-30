// SPDX-License-Identifier: GPL-3.0

mod expand;
pub(crate) mod parse;

use parse::UniversalTestBuilderInput;
use proc_macro::TokenStream;
use syn::ItemStruct;

pub(crate) fn universal_test_builder(attrs: TokenStream, item: TokenStream) -> TokenStream {
	let attrs = syn::parse_macro_input!(attrs as UniversalTestBuilderInput);
	let struct_ = syn::parse_macro_input!(item as ItemStruct);

	let struct_name = match parse::parse_unit_struct_ident(struct_) {
		Ok(struct_name) => struct_name,
		Err(err) => return err.to_compile_error().into(),
	};

	expand::expand_universal_test_builder(struct_name, attrs).into()
}
