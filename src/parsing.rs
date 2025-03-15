// SPDX-License-Identifier: GPL-3.0

#[cfg(test)]
mod tests;

use syn::{
	parse_quote, punctuated::Punctuated, GenericParam, Generics, Token, WhereClause, WherePredicate,
};

use proc_macro2::{TokenStream, TokenTree};

/// Given a [Generics](https://docs.rs/syn/latest/syn/struct.Generics.html), this function will
/// return:
/// - A [Punctuated](https://docs.rs/syn/latest/syn/punctuated/struct.Punctuated.html) including the
///   generics declarations without trait bounds.
/// - Another `Punctuated` including the generics idents.
/// - A [WhereClause](https://docs.rs/syn/latest/syn/struct.WhereClause.html) collecting the trait
///   bounds.
///
/// The difference between the two first outputs is only meaningful if a generic const is included
/// in the generics. While the first output will contain the whole const declaration, eg, `const N:
/// usize`, the second one will just contain the const ident, eg, `N`. Extracting them in two
/// different outputs is worthy as they have different use cases, as the snippets below show:
///
/// ```compile_fail
/// struct SomeStruct<const N:usize>([bool; N]);
///
/// struct OtherStruct<const N: usize>{
///     // This isn't allowed
///     item: SomeStruct<const N: usize>
/// }
/// ```
///
///
/// ```compile_fail
/// struct SomeStruct<const N:usize>([bool; N]);
///
/// struct OtherStruct<const N: usize>{
///     item: SomeStruct<N>
/// }
///
/// // This cannot compile as N isn't correctly bounded
/// impl<N> OtherStruct<N>{}
/// ```
///
///
/// ```
/// struct SomeStruct<const N:usize>([bool; N]);
///
/// struct OtherStruct<const N: usize>{
///     item: SomeStruct<N>
/// }
///
/// impl<const N:usize> OtherStruct<N>{}
/// ```
///
/// # Example
/// ```
/// use syn::{Generics, WhereClause, punctuated::Punctuated, GenericParam, parse_quote, Token};
///
/// // A Generics with some bounds and where clause
/// let mut input: Generics = parse_quote!{
///  <'a, T: Config + Clone, D: Debug, const N:usize>
/// };
///
/// let where_clause: Option<WhereClause> = Some(parse_quote!{where D:From<String>});
/// input = Generics{ where_clause, ..input};
///
/// let output_declarations: Punctuated<GenericParam, Token![,]> = parse_quote!{'a, T, D, const N:
/// usize};
/// let output_idents: Punctuated<GenericParam, Token![,]> = parse_quote!{'a,T,D,N};
/// let output_where_clause = Some(parse_quote!{where D:From<String>, T: Config + Clone, D: Debug});
///
/// assert_eq!((output_declarations, output_idents, output_where_clause), rustilities::parsing::extract_generics(&input));
///
/// // A Generics without bounds
/// let input: Generics = parse_quote! {
///  <'a, T, D>
/// };
///
/// let output_declarations: Punctuated<GenericParam, Token![,]> = parse_quote! {'a,T,D};
/// let output_idents: Punctuated<GenericParam, Token![,]> = parse_quote! {'a,T,D};
///
/// // There's not a WhereClause returned
/// assert_eq!((output_declarations, output_idents, None), rustilities::parsing::extract_generics(&input));
/// ```
pub fn extract_generics(
	generics: &Generics,
) -> (Punctuated<GenericParam, Token![,]>, Punctuated<GenericParam, Token![,]>, Option<WhereClause>)
{
	let mut where_clauses: Punctuated<WherePredicate, Token![,]> = Punctuated::new();
	let mut generics_idents: Punctuated<GenericParam, Token![,]> = Punctuated::new();
	let generics_declarations: Punctuated<GenericParam, Token![,]> = generics
		.params
		.iter()
		.map(|item| match item {
			GenericParam::Type(generic_type) => {
				let ident = &generic_type.ident;
				let bounds = &generic_type.bounds;
				if !bounds.is_empty() {
					where_clauses.push(parse_quote! {#ident: #bounds});
				}
				let ident = GenericParam::Type(parse_quote! { #ident });
				generics_idents.push(ident.clone());
				ident
			},
			GenericParam::Lifetime(lifetime) => {
				let lifetime_dec = &lifetime.lifetime;
				let bounds = &lifetime.bounds;
				if !bounds.is_empty() {
					where_clauses.push(parse_quote! {#lifetime_dec: #bounds});
				}
				let lifetime = GenericParam::Lifetime(parse_quote! {#lifetime_dec});
				generics_idents.push(lifetime.clone());
				lifetime
			},
			GenericParam::Const(generic_const) => {
				let ident = &generic_const.ident;
				generics_idents.push(GenericParam::Type(parse_quote! {#ident}));
				item.clone()
			},
		})
		.collect();

	let where_clause = generics
		.where_clause
		.clone()
		.map(|mut where_clause| {
			where_clause.predicates.extend(where_clauses.clone());
			where_clause
		})
		.or_else(|| {
			if !where_clauses.is_empty() {
				Some(parse_quote! { where #where_clauses })
			} else {
				None
			}
		});
	(generics_declarations, generics_idents, where_clause)
}

/// Compares two [`TokenTree`](https://docs.rs/proc-macro2/latest/proc_macro2/enum.TokenTree.html) based solely
/// on their syntactic content, without taking into account any other parsing detail, such as
/// spacing or spans
///
///
/// # Example
///
/// ```rust
/// use proc_macro2::{
///     Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree,
/// };
///
/// let ts1 = {
///     let mut ts = TokenStream::new();
///     ts.extend(
///         [
///             TokenTree::Ident(Ident::new("x", Span::call_site())),
///             TokenTree::Punct(Punct::new('+', Spacing::Alone)),
///             TokenTree::Literal(Literal::i32_unsuffixed(42)),
///         ]
///         .iter()
///         .cloned(),
///     );
///     ts
/// };
///
/// let ts2 = {
///     let mut ts = TokenStream::new();
///     ts.extend(
///         [
///             TokenTree::Ident(Ident::new("x", Span::call_site())),
///             // Same token with different spacing
///             TokenTree::Punct(Punct::new('+', Spacing::Joint)),
///             TokenTree::Literal(Literal::u128_unsuffixed(42)),
///         ]
///         .iter()
///         .cloned(),
///     );
///     ts
/// };
///
/// let group1 = TokenTree::Group(Group::new(Delimiter::Brace, ts1));
/// let group2 = TokenTree::Group(Group::new(Delimiter::Brace, ts2));
/// assert!(rustilities::parsing::syntactic_token_tree_compare(&group1, &group2));
/// ```
pub fn syntactic_token_tree_compare(tree1: &TokenTree, tree2: &TokenTree) -> bool {
	match (tree1, tree2) {
		(TokenTree::Ident(id1), TokenTree::Ident(id2)) => id1 == id2.to_string().as_str(),
		(TokenTree::Punct(p1), TokenTree::Punct(p2)) => p1.as_char() == p2.as_char(),
		(TokenTree::Literal(l1), TokenTree::Literal(l2)) => l1.to_string() == l2.to_string(),
		(TokenTree::Group(g1), TokenTree::Group(g2)) => {
			if g1.delimiter() != g2.delimiter() {
				return false;
			}

			let g1_tt: Vec<TokenTree> = g1.stream().into_iter().collect();
			let g2_tt: Vec<TokenTree> = g2.stream().into_iter().collect();
			if g1_tt.len() != g2_tt.len() {
				return false;
			}
			g1_tt
				.iter()
				.zip(g2_tt.iter())
				.all(|(tt1, tt2)| syntactic_token_tree_compare(tt1, tt2))
		},
		_ => false,
	}
}

/// Compares two [`TokenStream`](https://docs.rs/proc-macro2/latest/proc_macro2/struct.TokenStream.html) based solely
/// on their syntactic content, without taking into account any other parsing detail, such as
/// spacing or spans. This function compares the two token streams by syntactically comparing each
/// [`TokenTree`](https://docs.rs/proc-macro2/latest/proc_macro2/struct.TokenTree.html) contained
/// in both streams.
///
/// # Example
/// ```rust
/// use proc_macro2::{TokenStream, TokenTree, Ident, Punct, Spacing, Literal, Span};
///
/// let mut stream1 = TokenStream::new();
/// let mut stream2 = TokenStream::new();
/// stream1.extend([
///     TokenTree::Ident(Ident::new("x", Span::call_site())),
///     TokenTree::Punct(Punct::new('=', Spacing::Alone)),
///     TokenTree::Literal(Literal::usize_unsuffixed(42)),
/// ]);
/// stream2.extend([
///     TokenTree::Ident(Ident::new("x", Span::call_site())),
///     TokenTree::Punct(Punct::new('=', Spacing::Alone)),
///     TokenTree::Literal(Literal::u8_unsuffixed(42)),
/// ]);
/// assert!(rustilities::parsing::syntactic_token_stream_compare(stream1, stream2));
/// ```
pub fn syntactic_token_stream_compare(stream1: TokenStream, stream2: TokenStream) -> bool {
	let stream1_tt: Vec<TokenTree> = stream1.into_iter().collect();
	let stream2_tt: Vec<TokenTree> = stream2.into_iter().collect();

	if stream1_tt.len() != stream2_tt.len() {
		false
	} else {
		stream1_tt
			.iter()
			.zip(stream2_tt.iter())
			.all(|(tt1, tt2)| syntactic_token_tree_compare(tt1, tt2))
	}
}

/// Assert if a [`TokenStream`](https://docs.rs/proc-macro2/latest/proc_macro2/struct.TokenStream.html) is contained in another,
/// based solely on their syntactic content, without taking into account any other parsing detail,
/// such as spacing or spans.
///
/// The inclusion is also satisfied if the larger `TokenStream` contains a token group (or a nested
/// list of groups) containing the smaller `TokenStream`.
///
/// The inclusion isn't strict, this is, two equal `TokenStream` are contained within each other.
///
/// An empty TokenStream is considered contained in any TokenStream.
///
/// # Examples
///
/// `TokenStream` directly contained inside another `TokenStream`.
///
/// ```rust
/// use proc_macro2::{TokenStream, TokenTree, Ident, Punct, Spacing, Literal, Span};
///
/// let mut small_stream = TokenStream::new();
/// let mut large_stream = TokenStream::new();
/// small_stream.extend([
///   TokenTree::Ident(Ident::new("x", Span::call_site())),
///   TokenTree::Punct(Punct::new('=', Spacing::Alone)),
///   TokenTree::Literal(Literal::i32_suffixed(42)),
/// ]);
/// large_stream.extend([
///   TokenTree::Ident(Ident::new("x", Span::call_site())),
///   TokenTree::Punct(Punct::new('=', Spacing::Alone)),
///   TokenTree::Ident(Ident::new("x", Span::call_site())),
///   TokenTree::Ident(Ident::new("x", Span::call_site())),
///   TokenTree::Punct(Punct::new('=', Spacing::Alone)),
///   TokenTree::Literal(Literal::i32_suffixed(42)),
///   TokenTree::Ident(Ident::new("y", Span::call_site())),
/// ]);
///
/// // x = 42 is contained in x = x x = 42 y, but the opposite is false.
/// assert!(rustilities::parsing::syntactic_token_stream_contains(small_stream.clone(), large_stream.clone()));
/// assert!(!rustilities::parsing::syntactic_token_stream_contains(large_stream, small_stream.clone()));
/// ```
/// `TokenStream` contained inside a group in another `TokenStream`.
///
/// ```rust
/// use proc_macro2::{TokenStream, TokenTree, Delimiter, Group, Ident, Punct, Spacing, Literal, Span};
///
/// let mut small_stream = TokenStream::new();
/// let mut large_stream = TokenStream::new();
/// small_stream.extend([
///   TokenTree::Ident(Ident::new("x", Span::call_site())),
///   TokenTree::Punct(Punct::new('=', Spacing::Alone)),
///   TokenTree::Literal(Literal::i32_suffixed(42)),
/// ]);
///
/// let mut mid_stream = TokenStream::new();
/// mid_stream.extend([
///   TokenTree::Ident(Ident::new("a", Span::call_site())),
///   TokenTree::Punct(Punct::new('+', Spacing::Alone)),
///   TokenTree::Literal(Literal::i32_suffixed(10)),
/// ]);
/// mid_stream.extend(small_stream.clone());
/// let mid_group = TokenTree::Group(Group::new(Delimiter::Bracket, mid_stream));
///
/// let mut top_stream = TokenStream::new();
/// top_stream.extend([
///   TokenTree::Ident(Ident::new("b", Span::call_site())),
///   TokenTree::Punct(Punct::new('-', Spacing::Alone)),
///   TokenTree::Literal(Literal::i32_suffixed(5)),
///   mid_group,
/// ]);
///
/// let top_group = TokenTree::Group(Group::new(Delimiter::Parenthesis, top_stream));
///
/// large_stream.extend([
///   TokenTree::Ident(Ident::new("x", Span::call_site())),
///   TokenTree::Punct(Punct::new('=', Spacing::Alone)),
///   TokenTree::Ident(Ident::new("x", Span::call_site())),
///   TokenTree::Ident(Ident::new("y", Span::call_site())),
///   top_group,
///   TokenTree::Ident(Ident::new("zz", Span::call_site())),
///   TokenTree::Punct(Punct::new(';', Spacing::Alone)),
///   TokenTree::Literal(Literal::i32_suffixed(42)),
/// ]);
///
/// assert!(rustilities::parsing::syntactic_token_stream_contains(small_stream.clone(), large_stream.clone()));
/// assert!(!rustilities::parsing::syntactic_token_stream_contains(large_stream, small_stream));
/// ```
// This clippy lint: https://rust-lang.github.io/rust-clippy/master/index.html#mut_range_bound is
// triggered by the function when the outer index 'i' is mutated to 'j'. This is a false positive
// as immediately after that the flow goes back to the outer while loop, so we can tell clippy this
// is OK
#[allow(clippy::mut_range_bound)]
pub fn syntactic_token_stream_contains(small: TokenStream, large: TokenStream) -> bool {
	let small_tt: Vec<TokenTree> = small.clone().into_iter().collect();
	let large_tt: Vec<TokenTree> = large.into_iter().collect();

	if small_tt.is_empty() {
		return true;
	}

	if large_tt.len() < small_tt.len() {
		return false;
	}

	let mut i = 0;
	'outer: while i < large_tt.len() {
		if syntactic_token_tree_compare(&large_tt[i], &small_tt[0]) {
			for j in i..i + small_tt.len() {
				if !syntactic_token_tree_compare(&large_tt[j], &small_tt[j - i]) {
					i = j;
					continue 'outer;
				}
			}
			return true;
		}

		match &large_tt[i] {
			TokenTree::Group(group)
				if syntactic_token_stream_contains(small.clone(), group.stream()) =>
				return true,
			_ => (),
		}

		i += 1;
	}

	false
}
