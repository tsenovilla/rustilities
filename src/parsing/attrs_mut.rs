// SPDX-License-Identifier: GPL-3.0

//! This module provides the [`AttrsMut`] trait, a convenient way to retrieve mutable references
//! for attributes from a [`syn`] type if they exist. It is particularly useful when working with
//! inner attributes of [`syn`] enums (such as [`Item`]), where each variant holds its own
//! attributes. By using this trait, it's possible to avoid pattern matching on every variant when
//! the exact used variant is not relevant.
//!
//! Additionally, the module provides the [`tt_without_docs`] and [`tt_without_attrs`] functions,
//! which are useful to get a copy of a [`syn`] type without docs/attributes, in case they aren't
//! relevant (eg, when comparing two types, sometimes may be interesting to deem them equal without
//! taking into account their docs/attributes).

#[cfg(test)]
mod tests;

use syn::{Attribute, ImplItem, Item, TraitItem};

/// The [`AttrsMut`] trait offers a convenient way to retrieve mutable references to attributes from
/// a [`syn`] type if they exist. It is particularly useful when working with inner attributes of
/// [`syn`] enums (such as [`Item`]), where each variant holds its own attributes. By using this
/// trait, it's possible to avoid pattern matching on every variant when the exact used variant is
/// not relevant.
///
/// It's currently implemented for [`Item`], [`ImplItem`] and [`TraitItem`], but this will be
/// updated as needed.
///
/// ```rust
/// use syn::{Item, parse_quote, Attribute};
/// use rustilities::parsing::attrs_mut::AttrsMut;
///
/// let mut item_fn: Item = parse_quote! {
///   /// Doc comment for function
///   #[some_attr]
///   fn my_function() {}
/// };
///
/// let expected_attrs: Vec<Attribute> = vec![
///   parse_quote!(/// Doc comment for function
///   ),
///   parse_quote!(#[some_attr]),
/// ];
///
/// assert_eq!(item_fn.attrs_mut().unwrap(), &expected_attrs);
/// ```
pub trait AttrsMut {
	fn attrs_mut(&mut self) -> Option<&mut Vec<Attribute>>;
}

/// Get a copy of the input without its doc comments. Appliable to any [`syn`] type implementing
/// [`Clone`] and [`AttrsMut`].
///
/// ```rust
/// use syn::{parse_quote, Item};
///
/// let tt: Item = parse_quote! {
///   /// This is a doc comment that should be removed.
///   #[some_attr]
///   struct MyStruct;
/// };
///
/// let tt = rustilities::parsing::attrs_mut::tt_without_docs(&tt);
///
/// let expected_tt: Item = parse_quote! {
///   #[some_attr]
///   struct MyStruct;
/// };
///
/// assert_eq!(tt, expected_tt);
/// ```
pub fn tt_without_docs<T: AttrsMut + Clone>(item: &T) -> T {
	let mut output = item.clone();
	if let Some(attrs) = output.attrs_mut() {
		attrs.retain(|attr| !attr.path().is_ident("doc"));
	}
	output
}

/// Get a copy of the input without its attributes. Appliable to any [`syn`] type implementing
/// [`Clone`] and [`AttrsMut`].
///
/// ```rust
/// use syn::{parse_quote, Item};
///
/// let tt: Item = parse_quote! {
///   /// This is a doc comment that should be removed.
///   #[some_attr]
///   struct MyStruct;
/// };
///
/// let tt = rustilities::parsing::attrs_mut::tt_without_attrs(&tt);
///
/// let expected_tt: Item = parse_quote! {
///   struct MyStruct;
/// };
///
/// assert_eq!(tt, expected_tt);
/// ```
pub fn tt_without_attrs<T: AttrsMut + Clone>(item: &T) -> T {
	let mut output = item.clone();
	if let Some(attrs) = output.attrs_mut() {
		*attrs = Vec::new();
	}
	output
}

impl AttrsMut for Item {
	fn attrs_mut(&mut self) -> Option<&mut Vec<Attribute>> {
		match self {
			Item::Const(item) => Some(&mut item.attrs),
			Item::Enum(item) => Some(&mut item.attrs),
			Item::ExternCrate(item) => Some(&mut item.attrs),
			Item::Fn(item) => Some(&mut item.attrs),
			Item::ForeignMod(item) => Some(&mut item.attrs),
			Item::Impl(item) => Some(&mut item.attrs),
			Item::Macro(item) => Some(&mut item.attrs),
			Item::Mod(item) => Some(&mut item.attrs),
			Item::Static(item) => Some(&mut item.attrs),
			Item::Struct(item) => Some(&mut item.attrs),
			Item::Trait(item) => Some(&mut item.attrs),
			Item::TraitAlias(item) => Some(&mut item.attrs),
			Item::Type(item) => Some(&mut item.attrs),
			Item::Union(item) => Some(&mut item.attrs),
			Item::Use(item) => Some(&mut item.attrs),
			_ => None,
		}
	}
}

impl AttrsMut for ImplItem {
	fn attrs_mut(&mut self) -> Option<&mut Vec<Attribute>> {
		match self {
			ImplItem::Const(item) => Some(&mut item.attrs),
			ImplItem::Fn(item) => Some(&mut item.attrs),
			ImplItem::Type(item) => Some(&mut item.attrs),
			ImplItem::Macro(item) => Some(&mut item.attrs),
			_ => None,
		}
	}
}

impl AttrsMut for TraitItem {
	fn attrs_mut(&mut self) -> Option<&mut Vec<Attribute>> {
		match self {
			TraitItem::Const(item) => Some(&mut item.attrs),
			TraitItem::Fn(item) => Some(&mut item.attrs),
			TraitItem::Type(item) => Some(&mut item.attrs),
			TraitItem::Macro(item) => Some(&mut item.attrs),
			_ => None,
		}
	}
}
