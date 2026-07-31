// SPDX-License-Identifier: MIT OR Apache-2.0

use super::*;

#[test]
fn universal_test_builder_definition_parse_works() {
	let input = r#"
		{
			builder = MyBuilder
		}
	"#;

	let parsed: UniversalTestBuilderDefinition = syn::parse_str(input).unwrap();

	assert_eq!(parsed.builder.to_string(), "MyBuilder");
	assert!(!parsed.async_builder);
}

#[test]
fn universal_test_builder_definition_parse_works_with_async_builder() {
	let input = r#"
		{
			builder = MyBuilder,
			async_builder
		}
	"#;

	let parsed: UniversalTestBuilderDefinition = syn::parse_str(input).unwrap();

	assert_eq!(parsed.builder.to_string(), "MyBuilder");
	assert!(parsed.async_builder);
}

#[test]
fn universal_test_builder_definition_parse_works_with_async_builder_first() {
	let input = r#"
		{
			async_builder,
			builder = MyBuilder
		}
	"#;

	let parsed: UniversalTestBuilderDefinition = syn::parse_str(input).unwrap();

	assert_eq!(parsed.builder.to_string(), "MyBuilder");
	assert!(parsed.async_builder);
}

#[test]
fn universal_test_builder_definition_fails_if_builder_not_followed_by_eq() {
	let input = r#"
		{
			builder : MyBuilder
		}
	"#;

	let parsed: Result<UniversalTestBuilderDefinition> = syn::parse_str(input);

	assert_eq!("expected `=`", parsed.unwrap_err().to_string());
}

#[test]
fn universal_test_builder_definition_fails_if_builder_not_ident() {
	let input = r#"
		{
			builder = 42
		}
	"#;

	let parsed: Result<UniversalTestBuilderDefinition> = syn::parse_str(input);

	assert_eq!("expected identifier", parsed.unwrap_err().to_string());
}

#[test]
fn universal_test_builder_definition_fails_if_random_stuff_passed() {
	let input = r#"
		{
			builder = MyBuilder,
			random_stuff
		}
	"#;

	let parsed: Result<UniversalTestBuilderDefinition> = syn::parse_str(input);

	assert_eq!("unexpected token in universal_test_builder", parsed.unwrap_err().to_string());
}

#[test]
fn universal_test_builder_definition_fails_if_missing_builder() {
	let input = r#"
		{
			async_builder
		}
	"#;

	let parsed: Result<UniversalTestBuilderDefinition> = syn::parse_str(input);

	assert_eq!("Missing builder in universal_test_builder", parsed.unwrap_err().to_string());
}

#[test]
fn universal_test_builder_definition_fails_if_empty() {
	let input = r#"
		{
		}
	"#;

	let parsed: Result<UniversalTestBuilderDefinition> = syn::parse_str(input);

	assert_eq!("Missing builder in universal_test_builder", parsed.unwrap_err().to_string());
}

#[test]
fn universal_test_builder_input_parse_works_with_one_input() {
	let input = r#"
		{
			builder = MyBuilder
		}
	"#;

	let parsed: UniversalTestBuilderInput = syn::parse_str(input).unwrap();
	let parsed = parsed.blocks;

	assert_eq!(parsed.len(), 1);
	assert_eq!(parsed[0].builder.to_string(), "MyBuilder");
	assert!(!parsed[0].async_builder);
}

#[test]
fn universal_test_builder_input_parse_works_with_several_inputs() {
	let input = r#"
		{
			builder = FirstBuilder
		},
		{
			builder = SecondBuilder,
			async_builder
		}
	"#;

	let parsed: UniversalTestBuilderInput = syn::parse_str(input).unwrap();
	let parsed = parsed.blocks;

	assert_eq!(parsed.len(), 2);
	assert_eq!(parsed[0].builder.to_string(), "FirstBuilder");
	assert!(!parsed[0].async_builder);

	assert_eq!(parsed[1].builder.to_string(), "SecondBuilder");
	assert!(parsed[1].async_builder);
}

#[test]
fn universal_test_builder_input_parse_works_with_non_inputs() {
	let input = r#""#;

	let parsed: UniversalTestBuilderInput = syn::parse_str(input).unwrap();
	let parsed = parsed.blocks;

	assert_eq!(parsed.len(), 0);
}

#[test]
fn universal_test_builder_input_parse_fails_if_not_universal_test_builder_definition_passed() {
	let input = r#"
		{
			builder = MyBuilder
		},
		{ Something else }
	"#;

	let parsed: Result<UniversalTestBuilderInput> = syn::parse_str(input);
	assert_eq!("unexpected token in universal_test_builder", parsed.unwrap_err().to_string());
}

#[test]
fn derive_streams_works() {
	let input = r#"
		{
			builder = FirstBuilder
		},
		{
			builder = SecondBuilder,
			async_builder
		}
	"#;

	let parsed: UniversalTestBuilderInput = syn::parse_str(input).unwrap();
	let DerivedStreams {
		builders,
		builders_transition_functions,
		builders_snake_case,
		is_async,
		builder_args,
		builder_outputs,
	} = parsed.derive_streams();

	// Check builders
	assert_eq!(builders.len(), 2);
	assert_eq!(builders[0].to_string(), "FirstBuilder");
	assert_eq!(builders[1].to_string(), "SecondBuilder");

	// Check builders_transition_functions
	assert_eq!(builders_transition_functions.len(), 2);
	assert_eq!(builders_transition_functions[0].to_string(), "with_first_builder");
	assert_eq!(builders_transition_functions[1].to_string(), "with_second_builder");

	// Check builders_snake_case
	assert_eq!(builders_snake_case.len(), 2);
	assert_eq!(builders_snake_case[0].to_string(), "first_builder");
	assert_eq!(builders_snake_case[1].to_string(), "second_builder");

	// Check is_async
	assert_eq!(is_async.len(), 2);
	assert!(!is_async[0]);
	assert!(is_async[1]);

	// Check builder_args
	assert_eq!(builder_args.len(), 2);
	let expected_args_first: TokenStream =
		quote::quote! { <FirstBuilder as rustilities::universal_test_builder::Builder>::Args };
	let expected_args_second: TokenStream = quote::quote! { <SecondBuilder as rustilities::universal_test_builder::AsyncBuilder>::Args };
	assert_eq!(builder_args[0].to_string(), expected_args_first.to_string());
	assert_eq!(builder_args[1].to_string(), expected_args_second.to_string());

	// Check builder_outputs
	assert_eq!(builder_outputs.len(), 2);
	let expected_output_first: TokenStream =
		quote::quote! { <FirstBuilder as rustilities::universal_test_builder::Builder>::Output };
	let expected_output_second: TokenStream = quote::quote! { <SecondBuilder as rustilities::universal_test_builder::AsyncBuilder>::Output };
	assert_eq!(builder_outputs[0].to_string(), expected_output_first.to_string());
	assert_eq!(builder_outputs[1].to_string(), expected_output_second.to_string());
}

#[test]
fn derive_streams_works_with_empty_input() {
	let input = r#""#;

	let parsed: UniversalTestBuilderInput = syn::parse_str(input).unwrap();
	let DerivedStreams {
		builders,
		builders_transition_functions,
		builders_snake_case,
		is_async,
		builder_args,
		builder_outputs,
	} = parsed.derive_streams();

	assert!(builders.is_empty());
	assert!(builders_transition_functions.is_empty());
	assert!(builders_snake_case.is_empty());
	assert!(is_async.is_empty());
	assert!(builder_args.is_empty());
	assert!(builder_outputs.is_empty());
}

#[test]
fn universal_test_builder_input_fails_if_duplicate_builders() {
	let input = r#"
		{
			builder = MyBuilder
		},
		{
			builder = MyBuilder
		}
	"#;

	let parsed: Result<UniversalTestBuilderInput> = syn::parse_str(input);
	assert_eq!(
		"duplicate builder 'MyBuilder' in universal_test_builder",
		parsed.unwrap_err().to_string()
	);
}

#[test]
fn parse_unit_struct_ident_works() {
	let item: ItemStruct = syn::parse_str("struct Foo;").unwrap();
	assert_eq!(parse_unit_struct_ident(item).unwrap().to_string(), "Foo");
}

#[test]
fn parse_unit_struct_ident_fails_for_named_fields() {
	let item: ItemStruct = syn::parse_str("struct Foo { x: u32 }").unwrap();
	assert_eq!(parse_unit_struct_ident(item).unwrap_err().to_string(), "expected a unit struct");
}

#[test]
fn parse_unit_struct_ident_fails_for_tuple_struct() {
	let item: ItemStruct = syn::parse_str("struct Foo(u32);").unwrap();
	assert_eq!(parse_unit_struct_ident(item).unwrap_err().to_string(), "expected a unit struct");
}

#[test]
fn parse_unit_struct_ident_fails_for_empty_named_fields() {
	let item: ItemStruct = syn::parse_str("struct Foo {}").unwrap();
	assert_eq!(parse_unit_struct_ident(item).unwrap_err().to_string(), "expected a unit struct");
}

#[test]
fn to_snake_case_handles_consecutive_uppercase() {
	assert_eq!(to_snake_case("HTTPBuilder"), "http_builder");
	assert_eq!(to_snake_case("MyURLParser"), "my_url_parser");
	assert_eq!(to_snake_case("ABCDef"), "abc_def");
	assert_eq!(to_snake_case("MyBuilder"), "my_builder");
	assert_eq!(to_snake_case("A"), "a");
	assert_eq!(to_snake_case("AB"), "ab");
}
