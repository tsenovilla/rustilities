// SPDX-License-Identifier: GPL-3.0

use super::*;
use syn::{parse_quote, Attribute, ImplItem, Item, TraitItem};

#[test]
fn attrs_item_const() {
	let item_const: Item = parse_quote! {
		/// Doc comment for const
		#[some_attr]
		const CONST: &str = "hello world";
	};

	let expected_attrs: Vec<Attribute> = vec![
		parse_quote!(/// Doc comment for const
		),
		parse_quote!(#[some_attr]),
	];

	assert_eq!(item_const.attrs().unwrap(), &expected_attrs);
}

#[test]
fn attrs_item_enum() {
	let item_enum: Item = parse_quote! {
		/// Doc comment for enum
		#[some_attr]
		enum MyEnum { A, B }
	};

	let expected_attrs: Vec<Attribute> = vec![
		parse_quote!(/// Doc comment for enum
		),
		parse_quote!(#[some_attr]),
	];

	assert_eq!(item_enum.attrs().unwrap(), &expected_attrs);
}

#[test]
fn attrs_item_extern_crate() {
	let item_extern: Item = parse_quote! {
		/// Doc comment for extern crate
		#[some_attr]
		extern crate my_crate;
	};

	let expected_attrs: Vec<Attribute> = vec![
		parse_quote!(/// Doc comment for extern crate
		),
		parse_quote!(#[some_attr]),
	];

	assert_eq!(item_extern.attrs().unwrap(), &expected_attrs);
}

#[test]
fn attrs_item_fn() {
	let item_fn: Item = parse_quote! {
		/// Doc comment for function
		#[some_attr]
		fn my_function() {}
	};

	let expected_attrs: Vec<Attribute> = vec![
		parse_quote!(/// Doc comment for function
		),
		parse_quote!(#[some_attr]),
	];

	assert_eq!(item_fn.attrs().unwrap(), &expected_attrs);
}

#[test]
fn attrs_item_foreign_mod() {
	let item_foreign: Item = parse_quote! {
		/// Doc comment for foreign mod
		#[some_attr]
		extern "C" {
			fn foreign_function();
		}
	};

	let expected_attrs: Vec<Attribute> = vec![
		parse_quote!(/// Doc comment for foreign mod
		),
		parse_quote!(#[some_attr]),
	];

	assert_eq!(item_foreign.attrs().unwrap(), &expected_attrs);
}

#[test]
fn attrs_item_impl() {
	let item_impl: Item = parse_quote! {
		/// Doc comment for impl
		#[some_attr]
		impl MyTrait for MyStruct {}
	};

	let expected_attrs: Vec<Attribute> = vec![
		parse_quote!(/// Doc comment for impl
		),
		parse_quote!(#[some_attr]),
	];

	assert_eq!(item_impl.attrs().unwrap(), &expected_attrs);
}

#[test]
fn attrs_item_macro() {
	let item_macro: Item = parse_quote! {
		/// Doc comment for macro
		#[some_attr]
		macro_rules! my_macro { () => {} }
	};

	let expected_attrs: Vec<Attribute> = vec![
		parse_quote!(/// Doc comment for macro
		),
		parse_quote!(#[some_attr]),
	];

	assert_eq!(item_macro.attrs().unwrap(), &expected_attrs);
}

#[test]
fn attrs_item_mod() {
	let item_mod: Item = parse_quote! {
		/// Doc comment for module
		#[some_attr]
		mod my_mod {}
	};

	let expected_attrs: Vec<Attribute> = vec![
		parse_quote!(/// Doc comment for module
		),
		parse_quote!(#[some_attr]),
	];

	assert_eq!(item_mod.attrs().unwrap(), &expected_attrs);
}

#[test]
fn attrs_item_static() {
	let item_static: Item = parse_quote! {
		/// Doc comment for static
		#[some_attr]
		static MY_STATIC: i32 = 42;
	};

	let expected_attrs: Vec<Attribute> = vec![
		parse_quote!(/// Doc comment for static
		),
		parse_quote!(#[some_attr]),
	];

	assert_eq!(item_static.attrs().unwrap(), &expected_attrs);
}

#[test]
fn attrs_item_struct() {
	let item_struct: Item = parse_quote! {
		/// Doc comment for struct
		#[some_attr]
		struct MyStruct;
	};

	let expected_attrs: Vec<Attribute> = vec![
		parse_quote!(/// Doc comment for struct
		),
		parse_quote!(#[some_attr]),
	];

	assert_eq!(item_struct.attrs().unwrap(), &expected_attrs);
}

#[test]
fn attrs_item_trait() {
	let item_trait: Item = parse_quote! {
		/// Doc comment for trait
		#[some_attr]
		trait MyTrait {}
	};

	let expected_attrs: Vec<Attribute> = vec![
		parse_quote!(/// Doc comment for trait
		),
		parse_quote!(#[some_attr]),
	];

	assert_eq!(item_trait.attrs().unwrap(), &expected_attrs);
}

#[test]
fn attrs_item_trait_alias() {
	let item_trait_alias: Item = parse_quote! {
		/// Doc comment for trait alias
		#[some_attr]
		trait MyAlias = MyTrait;
	};

	let expected_attrs: Vec<Attribute> = vec![
		parse_quote!(/// Doc comment for trait alias
		),
		parse_quote!(#[some_attr]),
	];

	assert_eq!(item_trait_alias.attrs().unwrap(), &expected_attrs);
}

#[test]
fn attrs_item_type() {
	let item_type: Item = parse_quote! {
		/// Doc comment for type
		#[some_attr]
		type MyType = u32;
	};

	let expected_attrs: Vec<Attribute> = vec![
		parse_quote!(/// Doc comment for type
		),
		parse_quote!(#[some_attr]),
	];

	assert_eq!(item_type.attrs().unwrap(), &expected_attrs);
}

#[test]
fn attrs_item_union() {
	let item_union: Item = parse_quote! {
		/// Doc comment for union
		#[some_attr]
		union MyUnion { a: u32 }
	};

	let expected_attrs: Vec<Attribute> = vec![
		parse_quote!(/// Doc comment for union
		),
		parse_quote!(#[some_attr]),
	];

	assert_eq!(item_union.attrs().unwrap(), &expected_attrs);
}

#[test]
fn attrs_item_use() {
	let item_use: Item = parse_quote! {
		/// Doc comment for use
		#[some_attr]
		use std::collections::HashMap;
	};

	let expected_attrs: Vec<Attribute> = vec![
		parse_quote!(/// Doc comment for use
		),
		parse_quote!(#[some_attr]),
	];

	assert_eq!(item_use.attrs().unwrap(), &expected_attrs);
}

#[test]
fn attrs_item_verbatim() {
	let item_verbatim = Item::Verbatim(parse_quote! {
		/// Doc comment for impl const
		#[some_attr]
		const CONST: i32 = 1;
	});

	assert!(item_verbatim.attrs().is_none());
}

#[test]
fn attrs_impl_item_const() {
	let impl_item_const: ImplItem = parse_quote! {
		/// Doc comment for impl const
		#[some_attr]
		const CONST: i32 = 1;
	};

	let expected_attrs: Vec<Attribute> = vec![
		parse_quote!(/// Doc comment for impl const
		),
		parse_quote!(#[some_attr]),
	];

	assert_eq!(impl_item_const.attrs().unwrap(), &expected_attrs);
}

#[test]
fn attrs_impl_item_fn() {
	let impl_item_fn: ImplItem = parse_quote! {
		/// Doc comment for impl fn
		#[some_attr]
		fn my_method(&self) {}
	};

	let expected_attrs: Vec<Attribute> = vec![
		parse_quote!(/// Doc comment for impl fn
		),
		parse_quote!(#[some_attr]),
	];

	assert_eq!(impl_item_fn.attrs().unwrap(), &expected_attrs);
}

#[test]
fn attrs_impl_item_type() {
	let impl_item_type: ImplItem = parse_quote! {
		/// Doc comment for impl type
		#[some_attr]
		type MyType = u32;
	};

	let expected_attrs: Vec<Attribute> = vec![
		parse_quote!(/// Doc comment for impl type
		),
		parse_quote!(#[some_attr]),
	];

	assert_eq!(impl_item_type.attrs().unwrap(), &expected_attrs);
}

#[test]
fn attrs_impl_item_macro() {
	let impl_item_macro: ImplItem = parse_quote! {
		/// Doc comment for impl macro
		#[some_attr]
		dummy! { () => {} }
	};

	let expected_attrs: Vec<Attribute> = vec![
		parse_quote!(/// Doc comment for impl macro
		),
		parse_quote!(#[some_attr]),
	];

	assert_eq!(impl_item_macro.attrs().unwrap(), &expected_attrs);
}

#[test]
fn attrs_impl_item_verbatim() {
	let impl_item_verbatim = ImplItem::Verbatim(parse_quote! {
		/// Doc comment for impl const
		#[some_attr]
		const CONST: i32 = 1;
	});

	assert!(impl_item_verbatim.attrs().is_none());
}

#[test]
fn attrs_trait_item_const() {
	let trait_item_const: TraitItem = parse_quote! {
		/// Doc comment for trait const
		#[some_attr]
		const CONST: i32;
	};

	let expected_attrs: Vec<Attribute> = vec![
		parse_quote!(/// Doc comment for trait const
		),
		parse_quote!(#[some_attr]),
	];

	assert_eq!(trait_item_const.attrs().unwrap(), &expected_attrs);
}

#[test]
fn attrs_trait_item_fn() {
	let trait_item_fn: TraitItem = parse_quote! {
		/// Doc comment for trait fn
		#[some_attr]
		fn my_method();
	};

	let expected_attrs: Vec<Attribute> = vec![
		parse_quote!(/// Doc comment for trait fn
		),
		parse_quote!(#[some_attr]),
	];

	assert_eq!(trait_item_fn.attrs().unwrap(), &expected_attrs);
}

#[test]
fn attrs_trait_item_type() {
	let trait_item_type: TraitItem = parse_quote! {
		/// Doc comment for trait type
		#[some_attr]
		type MyType;
	};

	let expected_attrs: Vec<Attribute> = vec![
		parse_quote!(/// Doc comment for trait type
		),
		parse_quote!(#[some_attr]),
	];

	assert_eq!(trait_item_type.attrs().unwrap(), &expected_attrs);
}

#[test]
fn attrs_trait_item_macro() {
	let trait_item_macro: TraitItem = parse_quote! {
		/// Doc comment for trait macro
		#[some_attr]
		dummy! { () => {} }
	};

	let expected_attrs: Vec<Attribute> = vec![
		parse_quote!(/// Doc comment for trait macro
		),
		parse_quote!(#[some_attr]),
	];

	assert_eq!(trait_item_macro.attrs().unwrap(), &expected_attrs);
}

#[test]
fn attrs_trait_item_verbatim() {
	let trait_item_verbatim = TraitItem::Verbatim(parse_quote! {
		/// Doc comment for trait const
		#[some_attr]
		const CONST: i32 = 1;
	});

	assert!(trait_item_verbatim.attrs().is_none());
}
