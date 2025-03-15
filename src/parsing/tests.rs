// SPDX-License-Identifier: GPL-3.0

use super::*;
use proc_macro2::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream};

#[test]
fn extract_generics_simple_generics() {
	let input: Generics = parse_quote! {
		<'a, T, D, const N:usize>
	};

	let output_declarations: Punctuated<GenericParam, Token![,]> =
		parse_quote! {'a,T,D,const N:usize};
	let output_idents: Punctuated<GenericParam, Token![,]> = parse_quote! {'a,T,D,N};

	assert_eq!((output_declarations, output_idents, None), extract_generics(&input));
}

#[test]
fn extract_generics_with_bounds() {
	let input: Generics = parse_quote! {
		<'a, 'b: 'a, T: Config + Clone, D: Debug, const N:usize>
	};

	let output_declarations: Punctuated<GenericParam, Token![,]> =
		parse_quote! {'a, 'b,T,D,const N:usize};
	let output_idents: Punctuated<GenericParam, Token![,]> = parse_quote! {'a, 'b,T,D,N};
	let output_where_clause: WhereClause = parse_quote! {where 'b: 'a, T: Config + Clone, D: Debug};

	assert_eq!(
		(output_declarations, output_idents, Some(output_where_clause)),
		extract_generics(&input)
	);
}

#[test]
fn extract_generics_with_where_clause() {
	let mut input: Generics = parse_quote! {
		<'a, T, D>
	};

	let where_clause: Option<WhereClause> = Some(parse_quote! {where D:From<String>});
	input = Generics { where_clause, ..input };

	let output_declarations: Punctuated<GenericParam, Token![,]> = parse_quote! {'a,T,D};
	let output_idents: Punctuated<GenericParam, Token![,]> = parse_quote! {'a,T,D};
	let output_where_clause: WhereClause = parse_quote! {where D:From<String>};

	assert_eq!(
		(output_declarations, output_idents, Some(output_where_clause)),
		extract_generics(&input)
	);
}

#[test]
fn extract_generics_with_bounds_and_where_clause() {
	let mut input: Generics = parse_quote! {
		<'a, T: Config + Clone, D: Debug, const N:usize>
	};

	let where_clause: Option<WhereClause> = Some(parse_quote! {where D:From<String>});
	input = Generics { where_clause, ..input };

	let output_declarations: Punctuated<GenericParam, Token![,]> =
		parse_quote! {'a,T,D,const N:usize};
	let output_idents: Punctuated<GenericParam, Token![,]> = parse_quote! {'a,T,D,N};
	let output_where_clause: WhereClause =
		parse_quote! {where D:From<String>, T: Config + Clone, D: Debug};

	assert_eq!(
		(output_declarations, output_idents, Some(output_where_clause)),
		extract_generics(&input)
	);
}

#[test]
fn ident_equal() {
	let id1 = TokenTree::Ident(Ident::new("foo", Span::call_site()));
	let id2 = TokenTree::Ident(Ident::new("foo", Span::call_site()));
	assert!(syntactic_token_tree_compare(&id1, &id2));
}

#[test]
fn ident_not_equal() {
	let id1 = TokenTree::Ident(Ident::new("foo", Span::call_site()));
	let id2 = TokenTree::Ident(Ident::new("bar", Span::call_site()));
	assert!(!syntactic_token_tree_compare(&id1, &id2));
}

#[test]
fn punct_equal_same_spacing() {
	let punct1 = TokenTree::Punct(Punct::new(';', Spacing::Alone));
	let punct2 = TokenTree::Punct(Punct::new(';', Spacing::Alone));
	assert!(syntactic_token_tree_compare(&punct1, &punct2));
}

#[test]
fn punct_equal_different_spacing() {
	let punct1 = TokenTree::Punct(Punct::new(';', Spacing::Alone));
	let punct2 = TokenTree::Punct(Punct::new(';', Spacing::Joint));
	assert!(syntactic_token_tree_compare(&punct1, &punct2));
}

#[test]
fn punct_not_equal() {
	let punct1 = TokenTree::Punct(Punct::new(';', Spacing::Alone));
	let punct2 = TokenTree::Punct(Punct::new(',', Spacing::Alone));
	assert!(!syntactic_token_tree_compare(&punct1, &punct2));
}

#[test]
fn literal_equal() {
	let lit1 = TokenTree::Literal(Literal::string("hello"));
	let lit2 = TokenTree::Literal(Literal::string("hello"));
	assert!(syntactic_token_tree_compare(&lit1, &lit2));
}

#[test]
fn literal_equal_different_literal_type_with_same_syntactic_content() {
	let lit1 = TokenTree::Literal(Literal::u8_unsuffixed(1));
	let lit2 = TokenTree::Literal(Literal::usize_unsuffixed(1));
	assert!(syntactic_token_tree_compare(&lit1, &lit2));
}

#[test]
fn literal_not_equal() {
	let lit1 = TokenTree::Literal(Literal::string("hello"));
	let lit2 = TokenTree::Literal(Literal::string("world"));
	assert!(!syntactic_token_tree_compare(&lit1, &lit2));
}

#[test]
fn group_empty() {
	let group1 = TokenTree::Group(Group::new(Delimiter::None, TokenStream::new()));
	let group2 = TokenTree::Group(Group::new(Delimiter::None, TokenStream::new()));
	assert!(syntactic_token_tree_compare(&group1, &group2));
}

#[test]
fn group_different_length() {
	// group1 has one token, group2 is empty.
	let ts1 = {
		let mut ts = TokenStream::new();
		ts.extend([TokenTree::Ident(Ident::new("a", Span::call_site()))].iter().cloned());
		ts
	};
	let group1 = TokenTree::Group(Group::new(Delimiter::None, ts1));
	let group2 = TokenTree::Group(Group::new(Delimiter::None, TokenStream::new()));
	assert!(!syntactic_token_tree_compare(&group1, &group2));
}

#[test]
fn group_equal_tokens() {
	let ts1 = {
		let mut ts = TokenStream::new();
		ts.extend(
			[
				TokenTree::Ident(Ident::new("x", Span::call_site())),
				TokenTree::Punct(Punct::new('+', Spacing::Alone)),
				TokenTree::Literal(Literal::i32_unsuffixed(42)),
			]
			.iter()
			.cloned(),
		);
		ts
	};
	let ts2 = {
		let mut ts = TokenStream::new();
		ts.extend(
			[
				TokenTree::Ident(Ident::new("x", Span::call_site())),
				TokenTree::Punct(Punct::new('+', Spacing::Joint)),
				TokenTree::Literal(Literal::i32_unsuffixed(42)),
			]
			.iter()
			.cloned(),
		);
		ts
	};
	let group1 = TokenTree::Group(Group::new(Delimiter::None, ts1));
	let group2 = TokenTree::Group(Group::new(Delimiter::None, ts2));
	assert!(syntactic_token_tree_compare(&group1, &group2));
}

#[test]
fn group_not_equal_tokens() {
	let ts1 = {
		let mut ts = TokenStream::new();
		ts.extend(
			[
				TokenTree::Ident(Ident::new("x", Span::call_site())),
				TokenTree::Punct(Punct::new('+', Spacing::Alone)),
				TokenTree::Literal(Literal::i32_unsuffixed(42)),
			]
			.iter()
			.cloned(),
		);
		ts
	};
	let ts2 = {
		let mut ts = TokenStream::new();
		ts.extend(
			[
				TokenTree::Ident(Ident::new("x", Span::call_site())),
				TokenTree::Punct(Punct::new('-', Spacing::Alone)),
				TokenTree::Literal(Literal::i32_unsuffixed(42)),
			]
			.iter()
			.cloned(),
		);
		ts
	};
	let group1 = TokenTree::Group(Group::new(Delimiter::None, ts1));
	let group2 = TokenTree::Group(Group::new(Delimiter::None, ts2));
	assert!(!syntactic_token_tree_compare(&group1, &group2));
}

#[test]
fn group_not_equal_ordering() {
	let ts1 = {
		let mut ts = TokenStream::new();
		ts.extend(
			[
				TokenTree::Ident(Ident::new("x", Span::call_site())),
				TokenTree::Punct(Punct::new('+', Spacing::Alone)),
				TokenTree::Literal(Literal::i32_unsuffixed(42)),
			]
			.iter()
			.cloned(),
		);
		ts
	};
	let ts2 = {
		let mut ts = TokenStream::new();
		ts.extend(
			[
				TokenTree::Punct(Punct::new('+', Spacing::Alone)),
				TokenTree::Ident(Ident::new("x", Span::call_site())),
				TokenTree::Literal(Literal::i32_unsuffixed(42)),
			]
			.iter()
			.cloned(),
		);
		ts
	};
	let group1 = TokenTree::Group(Group::new(Delimiter::None, ts1));
	let group2 = TokenTree::Group(Group::new(Delimiter::None, ts2));
	assert!(!syntactic_token_tree_compare(&group1, &group2));
}

#[test]
fn cross_variant_comparison() {
	let id = TokenTree::Ident(Ident::new("test", Span::call_site()));
	let lit = TokenTree::Literal(Literal::string("test"));
	assert!(!syntactic_token_tree_compare(&id, &lit));
}

#[test]
fn nested_groups_equal() {
	let inner_ts1 = {
		let mut ts = TokenStream::new();
		ts.extend([TokenTree::Ident(Ident::new("inner", Span::call_site()))].iter().cloned());
		ts
	};
	let inner_ts2 = {
		let mut ts = TokenStream::new();
		ts.extend([TokenTree::Ident(Ident::new("inner", Span::call_site()))].iter().cloned());
		ts
	};
	let inner_group1 = TokenTree::Group(Group::new(Delimiter::None, inner_ts1));
	let inner_group2 = TokenTree::Group(Group::new(Delimiter::None, inner_ts2));

	let ts1 = {
		let mut ts = TokenStream::new();
		ts.extend(
			[inner_group1, TokenTree::Punct(Punct::new('!', Spacing::Alone))]
				.iter()
				.cloned(),
		);
		ts
	};
	let ts2 = {
		let mut ts = TokenStream::new();
		ts.extend(
			[inner_group2, TokenTree::Punct(Punct::new('!', Spacing::Alone))]
				.iter()
				.cloned(),
		);
		ts
	};

	let group1 = TokenTree::Group(Group::new(Delimiter::None, ts1));
	let group2 = TokenTree::Group(Group::new(Delimiter::None, ts2));
	assert!(syntactic_token_tree_compare(&group1, &group2));
}

#[test]
fn nested_groups_not_equal() {
	let inner_ts1 = {
		let mut ts = TokenStream::new();
		ts.extend([TokenTree::Ident(Ident::new("inner", Span::call_site()))].iter().cloned());
		ts
	};
	let inner_ts2 = {
		let mut ts = TokenStream::new();
		ts.extend([TokenTree::Ident(Ident::new("different", Span::call_site()))].iter().cloned());
		ts
	};
	let inner_group1 = TokenTree::Group(Group::new(Delimiter::None, inner_ts1));
	let inner_group2 = TokenTree::Group(Group::new(Delimiter::None, inner_ts2));

	let ts1 = {
		let mut ts = TokenStream::new();
		ts.extend(
			[inner_group1, TokenTree::Punct(Punct::new('!', Spacing::Alone))]
				.iter()
				.cloned(),
		);
		ts
	};
	let ts2 = {
		let mut ts = TokenStream::new();
		ts.extend(
			[inner_group2, TokenTree::Punct(Punct::new('!', Spacing::Alone))]
				.iter()
				.cloned(),
		);
		ts
	};

	let group1 = TokenTree::Group(Group::new(Delimiter::None, ts1));
	let group2 = TokenTree::Group(Group::new(Delimiter::None, ts2));
	assert!(!syntactic_token_tree_compare(&group1, &group2));
}
