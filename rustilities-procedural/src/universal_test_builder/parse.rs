// SPDX-License-Identifier: GPL-3.0

mod keywords {
	syn::custom_keyword!(state_index);
	syn::custom_keyword!(build_method);
	syn::custom_keyword!(generates);
	syn::custom_keyword!(extra_args);
}

#[cfg(test)]
mod tests;

use syn::{
	Error, Ident, LitInt, Result, Token, Type,
	parse::{Parse, ParseStream},
	punctuated::Punctuated,
};

#[cfg_attr(test, derive(Debug))]
pub(crate) struct UniversalTestBuilderDefinition {
	pub(crate) state_index: u128,
	pub(crate) build_method: Ident,
	pub(crate) generates: Type,
	pub(crate) extra_args: Option<Punctuated<Type, Token![,]>>,
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
		let mut state_index = None;
		let mut build_method = None;
		let mut generates = None;
		let mut extra_args = None;

		let initial_span = input.span();

		while !input.is_empty() {
			if input.peek(keywords::state_index) {
				input
					.parse::<keywords::state_index>()
					.expect("Lookahead guarantees this is Ok; qed;");
				input.parse::<Token![=]>()?;
				let lit: LitInt = input.parse()?;
				state_index = Some(lit.base10_parse()?);
			} else if input.peek(keywords::build_method) {
				input
					.parse::<keywords::build_method>()
					.expect("Lookahead guarantees this is Ok; qed;");
				input.parse::<Token![=]>()?;
				build_method = Some(input.parse()?);
			} else if input.peek(keywords::generates) {
				input
					.parse::<keywords::generates>()
					.expect("Lookahead guarantees this is Ok; qed;");
				input.parse::<Token![=]>()?;
				generates = Some(input.parse()?);
			} else if input.peek(keywords::extra_args) {
				input
					.parse::<keywords::extra_args>()
					.expect("Lookahead guarantees this is Ok; qed;");
				input.parse::<Token![=]>()?;
				let content;
				syn::parenthesized!(content in input);
				extra_args = Some(content.parse_terminated(Type::parse, Token![,])?);
			} else {
				return Err(input.error("unexpected token in universal_test_builder"));
			}

			if input.peek(Token![,]) {
				input.parse::<Token![,]>().expect("Lookahead guarantees this is Ok; qed;");
			}
		}

		match (state_index, build_method, generates, extra_args.clone()) {
			(Some(state_index), Some(build_method), Some(generates), _) =>
				Ok(Self { state_index, build_method, generates, extra_args }),
			(None, _, _, _) =>
				Err(Error::new(initial_span, "Missing state_index in universal_test_builder")),
			(_, None, _, _) =>
				Err(Error::new(initial_span, "Missing build_method in universal_test_builder")),
			(_, _, None, _) =>
				Err(Error::new(initial_span, "Missing generates in universal_test_builder")),
		}
	}
}

#[cfg_attr(test, derive(Debug))]
pub(crate) struct UniversalTestBuilderInput {
	pub(crate) blocks: Punctuated<UniversalTestBuilderDefinition, Token![,]>,
}

impl Parse for UniversalTestBuilderInput {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let blocks = Punctuated::parse_terminated(input)?;
		Ok(Self { blocks })
	}
}
