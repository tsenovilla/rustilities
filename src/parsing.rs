// SPDX-License-Identifier: GPL-3.0

#[cfg(test)]
mod tests;

use syn::{
	parse_quote, punctuated::Punctuated, GenericParam, Generics, Token, WhereClause, WherePredicate,
};

/// Given a [Generics](https://docs.rs/syn/latest/syn/struct.Generics.html), this function will
/// return:
/// - A [Punctuated](https://docs.rs/syn/latest/syn/punctuated/struct.Punctuated.html) including
/// the generics declarations without trait bounds.
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
