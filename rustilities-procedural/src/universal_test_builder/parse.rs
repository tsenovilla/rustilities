// SPDX-License-Identifier: GPL-3.0

mod keywords {
	syn::custom_keyword!(builder);
	syn::custom_keyword!(async_builder);
}

#[cfg(test)]
mod tests;

use std::collections::HashSet;

use proc_macro2::TokenStream;
use syn::{
	Error, Ident, Index, Result, Token,
	parse::{Parse, ParseStream},
	punctuated::Punctuated,
};

#[cfg_attr(test, derive(Debug))]
pub(crate) struct UniversalTestBuilderDefinition {
	pub(crate) builder: Ident,
	pub(crate) async_builder: bool,
}

impl Parse for UniversalTestBuilderDefinition {
	fn parse(input: ParseStream) -> Result<Self> {
		let content;
		syn::braced! {content in input};
		Self::parse_fields(&content)
	}
}

impl UniversalTestBuilderDefinition {
	fn parse_fields(input: ParseStream) -> Result<Self> {
		let mut builder: Option<Ident> = None;
		let mut async_builder = false;

		let initial_span = input.span();

		while !input.is_empty() {
			if input.peek(keywords::builder) {
				input
					.parse::<keywords::builder>()
					.expect("Lookahead guarantees this is Ok; qed;");
				input.parse::<Token![=]>()?;
				builder = Some(input.parse()?);
			} else if input.peek(keywords::async_builder) {
				input
					.parse::<keywords::async_builder>()
					.expect("Lookahead guarantees this is Ok; qed;");
				async_builder = true;
			} else {
				return Err(input.error("unexpected token in universal_test_builder"));
			}

			if input.peek(Token![,]) {
				input.parse::<Token![,]>().expect("Lookahead guarantees this is Ok; qed;");
			}
		}

		match builder {
			Some(builder) => Ok(Self { builder, async_builder }),
			None => Err(Error::new(initial_span, "Missing builder in universal_test_builder")),
		}
	}
}

#[cfg_attr(test, derive(Debug))]
pub(crate) struct UniversalTestBuilderInput {
	pub(crate) blocks: Punctuated<UniversalTestBuilderDefinition, Token![,]>,
}

impl Parse for UniversalTestBuilderInput {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let blocks: Punctuated<UniversalTestBuilderDefinition, Token![,]> =
			Punctuated::parse_terminated(input)?;

		if blocks.len() > 128 {
			return Err(Error::new(
				input.span(),
				"universal_test_builder supports at most 128 builders",
			));
		}

		// Validate: all builders must be unique
		let mut seen = HashSet::new();
		for block in &blocks {
			let name = block.builder.to_string();
			if !seen.insert(name.clone()) {
				return Err(Error::new(
					block.builder.span(),
					format!("duplicate builder '{}' in universal_test_builder", name),
				));
			}
		}

		Ok(Self { blocks })
	}
}

/// Converts PascalCase to snake_case.
/// E.g., `MyBuilder` -> `my_builder`
fn to_snake_case(s: &str) -> String {
	let mut result = String::new();
	for (i, c) in s.chars().enumerate() {
		if c.is_uppercase() {
			if i > 0 {
				result.push('_');
			}
			result.push(c.to_ascii_lowercase());
		} else {
			result.push(c);
		}
	}
	result
}

impl UniversalTestBuilderInput {
	/// Returns 6 vectors derived from the builders:
	/// 1. `indices`: Position indices as `syn::Index`
	/// 2. `builders`: The original builder Idents
	/// 3. `builders_snake_case`: Builder names in snake_case as Idents (e.g., `MyBuilder` -> `my_builder`)
	/// 4. `is_async`: Whether each builder is async
	/// 5. `building_args`: TokenStreams like `<Builder as Builder>::Args` or
	///    `<Builder as AsyncBuilder>::Args`
	/// 6. `builder_outputs`: TokenStreams like `<Builder as Builder>::Output` or
	///    `<Builder as AsyncBuilder>::Output`
	pub(crate) fn derived_vecs(
		self,
	) -> (Vec<Index>, Vec<Ident>, Vec<Ident>, Vec<bool>, Vec<TokenStream>, Vec<TokenStream>) {
		let mut indices = Vec::new();
		let mut builders = Vec::new();
		let mut builders_snake_case = Vec::new();
		let mut is_async = Vec::new();
		let mut building_args = Vec::new();
		let mut builder_outputs = Vec::new();

		for (index, block) in self.blocks.iter().enumerate() {
			indices.push(Index::from(index + 1));
			builders.push(block.builder.clone());
			builders_snake_case
				.push(quote::format_ident!("{}", to_snake_case(&block.builder.to_string())));
			is_async.push(block.async_builder);

			let builder = &block.builder;
			if block.async_builder {
				building_args.push(
					quote::quote! { <#builder as rustilities::universal_test_builder::AsyncBuilder>::Args },
				);
				builder_outputs.push(
					quote::quote! { <#builder as rustilities::universal_test_builder::AsyncBuilder>::Output },
				);
			} else {
				building_args.push(
					quote::quote! { <#builder as rustilities::universal_test_builder::Builder>::Args },
				);
				builder_outputs.push(
					quote::quote! { <#builder as rustilities::universal_test_builder::Builder>::Output },
				);
			};
		}

		(indices, builders, builders_snake_case, is_async, building_args, builder_outputs)
	}
}
