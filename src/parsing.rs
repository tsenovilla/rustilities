// SPDX-License-Identifier: GPL-3.0

#[cfg(test)]
mod tests;

use syn::{
	parse_quote, punctuated::Punctuated, GenericParam, Generics, Token, WhereClause, WherePredicate,
};

/// Given a [Generics](https://docs.rs/syn/latest/syn/struct.Generics.html), this function will
/// return a [Punctuated](https://docs.rs/syn/latest/syn/punctuated/struct.Punctuated.html)
/// whose elements are the input generics without their trait bounds and a
/// [WhereClause](https://docs.rs/syn/latest/syn/struct.WhereClause.html) collecting that bounds.
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
/// let output_idents: Punctuated<GenericParam, Token![,]> = parse_quote!{'a,T,D,const N:usize};
/// let output_where_clause = Some(parse_quote!{where D:From<String>, T: Config + Clone, D: Debug});
///
/// // The first output contains the generics names, while the second one contains the where clause
/// // with all the trait bounds.
/// assert_eq!((output_idents, output_where_clause), rustilities::parsing::extract_generics(&input));
///
/// // A Generics without bounds
/// let input: Generics = parse_quote! {
///  <'a, T, D, const N:usize>
/// };
///
/// let output_idents: Punctuated<GenericParam, Token![,]> = parse_quote! {'a,T,D,const N:usize};
///
/// // There's not a WhereClause returned
/// assert_eq!((output_idents, None), rustilities::parsing::extract_generics(&input));
/// ```
pub fn extract_generics(
	generics: &Generics,
) -> (Punctuated<GenericParam, Token![,]>, Option<WhereClause>) {
	let mut where_clauses: Punctuated<WherePredicate, Token![,]> = Punctuated::new();
	let generics_idents: Punctuated<GenericParam, Token![,]> = generics
		.params
		.iter()
		.map(|item| {
			if let GenericParam::Type(generic_type) = item {
				let ident = &generic_type.ident;
				let bounds = &generic_type.bounds;
				if !bounds.is_empty() {
					where_clauses.push(parse_quote! {#ident: #bounds});
				}
				GenericParam::Type(parse_quote! { #ident })
			} else {
				item.clone()
			}
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
	(generics_idents, where_clause)
}
