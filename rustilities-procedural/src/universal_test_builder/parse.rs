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
	Error, Fields, Ident, ItemStruct, Result, Token,
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

		// Validate: all builders must be unique
		let mut seen = HashSet::new();
		for block in &blocks {
			let name = block.builder.to_string();
			if !seen.insert(name) {
				return Err(Error::new(
					block.builder.span(),
					format!("duplicate builder '{}' in universal_test_builder", block.builder),
				));
			}
		}

		Ok(Self { blocks })
	}
}

/// Converts PascalCase to snake_case, handling consecutive uppercase (acronyms).
/// E.g., `MyBuilder` -> `my_builder`, `HTTPBuilder` -> `http_builder`
fn to_snake_case(s: &str) -> String {
	let mut result = String::new();
	let chars: Vec<char> = s.chars().collect();
	for (i, &c) in chars.iter().enumerate() {
		if c.is_uppercase() {
			if i > 0 {
				let prev = chars[i - 1];
				let next = chars.get(i + 1);
				if prev.is_lowercase() ||
					(prev.is_uppercase() && next.is_some_and(|n| n.is_lowercase()))
				{
					result.push('_');
				}
			}
			result.push(c.to_ascii_lowercase());
		} else {
			result.push(c);
		}
	}
	result
}

pub(crate) struct DerivedStreams {
	// The builder struct name.
	pub(crate) builders: Vec<Ident>,
	// The transition function to include a builder
	pub(crate) builders_transition_functions: Vec<Ident>,
	// The builder name in snake_case
	pub(crate) builders_snake_case: Vec<Ident>,
	// Whether this builder is async.
	pub(crate) is_async: Vec<bool>,
	// The args this builder needs.
	pub(crate) builder_args: Vec<TokenStream>,
	// The output produced by this builder.
	pub(crate) builder_outputs: Vec<TokenStream>,
}

impl UniversalTestBuilderInput {
	pub(crate) fn derive_streams(self) -> DerivedStreams {
		let mut derived = DerivedStreams {
			builders: Vec::new(),
			builders_transition_functions: Vec::new(),
			builders_snake_case: Vec::new(),
			is_async: Vec::new(),
			builder_args: Vec::new(),
			builder_outputs: Vec::new(),
		};

		for block in self.blocks.iter() {
			let snake = to_snake_case(&block.builder.to_string());
			derived.builders.push(block.builder.clone());
			derived
				.builders_transition_functions
				.push(quote::format_ident!("with_{}", snake));
			derived.builders_snake_case.push(quote::format_ident!("{}", snake));
			derived.is_async.push(block.async_builder);

			let builder = &block.builder;
			if block.async_builder {
				derived.builder_args.push(
					quote::quote! { <#builder as rustilities::universal_test_builder::AsyncBuilder>::Args },
				);
				derived.builder_outputs.push(
					quote::quote! { <#builder as rustilities::universal_test_builder::AsyncBuilder>::Output },
				);
			} else {
				derived.builder_args.push(
					quote::quote! { <#builder as rustilities::universal_test_builder::Builder>::Args },
				);
				derived.builder_outputs.push(
					quote::quote! { <#builder as rustilities::universal_test_builder::Builder>::Output },
				);
			};
		}

		derived
	}
}

pub(crate) fn parse_unit_struct_ident(item: ItemStruct) -> Result<Ident> {
	if matches!(item.fields, Fields::Unit) {
		Ok(item.ident)
	} else {
		Err(Error::new_spanned(&item, "expected a unit struct"))
	}
}
