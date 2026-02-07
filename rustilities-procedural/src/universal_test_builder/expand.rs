// SPDX-License-Identifier: GPL-3.0

use crate::universal_test_builder::parse::{DerivedStreams, UniversalTestBuilderInput};
use proc_macro2::TokenStream;
use syn::Ident;

pub(crate) fn expand_universal_test_builder(
	universal_test_builder_name: Ident,
	builders: UniversalTestBuilderInput,
) -> TokenStream {
	let DerivedStreams {
		builders,
		builders_transition_functions,
		builders_snake_case,
		builder_args,
		builder_outputs,
		is_async,
	} = builders.derive_streams();

	let async_methods_needed = is_async.iter().any(|&x| x);

	let universal_test_builder_args = quote::quote! {
		#[derive(Default)]
		struct UniversalTestBuilderArgs {
			#(
				#builders_snake_case: Option<#builder_args>,
			)*
		}
	};

	let universal_test_builder_output = quote::quote! {
		struct UniversalTestBuilderContext {
			#(
				#builders_snake_case: Option<#builder_outputs>,
			)*
		}
	};

	let universal_test_builder = quote::quote! {
		pub struct #universal_test_builder_name {
			_args: UniversalTestBuilderArgs,
		}

		impl Default for #universal_test_builder_name {
			fn default() -> Self {
				Self {
					_args: UniversalTestBuilderArgs::default(),
				}
			}
		}
	};

	let transition_functions = quote::quote! {
		impl #universal_test_builder_name {
			#(
				pub fn #builders_transition_functions(self, args: #builder_args) -> Self {
					let mut updated_args = self._args;
					updated_args.#builders_snake_case = Some(args);
					Self {
						_args: updated_args,
					}
				}
			)*
		}
	};

	let build_methods = if async_methods_needed {
		quote::quote! {
			// As each builder implements either `Builder` or `AsyncBuilder`,
			// this inner macro ensures only the valid calls are expanded.
			macro_rules! expand_valid_call {
				(build, $context_ident:ident, $builder:ident, true) => {
					None
				};
				(build, $context_ident:ident, $builder:ident, false) => {
					$context_ident.map(|args| $builder::build(args))
				};
				(async_build, $context_ident:ident, $builder:ident, true) => {
					match $context_ident {
						Some(args) => Some($builder::async_build(args).await),
						None => None,
					}
				};
				(async_build, $context_ident:ident, $builder:ident, false) => {
					$context_ident.map(|args| $builder::build(args))
				};
			}

			impl #universal_test_builder_name {
				pub fn build(self) -> UniversalTestBuilderContext {
					let UniversalTestBuilderArgs {
						#(#builders_snake_case,)*
					} = self._args;
					#(
						let #builders_snake_case = expand_valid_call!(build, #builders_snake_case, #builders, #is_async);
					)*
					UniversalTestBuilderContext {
						#(#builders_snake_case,)*
					}
				}

				pub async fn async_build(self) -> UniversalTestBuilderContext {
					let UniversalTestBuilderArgs {
						#(#builders_snake_case,)*
					} = self._args;
					#(
						let #builders_snake_case = expand_valid_call!(async_build, #builders_snake_case, #builders, #is_async);
					)*
					UniversalTestBuilderContext {
						#(#builders_snake_case,)*
					}
				}
			}
		}
	} else {
		quote::quote! {
			impl #universal_test_builder_name {
				pub fn build(self) -> UniversalTestBuilderContext {
					let UniversalTestBuilderArgs {
						#(#builders_snake_case,)*
					} = self._args;
					#(
						let #builders_snake_case = #builders_snake_case.map(|args| #builders::build(args));
					)*
					UniversalTestBuilderContext {
						#(#builders_snake_case,)*
					}
				}
			}
		}
	};

	let execute_methods = quote::quote! {
		impl UniversalTestBuilderContext {
			pub fn execute<F>(self, test: F)
			where
				F: Fn(Self) -> (),
			{
				test(self);
			}

			pub async fn async_execute<F, Fut>(self, test: F)
			where
				F: Fn(Self) -> Fut,
				Fut: core::future::Future<Output = ()>,
			{
				test(self).await;
			}
		}
	};

	quote::quote! {
		#universal_test_builder_args
		#universal_test_builder_output
		#universal_test_builder
		#transition_functions
		#build_methods
		#execute_methods
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn expand_universal_test_builder_works_with_async() {
		let input = r#"
			{
				builder = FirstBuilder
			},
			{
				builder = SecondBuilder,
				async_builder
			}
		"#;

		let builders: UniversalTestBuilderInput = syn::parse_str(input).unwrap();
		let builder_name: Ident = syn::parse_str("UniversalBuilder").unwrap();

		let actual = expand_universal_test_builder(builder_name, builders);
		let expected = quote::quote! {
			#[derive(Default)]
			struct UniversalTestBuilderArgs {
				first_builder: Option<<FirstBuilder as rustilities::universal_test_builder::Builder>::Args>,
				second_builder: Option<<SecondBuilder as rustilities::universal_test_builder::AsyncBuilder>::Args>,
			}

			struct UniversalTestBuilderContext {
				first_builder: Option<<FirstBuilder as rustilities::universal_test_builder::Builder>::Output>,
				second_builder: Option<<SecondBuilder as rustilities::universal_test_builder::AsyncBuilder>::Output>,
			}

			pub struct UniversalBuilder {
				_args: UniversalTestBuilderArgs,
			}

			impl Default for UniversalBuilder {
				fn default() -> Self {
					Self {
						_args: UniversalTestBuilderArgs::default(),
					}
				}
			}

			impl UniversalBuilder {
				pub fn with_first_builder(self, args: <FirstBuilder as rustilities::universal_test_builder::Builder>::Args) -> Self {
					let mut updated_args = self._args;
					updated_args.first_builder = Some(args);
					Self {
						_args: updated_args,
					}
				}

				pub fn with_second_builder(self, args: <SecondBuilder as rustilities::universal_test_builder::AsyncBuilder>::Args) -> Self {
					let mut updated_args = self._args;
					updated_args.second_builder = Some(args);
					Self {
						_args: updated_args,
					}
				}
			}

			macro_rules! expand_valid_call {
				(build, $context_ident:ident, $builder:ident, true) => {
					None
				};
				(build, $context_ident:ident, $builder:ident, false) => {
					$context_ident.map(|args| $builder::build(args))
				};
				(async_build, $context_ident:ident, $builder:ident, true) => {
					match $context_ident {
						Some(args) => Some($builder::async_build(args).await),
						None => None,
					}
				};
				(async_build, $context_ident:ident, $builder:ident, false) => {
					$context_ident.map(|args| $builder::build(args))
				};
			}

			impl UniversalBuilder {
				pub fn build(self) -> UniversalTestBuilderContext {
					let UniversalTestBuilderArgs {
						first_builder,
						second_builder,
					} = self._args;
					let first_builder = expand_valid_call!(build, first_builder, FirstBuilder, false);
					let second_builder = expand_valid_call!(build, second_builder, SecondBuilder, true);
					UniversalTestBuilderContext {
						first_builder,
						second_builder,
					}
				}

				pub async fn async_build(self) -> UniversalTestBuilderContext {
					let UniversalTestBuilderArgs {
						first_builder,
						second_builder,
					} = self._args;
					let first_builder = expand_valid_call!(async_build, first_builder, FirstBuilder, false);
					let second_builder = expand_valid_call!(async_build, second_builder, SecondBuilder, true);
					UniversalTestBuilderContext {
						first_builder,
						second_builder,
					}
				}
			}

			impl UniversalTestBuilderContext {
				pub fn execute<F>(self, test: F)
				where
					F: Fn(Self) -> (),
				{
					test(self);
				}

				pub async fn async_execute<F, Fut>(self, test: F)
				where
					F: Fn(Self) -> Fut,
					Fut: core::future::Future<Output = ()>,
				{
					test(self).await;
				}
			}
		};

		assert!(rustilities::parsing::syntactic_token_stream_compare(actual, expected));
	}

	#[test]
	fn expand_universal_test_builder_works_without_async() {
		let input = r#"
			{
				builder = FirstBuilder
			},
			{
				builder = SecondBuilder
			}
		"#;

		let builders: UniversalTestBuilderInput = syn::parse_str(input).unwrap();
		let builder_name: Ident = syn::parse_str("UniversalBuilder").unwrap();

		let actual = expand_universal_test_builder(builder_name, builders);
		let expected = quote::quote! {
			#[derive(Default)]
			struct UniversalTestBuilderArgs {
				first_builder: Option<<FirstBuilder as rustilities::universal_test_builder::Builder>::Args>,
				second_builder: Option<<SecondBuilder as rustilities::universal_test_builder::Builder>::Args>,
			}

			struct UniversalTestBuilderContext {
				first_builder: Option<<FirstBuilder as rustilities::universal_test_builder::Builder>::Output>,
				second_builder: Option<<SecondBuilder as rustilities::universal_test_builder::Builder>::Output>,
			}

			pub struct UniversalBuilder {
				_args: UniversalTestBuilderArgs,
			}

			impl Default for UniversalBuilder {
				fn default() -> Self {
					Self {
						_args: UniversalTestBuilderArgs::default(),
					}
				}
			}

			impl UniversalBuilder {
				pub fn with_first_builder(self, args: <FirstBuilder as rustilities::universal_test_builder::Builder>::Args) -> Self {
					let mut updated_args = self._args;
					updated_args.first_builder = Some(args);
					Self {
						_args: updated_args,
					}
				}

				pub fn with_second_builder(self, args: <SecondBuilder as rustilities::universal_test_builder::Builder>::Args) -> Self {
					let mut updated_args = self._args;
					updated_args.second_builder = Some(args);
					Self {
						_args: updated_args,
					}
				}
			}

			impl UniversalBuilder {
				pub fn build(self) -> UniversalTestBuilderContext {
					let UniversalTestBuilderArgs {
						first_builder,
						second_builder,
					} = self._args;
					let first_builder = first_builder.map(|args| FirstBuilder::build(args));
					let second_builder = second_builder.map(|args| SecondBuilder::build(args));
					UniversalTestBuilderContext {
						first_builder,
						second_builder,
					}
				}
			}

			impl UniversalTestBuilderContext {
				pub fn execute<F>(self, test: F)
				where
					F: Fn(Self) -> (),
				{
					test(self);
				}

				pub async fn async_execute<F, Fut>(self, test: F)
				where
					F: Fn(Self) -> Fut,
					Fut: core::future::Future<Output = ()>,
				{
					test(self).await;
				}
			}
		};

		assert!(rustilities::parsing::syntactic_token_stream_compare(actual, expected));
	}
}
