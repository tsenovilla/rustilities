// SPDX-License-Identifier: GPL-3.0

use super::*;

#[test]
fn extract_generics_simple_generics() {
	let input: Generics = parse_quote! {
		<'a, T, D, const N:usize>
	};

	let output_idents: Punctuated<GenericParam, Token![,]> = parse_quote! {'a,T,D,const N:usize};

	assert_eq!((output_idents, None), extract_generics(&input));
}

#[test]
fn extract_generics_with_bounds() {
	let input: Generics = parse_quote! {
		<'a, T: Config + Clone, D: Debug, const N:usize>
	};

	let output_idents: Punctuated<GenericParam, Token![,]> = parse_quote! {'a,T,D,const N:usize};
	let output_where_clause: WhereClause = parse_quote! {where T: Config + Clone, D: Debug};

	assert_eq!((output_idents, Some(output_where_clause)), extract_generics(&input));
}

#[test]
fn extract_generics_with_where_clause() {
	let mut input: Generics = parse_quote! {
		<'a, T, D, const N:usize>
	};

	let where_clause: Option<WhereClause> = Some(parse_quote! {where D:From<String>});
	input = Generics { where_clause, ..input };

	let output_idents: Punctuated<GenericParam, Token![,]> = parse_quote! {'a,T,D,const N:usize};
	let output_where_clause: WhereClause = parse_quote! {where D:From<String>};

	assert_eq!((output_idents, Some(output_where_clause)), extract_generics(&input));
}

#[test]
fn extract_generics_with_bounds_and_where_clause() {
	let mut input: Generics = parse_quote! {
		<'a, T: Config + Clone, D: Debug, const N:usize>
	};

	let where_clause: Option<WhereClause> = Some(parse_quote! {where D:From<String>});
	input = Generics { where_clause, ..input };

	let output_idents: Punctuated<GenericParam, Token![,]> = parse_quote! {'a,T,D,const N:usize};
	let output_where_clause: WhereClause =
		parse_quote! {where D:From<String>, T: Config + Clone, D: Debug};

	assert_eq!((output_idents, Some(output_where_clause)), extract_generics(&input));
}
