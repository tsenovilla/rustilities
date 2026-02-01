// SPDX-License-Identifier: GPL-3.0

use super::*;

#[test]
fn universal_test_builder_definition_parse_works() {
	let input = r#"
    state_index = 2,
    build_method = with_temp_dir,
    generates = TempDir,
    extra_args = (u128, Path)
"#;

	let parsed: UniversalTestBuilderDefinition = syn::parse_str(input).unwrap();

	let expected_generates: Type = syn::parse_quote!(TempDir);
	let expected_extra_args: Punctuated<Type, Token![,]> = syn::parse_quote!(u128, Path);

	assert_eq!(parsed.state_index, 2);
	assert_eq!(parsed.build_method.to_string(), "with_temp_dir");
	assert_eq!(parsed.generates, expected_generates);
	assert_eq!(parsed.extra_args, Some(expected_extra_args));
}

#[test]
fn universal_test_builder_definition_parse_works_with_single_extra_arg() {
	let input = r#"
    state_index = 2,
    build_method = with_temp_dir,
    generates = TempDir,
    extra_args = (u128)
"#;

	let parsed: UniversalTestBuilderDefinition = syn::parse_str(input).unwrap();

	let expected_generates: Type = syn::parse_quote!(TempDir);
	let expected_extra_args: Punctuated<Type, Token![,]> = syn::parse_quote!(u128);

	assert_eq!(parsed.state_index, 2);
	assert_eq!(parsed.build_method.to_string(), "with_temp_dir");
	assert_eq!(parsed.generates, expected_generates);
	assert_eq!(parsed.extra_args, Some(expected_extra_args));
}

#[test]
fn universal_test_builder_definition_parse_works_without_extra_args() {
	let input = r#"
    state_index = 2,
    build_method = with_temp_dir,
    generates = TempDir
"#;

	let parsed: UniversalTestBuilderDefinition = syn::parse_str(input).unwrap();

	let expected_generates: Type = syn::parse_quote!(TempDir);

	assert_eq!(parsed.state_index, 2);
	assert_eq!(parsed.build_method.to_string(), "with_temp_dir");
	assert_eq!(parsed.generates, expected_generates);
	assert_eq!(parsed.extra_args, None);
}

#[test]
fn universal_test_builder_definition_fails_if_state_index_not_followed_by_eq() {
	let input = r#"
    state_index : 2,
    build_method = with_temp_dir,
    generates = TempDir
"#;

	let parsed: Result<UniversalTestBuilderDefinition> = syn::parse_str(input);

	assert_eq!("expected `=`", parsed.unwrap_err().to_string());
}

#[test]
fn universal_test_builder_definition_fails_if_state_index_not_number() {
	let input = r#"
    state_index = A,
    build_method = with_temp_dir,
    generates = TempDir
"#;

	let parsed: Result<UniversalTestBuilderDefinition> = syn::parse_str(input);

	assert_eq!("expected integer literal", parsed.unwrap_err().to_string());
}

#[test]
fn universal_test_builder_definition_fails_if_state_index_doesnt_fit_into_u128() {
	let input = r#"
    state_index = 340282366920938463463374607431768211456,
    build_method = with_temp_dir,
    generates = TempDir
"#;

	let parsed: Result<UniversalTestBuilderDefinition> = syn::parse_str(input);

	assert_eq!("number too large to fit in target type", parsed.unwrap_err().to_string());
}

#[test]
fn universal_test_builder_definition_fails_if_build_method_not_followed_by_eq() {
	let input = r#"
    state_index = 2,
    build_method : with_temp_dir,
    generates = TempDir
"#;

	let parsed: Result<UniversalTestBuilderDefinition> = syn::parse_str(input);

	assert_eq!("expected `=`", parsed.unwrap_err().to_string());
}

#[test]
fn universal_test_builder_definition_fails_if_build_method_not_ident() {
	let input = r#"
    state_index = 2,
    build_method = 42,
    generates = TempDir
"#;

	let parsed: Result<UniversalTestBuilderDefinition> = syn::parse_str(input);

	assert_eq!("expected identifier", parsed.unwrap_err().to_string());
}

#[test]
fn universal_test_builder_definition_fails_if_generates_not_followed_by_eq() {
	let input = r#"
    state_index = 2,
    build_method = with_temp_dir,
    generates : TempDir
"#;

	let parsed: Result<UniversalTestBuilderDefinition> = syn::parse_str(input);

	assert_eq!("expected `=`", parsed.unwrap_err().to_string());
}

#[test]
fn universal_test_builder_definition_fails_if_generates_not_followed_by_type() {
	let input = r#"
    state_index = 2,
    build_method = with_temp_dir,
    generates = 2
"#;

	let parsed: Result<UniversalTestBuilderDefinition> = syn::parse_str(input);

	assert_eq!(
		"expected one of: `for`, parentheses, `fn`, `unsafe`, `extern`, identifier, `::`, `<`, `dyn`, square brackets, `*`, `&`, `!`, `impl`, `_`, lifetime",
		parsed.unwrap_err().to_string()
	);
}

#[test]
fn universal_test_builder_definition_fails_if_extra_args_not_followed_by_eq() {
	let input = r#"
    state_index = 2,
    build_method = with_temp_dir,
    generates = TempDir,
    extra_args: (u128, Path)
"#;

	let parsed: Result<UniversalTestBuilderDefinition> = syn::parse_str(input);

	assert_eq!("expected `=`", parsed.unwrap_err().to_string());
}

#[test]
fn universal_test_builder_definition_fails_if_extra_args_not_well_written() {
	let input = r#"
    state_index = 2,
    build_method = with_temp_dir,
    generates = TempDir,
    extra_args = (u128; Path)
"#;

	let parsed: Result<UniversalTestBuilderDefinition> = syn::parse_str(input);

	assert_eq!("expected `,`", parsed.unwrap_err().to_string());
}

#[test]
fn universal_test_builder_definition_fails_if_random_stuff_passed() {
	let input = r#"
    state_index = 2,
    random_stuff,
    build_method = with_temp_dir,
    generates = TempDir,
    extra_args = (u128; Path)
"#;

	let parsed: Result<UniversalTestBuilderDefinition> = syn::parse_str(input);

	assert_eq!("unexpected token in universal_test_builder", parsed.unwrap_err().to_string());
}

#[test]
fn universal_test_builder_definition_fails_if_missing_state_index() {
	let input = r#"
    build_method = with_temp_dir,
    generates = TempDir,
    extra_args = (u128, Path)
"#;

	let parsed: Result<UniversalTestBuilderDefinition> = syn::parse_str(input);

	assert_eq!("Missing state_index in universal_test_builder", parsed.unwrap_err().to_string());
}

#[test]
fn universal_test_builder_definition_fails_if_missing_build_method() {
	let input = r#"
    state_index = 2,
    generates = TempDir,
    extra_args = (u128, Path)
"#;

	let parsed: Result<UniversalTestBuilderDefinition> = syn::parse_str(input);

	assert_eq!("Missing build_method in universal_test_builder", parsed.unwrap_err().to_string());
}

#[test]
fn universal_test_builder_definition_fails_if_missing_generates() {
	let input = r#"
    state_index = 2,
    build_method = with_temp_dir,
    extra_args = (u128, Path)
"#;

	let parsed: Result<UniversalTestBuilderDefinition> = syn::parse_str(input);

	assert_eq!("Missing generates in universal_test_builder", parsed.unwrap_err().to_string());
}
