// SPDX-License-Identifier: MIT OR Apache-2.0

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

	let args_name = quote::format_ident!("{}Args", universal_test_builder_name);
	let context_name = quote::format_ident!("{}Context", universal_test_builder_name);

	let universal_test_builder_args = quote::quote! {
		#[derive(Default)]
		struct #args_name {
			#(
				#builders_snake_case: Option<#builder_args>,
			)*
		}
	};

	let universal_test_builder_output = quote::quote! {
		pub struct #context_name {
			#(
				#builders_snake_case: Option<#builder_outputs>,
			)*
		}
	};

	let universal_test_builder = quote::quote! {
		pub struct #universal_test_builder_name {
			args: #args_name,
		}

		impl Default for #universal_test_builder_name {
			fn default() -> Self {
				Self {
					args: #args_name::default(),
				}
			}
		}
	};

	let transition_functions = quote::quote! {
		impl #universal_test_builder_name {
			#(
				pub fn #builders_transition_functions(self, args: #builder_args) -> Self {
					let mut updated_args = self.args;
					updated_args.#builders_snake_case = Some(args);
					Self {
						args: updated_args,
					}
				}
			)*
		}
	};

	let build_calls: Vec<TokenStream> = builders
		.iter()
		.zip(builders_snake_case.iter())
		.zip(is_async.iter())
		.map(|((builder, snake), &is_async)| {
			if is_async {
				quote::quote! { let #snake = None; }
			} else {
				quote::quote! { let #snake = #snake.map(|args| #builder::build(args)); }
			}
		})
		.collect();

	let build_methods = if async_methods_needed {
		let async_build_calls: Vec<TokenStream> = builders
			.iter()
			.zip(builders_snake_case.iter())
			.zip(is_async.iter())
			.map(|((builder, snake), &is_async)| {
				if is_async {
					quote::quote! {
						let #snake = match #snake {
							Some(args) => Some(#builder::async_build(args).await),
							None => None,
						};
					}
				} else {
					quote::quote! { let #snake = #snake.map(|args| #builder::build(args)); }
				}
			})
			.collect();

		quote::quote! {
			impl #universal_test_builder_name {
				pub fn build(self) -> #context_name {
					let #args_name {
						#(#builders_snake_case,)*
					} = self.args;
					#(#build_calls)*
					#context_name {
						#(#builders_snake_case,)*
					}
				}

				pub async fn async_build(self) -> #context_name {
					let #args_name {
						#(#builders_snake_case,)*
					} = self.args;
					#(#async_build_calls)*
					#context_name {
						#(#builders_snake_case,)*
					}
				}
			}
		}
	} else {
		quote::quote! {
			impl #universal_test_builder_name {
				pub fn build(self) -> #context_name {
					let #args_name {
						#(#builders_snake_case,)*
					} = self.args;
					#(#build_calls)*
					#context_name {
						#(#builders_snake_case,)*
					}
				}
			}
		}
	};

	let accessors_mut: Vec<Ident> = builders_snake_case
		.iter()
		.map(|snake| quote::format_ident!("{}_mut", snake))
		.collect();

	let accessors_panic_messages: Vec<String> = builders_snake_case
		.iter()
		.zip(builders_transition_functions.iter())
		.map(|(snake, transition)| format!("`{snake}` was not requested: call `{transition}()`"))
		.collect();

	let execute_methods = quote::quote! {
		impl #context_name {
			#(
				pub fn #builders_snake_case(&self) -> &#builder_outputs {
					self.#builders_snake_case.as_ref().expect(#accessors_panic_messages)
				}

				pub fn #accessors_mut(&mut self) -> &mut #builder_outputs {
					self.#builders_snake_case.as_mut().expect(#accessors_panic_messages)
				}
			)*

			pub fn execute<F>(self, test: F)
			where
				F: FnOnce(Self),
			{
				test(self);
			}

			pub async fn async_execute<F, Fut>(self, test: F)
			where
				F: FnOnce(Self) -> Fut,
				Fut: core::future::Future<Output = ()>,
			{
				test(self).await;
			}
		}
	};

	let post_build_calls: Vec<TokenStream> = builders
		.iter()
		.zip(builders_snake_case.iter())
		.zip(is_async.iter())
		.map(|((builder, snake), &is_async)| {
			let trait_path = if is_async {
				quote::quote! { rustilities::universal_test_builder::AsyncBuilder }
			} else {
				quote::quote! { rustilities::universal_test_builder::Builder }
			};
			quote::quote! {
				if let Some(output) = self.#snake.take() {
					<#builder as #trait_path>::on_drop(output);
				}
			}
		})
		.collect();

	let drop_impl = quote::quote! {
		impl Drop for #context_name {
			fn drop(&mut self) {
				#(#post_build_calls)*
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
		#drop_impl
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
			struct UniversalBuilderArgs {
				first_builder: Option<<FirstBuilder as rustilities::universal_test_builder::Builder>::Args>,
				second_builder: Option<<SecondBuilder as rustilities::universal_test_builder::AsyncBuilder>::Args>,
			}

			pub struct UniversalBuilderContext {
				first_builder: Option<<FirstBuilder as rustilities::universal_test_builder::Builder>::Output>,
				second_builder: Option<<SecondBuilder as rustilities::universal_test_builder::AsyncBuilder>::Output>,
			}

			pub struct UniversalBuilder {
				args: UniversalBuilderArgs,
			}

			impl Default for UniversalBuilder {
				fn default() -> Self {
					Self {
						args: UniversalBuilderArgs::default(),
					}
				}
			}

			impl UniversalBuilder {
				pub fn with_first_builder(self, args: <FirstBuilder as rustilities::universal_test_builder::Builder>::Args) -> Self {
					let mut updated_args = self.args;
					updated_args.first_builder = Some(args);
					Self {
						args: updated_args,
					}
				}

				pub fn with_second_builder(self, args: <SecondBuilder as rustilities::universal_test_builder::AsyncBuilder>::Args) -> Self {
					let mut updated_args = self.args;
					updated_args.second_builder = Some(args);
					Self {
						args: updated_args,
					}
				}
			}

			impl UniversalBuilder {
				pub fn build(self) -> UniversalBuilderContext {
					let UniversalBuilderArgs {
						first_builder,
						second_builder,
					} = self.args;
					let first_builder = first_builder.map(|args| FirstBuilder::build(args));
					let second_builder = None;
					UniversalBuilderContext {
						first_builder,
						second_builder,
					}
				}

				pub async fn async_build(self) -> UniversalBuilderContext {
					let UniversalBuilderArgs {
						first_builder,
						second_builder,
					} = self.args;
					let first_builder = first_builder.map(|args| FirstBuilder::build(args));
					let second_builder = match second_builder {
						Some(args) => Some(SecondBuilder::async_build(args).await),
						None => None,
					};
					UniversalBuilderContext {
						first_builder,
						second_builder,
					}
				}
			}

			impl UniversalBuilderContext {
				pub fn first_builder(&self) -> &<FirstBuilder as rustilities::universal_test_builder::Builder>::Output {
					self.first_builder.as_ref().expect("`first_builder` was not requested: call `with_first_builder()`")
				}

				pub fn first_builder_mut(&mut self) -> &mut <FirstBuilder as rustilities::universal_test_builder::Builder>::Output {
					self.first_builder.as_mut().expect("`first_builder` was not requested: call `with_first_builder()`")
				}

				pub fn second_builder(&self) -> &<SecondBuilder as rustilities::universal_test_builder::AsyncBuilder>::Output {
					self.second_builder.as_ref().expect("`second_builder` was not requested: call `with_second_builder()`")
				}

				pub fn second_builder_mut(&mut self) -> &mut <SecondBuilder as rustilities::universal_test_builder::AsyncBuilder>::Output {
					self.second_builder.as_mut().expect("`second_builder` was not requested: call `with_second_builder()`")
				}

				pub fn execute<F>(self, test: F)
				where
					F: FnOnce(Self),
				{
					test(self);
				}

				pub async fn async_execute<F, Fut>(self, test: F)
				where
					F: FnOnce(Self) -> Fut,
					Fut: core::future::Future<Output = ()>,
				{
					test(self).await;
				}
			}

			impl Drop for UniversalBuilderContext {
				fn drop(&mut self) {
					if let Some(output) = self.first_builder.take() {
						<FirstBuilder as rustilities::universal_test_builder::Builder>::on_drop(output);
					}
					if let Some(output) = self.second_builder.take() {
						<SecondBuilder as rustilities::universal_test_builder::AsyncBuilder>::on_drop(output);
					}
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
			struct UniversalBuilderArgs {
				first_builder: Option<<FirstBuilder as rustilities::universal_test_builder::Builder>::Args>,
				second_builder: Option<<SecondBuilder as rustilities::universal_test_builder::Builder>::Args>,
			}

			pub struct UniversalBuilderContext {
				first_builder: Option<<FirstBuilder as rustilities::universal_test_builder::Builder>::Output>,
				second_builder: Option<<SecondBuilder as rustilities::universal_test_builder::Builder>::Output>,
			}

			pub struct UniversalBuilder {
				args: UniversalBuilderArgs,
			}

			impl Default for UniversalBuilder {
				fn default() -> Self {
					Self {
						args: UniversalBuilderArgs::default(),
					}
				}
			}

			impl UniversalBuilder {
				pub fn with_first_builder(self, args: <FirstBuilder as rustilities::universal_test_builder::Builder>::Args) -> Self {
					let mut updated_args = self.args;
					updated_args.first_builder = Some(args);
					Self {
						args: updated_args,
					}
				}

				pub fn with_second_builder(self, args: <SecondBuilder as rustilities::universal_test_builder::Builder>::Args) -> Self {
					let mut updated_args = self.args;
					updated_args.second_builder = Some(args);
					Self {
						args: updated_args,
					}
				}
			}

			impl UniversalBuilder {
				pub fn build(self) -> UniversalBuilderContext {
					let UniversalBuilderArgs {
						first_builder,
						second_builder,
					} = self.args;
					let first_builder = first_builder.map(|args| FirstBuilder::build(args));
					let second_builder = second_builder.map(|args| SecondBuilder::build(args));
					UniversalBuilderContext {
						first_builder,
						second_builder,
					}
				}
			}

			impl UniversalBuilderContext {
				pub fn first_builder(&self) -> &<FirstBuilder as rustilities::universal_test_builder::Builder>::Output {
					self.first_builder.as_ref().expect("`first_builder` was not requested: call `with_first_builder()`")
				}

				pub fn first_builder_mut(&mut self) -> &mut <FirstBuilder as rustilities::universal_test_builder::Builder>::Output {
					self.first_builder.as_mut().expect("`first_builder` was not requested: call `with_first_builder()`")
				}

				pub fn second_builder(&self) -> &<SecondBuilder as rustilities::universal_test_builder::Builder>::Output {
					self.second_builder.as_ref().expect("`second_builder` was not requested: call `with_second_builder()`")
				}

				pub fn second_builder_mut(&mut self) -> &mut <SecondBuilder as rustilities::universal_test_builder::Builder>::Output {
					self.second_builder.as_mut().expect("`second_builder` was not requested: call `with_second_builder()`")
				}

				pub fn execute<F>(self, test: F)
				where
					F: FnOnce(Self),
				{
					test(self);
				}

				pub async fn async_execute<F, Fut>(self, test: F)
				where
					F: FnOnce(Self) -> Fut,
					Fut: core::future::Future<Output = ()>,
				{
					test(self).await;
				}
			}

			impl Drop for UniversalBuilderContext {
				fn drop(&mut self) {
					if let Some(output) = self.first_builder.take() {
						<FirstBuilder as rustilities::universal_test_builder::Builder>::on_drop(output);
					}
					if let Some(output) = self.second_builder.take() {
						<SecondBuilder as rustilities::universal_test_builder::Builder>::on_drop(output);
					}
				}
			}
		};

		assert!(rustilities::parsing::syntactic_token_stream_compare(actual, expected));
	}
}
