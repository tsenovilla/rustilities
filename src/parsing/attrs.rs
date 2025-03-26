// SPDX-License-Identifier: GPL-3.0

//! This module provides the [`Attrs`] trait, a convenient way to retrieve references to attributes
//! from a [`syn`] type if they exist. It is particularly useful when working with inner attributes
//! of [`syn`] enums (such as [`Item`]), where each variant holds its own attributes. By using this
//! trait, it's possibel to avoid pattern matching on every variant when the exact used variant is
//! not relevant.

#[cfg(test)]
mod tests;

use syn::{Attribute, ImplItem, Item, TraitItem};

/// The [`Attrs`] trait offers a convenient way to retrieve references to attributes from a
/// [`syn`] type if they exist. It is particularly useful when working with inner attributes of
/// [`syn`] enums (such as [`Item`]), where each variant holds its own attributes. By using this
/// trait, it's possible to avoid pattern matching on every variant when the exact used variant is
/// not relevant.
///
/// It's currently implemented for [`Item`], [`ImplItem`] and [`TraitItem`], but this will be
/// updated as needed.
///
/// ```rust
/// use syn::{Item, parse_quote, Attribute};
/// use rustilities::parsing::attrs::Attrs;
///
/// let item_fn: Item = parse_quote! {
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
/// assert_eq!(item_fn.attrs().unwrap(), &expected_attrs);
/// ```
pub trait Attrs {
	fn attrs(&self) -> Option<&Vec<Attribute>>;
}

impl Attrs for Item {
	fn attrs(&self) -> Option<&Vec<Attribute>> {
		match self {
			Item::Const(item) => Some(&item.attrs),
			Item::Enum(item) => Some(&item.attrs),
			Item::ExternCrate(item) => Some(&item.attrs),
			Item::Fn(item) => Some(&item.attrs),
			Item::ForeignMod(item) => Some(&item.attrs),
			Item::Impl(item) => Some(&item.attrs),
			Item::Macro(item) => Some(&item.attrs),
			Item::Mod(item) => Some(&item.attrs),
			Item::Static(item) => Some(&item.attrs),
			Item::Struct(item) => Some(&item.attrs),
			Item::Trait(item) => Some(&item.attrs),
			Item::TraitAlias(item) => Some(&item.attrs),
			Item::Type(item) => Some(&item.attrs),
			Item::Union(item) => Some(&item.attrs),
			Item::Use(item) => Some(&item.attrs),
			_ => None,
		}
	}
}

impl Attrs for ImplItem {
	fn attrs(&self) -> Option<&Vec<Attribute>> {
		match self {
			ImplItem::Const(item) => Some(&item.attrs),
			ImplItem::Fn(item) => Some(&item.attrs),
			ImplItem::Type(item) => Some(&item.attrs),
			ImplItem::Macro(item) => Some(&item.attrs),
			_ => None,
		}
	}
}

impl Attrs for TraitItem {
	fn attrs(&self) -> Option<&Vec<Attribute>> {
		match self {
			TraitItem::Const(item) => Some(&item.attrs),
			TraitItem::Fn(item) => Some(&item.attrs),
			TraitItem::Type(item) => Some(&item.attrs),
			TraitItem::Macro(item) => Some(&item.attrs),
			_ => None,
		}
	}
}
