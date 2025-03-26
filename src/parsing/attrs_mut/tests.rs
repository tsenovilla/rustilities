// SPDX-License-Identifier: GPL-3.0

use super::*;
use syn::{parse_quote, Attribute, ImplItem, Item, TraitItem};

#[test]
fn attrs_mut_item_const() {
	let mut item_const: Item = parse_quote! {
		/// Doc comment for const
		#[some_attr]
		const CONST: &str = "hello world";
	};

	let expected_attrs: Vec<Attribute> = vec![
		parse_quote!(/// Doc comment for const
		),
		parse_quote!(#[some_attr]),
	];

	assert_eq!(*item_const.attrs_mut().unwrap(), expected_attrs);
}

#[test]
fn attrs_mut_item_enum() {
	let mut item_enum: Item = parse_quote! {
		/// Doc comment for enum
		#[some_attr]
		enum MyEnum { A, B }
	};

	let expected_attrs: Vec<Attribute> = vec![
		parse_quote!(/// Doc comment for enum
		),
		parse_quote!(#[some_attr]),
	];

	assert_eq!(*item_enum.attrs_mut().unwrap(), expected_attrs);
}

#[test]
fn attrs_mut_item_extern_crate() {
	let mut item_extern: Item = parse_quote! {
		/// Doc comment for extern crate
		#[some_attr]
		extern crate my_crate;
	};

	let expected_attrs: Vec<Attribute> = vec![
		parse_quote!(/// Doc comment for extern crate
		),
		parse_quote!(#[some_attr]),
	];

	assert_eq!(*item_extern.attrs_mut().unwrap(), expected_attrs);
}

#[test]
fn attrs_mut_item_fn() {
	let mut item_fn: Item = parse_quote! {
		/// Doc comment for function
		#[some_attr]
		fn my_function() {}
	};

	let expected_attrs: Vec<Attribute> = vec![
		parse_quote!(/// Doc comment for function
		),
		parse_quote!(#[some_attr]),
	];

	assert_eq!(*item_fn.attrs_mut().unwrap(), expected_attrs);
}

#[test]
fn attrs_mut_item_foreign_mod() {
	let mut item_foreign: Item = parse_quote! {
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

	assert_eq!(*item_foreign.attrs_mut().unwrap(), expected_attrs);
}

#[test]
fn attrs_mut_item_impl() {
	let mut item_impl: Item = parse_quote! {
		/// Doc comment for impl
		#[some_attr]
		impl MyTrait for MyStruct {}
	};

	let expected_attrs: Vec<Attribute> = vec![
		parse_quote!(/// Doc comment for impl
		),
		parse_quote!(#[some_attr]),
	];

	assert_eq!(*item_impl.attrs_mut().unwrap(), expected_attrs);
}

#[test]
fn attrs_mut_item_macro() {
	let mut item_macro: Item = parse_quote! {
		/// Doc comment for macro
		#[some_attr]
		macro_rules! my_macro { () => {} }
	};

	let expected_attrs: Vec<Attribute> = vec![
		parse_quote!(/// Doc comment for macro
		),
		parse_quote!(#[some_attr]),
	];

	assert_eq!(*item_macro.attrs_mut().unwrap(), expected_attrs);
}

#[test]
fn attrs_mut_item_mod() {
	let mut item_mod: Item = parse_quote! {
		/// Doc comment for module
		#[some_attr]
		mod my_mod {}
	};

	let expected_attrs: Vec<Attribute> = vec![
		parse_quote!(/// Doc comment for module
		),
		parse_quote!(#[some_attr]),
	];

	assert_eq!(*item_mod.attrs_mut().unwrap(), expected_attrs);
}

#[test]
fn attrs_mut_item_static() {
	let mut item_static: Item = parse_quote! {
		/// Doc comment for static
		#[some_attr]
		static MY_STATIC: i32 = 42;
	};

	let expected_attrs: Vec<Attribute> = vec![
		parse_quote!(/// Doc comment for static
		),
		parse_quote!(#[some_attr]),
	];

	assert_eq!(*item_static.attrs_mut().unwrap(), expected_attrs);
}

#[test]
fn attrs_mut_item_struct() {
	let mut item_struct: Item = parse_quote! {
		/// Doc comment for struct
		#[some_attr]
		struct MyStruct;
	};

	let expected_attrs: Vec<Attribute> = vec![
		parse_quote!(/// Doc comment for struct
		),
		parse_quote!(#[some_attr]),
	];

	assert_eq!(*item_struct.attrs_mut().unwrap(), expected_attrs);
}

#[test]
fn attrs_mut_item_trait() {
	let mut item_trait: Item = parse_quote! {
		/// Doc comment for trait
		#[some_attr]
		trait MyTrait {}
	};

	let expected_attrs: Vec<Attribute> = vec![
		parse_quote!(/// Doc comment for trait
		),
		parse_quote!(#[some_attr]),
	];

	assert_eq!(*item_trait.attrs_mut().unwrap(), expected_attrs);
}

#[test]
fn attrs_mut_item_trait_alias() {
	let mut item_trait_alias: Item = parse_quote! {
		/// Doc comment for trait alias
		#[some_attr]
		trait MyAlias = MyTrait;
	};

	let expected_attrs: Vec<Attribute> = vec![
		parse_quote!(/// Doc comment for trait alias
		),
		parse_quote!(#[some_attr]),
	];

	assert_eq!(*item_trait_alias.attrs_mut().unwrap(), expected_attrs);
}

#[test]
fn attrs_mut_item_type() {
	let mut item_type: Item = parse_quote! {
		/// Doc comment for type
		#[some_attr]
		type MyType = u32;
	};

	let expected_attrs: Vec<Attribute> = vec![
		parse_quote!(/// Doc comment for type
		),
		parse_quote!(#[some_attr]),
	];

	assert_eq!(*item_type.attrs_mut().unwrap(), expected_attrs);
}

#[test]
fn attrs_mut_item_union() {
	let mut item_union: Item = parse_quote! {
		/// Doc comment for union
		#[some_attr]
		union MyUnion { a: u32 }
	};

	let expected_attrs: Vec<Attribute> = vec![
		parse_quote!(/// Doc comment for union
		),
		parse_quote!(#[some_attr]),
	];

	assert_eq!(*item_union.attrs_mut().unwrap(), expected_attrs);
}

#[test]
fn attrs_mut_item_use() {
	let mut item_use: Item = parse_quote! {
		/// Doc comment for use
		#[some_attr]
		use std::collections::HashMap;
	};

	let expected_attrs: Vec<Attribute> = vec![
		parse_quote!(/// Doc comment for use
		),
		parse_quote!(#[some_attr]),
	];

	assert_eq!(*item_use.attrs_mut().unwrap(), expected_attrs);
}

#[test]
fn attrs_mut_item_verbatim() {
	let mut item_verbatim = Item::Verbatim(parse_quote! {
		/// Doc comment for impl const
		#[some_attr]
		const CONST: i32 = 1;
	});

	assert!(item_verbatim.attrs_mut().is_none());
}

#[test]
fn attrs_mut_impl_item_const() {
	let mut impl_item_const: ImplItem = parse_quote! {
		/// Doc comment for impl const
		#[some_attr]
		const CONST: i32 = 1;
	};

	let expected_attrs: Vec<Attribute> = vec![
		parse_quote!(/// Doc comment for impl const
		),
		parse_quote!(#[some_attr]),
	];

	assert_eq!(*impl_item_const.attrs_mut().unwrap(), expected_attrs);
}

#[test]
fn attrs_mut_impl_item_fn() {
	let mut impl_item_fn: ImplItem = parse_quote! {
		/// Doc comment for impl fn
		#[some_attr]
		fn my_method(&self) {}
	};

	let expected_attrs: Vec<Attribute> = vec![
		parse_quote!(/// Doc comment for impl fn
		),
		parse_quote!(#[some_attr]),
	];

	assert_eq!(*impl_item_fn.attrs_mut().unwrap(), expected_attrs);
}

#[test]
fn attrs_mut_impl_item_type() {
	let mut impl_item_type: ImplItem = parse_quote! {
		/// Doc comment for impl type
		#[some_attr]
		type MyType = u32;
	};

	let expected_attrs: Vec<Attribute> = vec![
		parse_quote!(/// Doc comment for impl type
		),
		parse_quote!(#[some_attr]),
	];

	assert_eq!(*impl_item_type.attrs_mut().unwrap(), expected_attrs);
}

#[test]
fn attrs_mut_impl_item_macro() {
	let mut impl_item_macro: ImplItem = parse_quote! {
		/// Doc comment for impl macro
		#[some_attr]
		dummy! { () => {} }
	};

	let expected_attrs: Vec<Attribute> = vec![
		parse_quote!(/// Doc comment for impl macro
		),
		parse_quote!(#[some_attr]),
	];

	assert_eq!(*impl_item_macro.attrs_mut().unwrap(), expected_attrs);
}

#[test]
fn attrs_mut_impl_item_verbatim() {
	let mut impl_item_verbatim = ImplItem::Verbatim(parse_quote! {
		/// Doc comment for impl const
		#[some_attr]
		const CONST: i32 = 1;
	});

	assert!(impl_item_verbatim.attrs_mut().is_none());
}

#[test]
fn attrs_mut_trait_item_const() {
	let mut trait_item_const: TraitItem = parse_quote! {
		/// Doc comment for trait const
		#[some_attr]
		const CONST: i32;
	};

	let expected_attrs: Vec<Attribute> = vec![
		parse_quote!(/// Doc comment for trait const
		),
		parse_quote!(#[some_attr]),
	];

	assert_eq!(*trait_item_const.attrs_mut().unwrap(), expected_attrs);
}

#[test]
fn attrs_mut_trait_item_fn() {
	let mut trait_item_fn: TraitItem = parse_quote! {
		/// Doc comment for trait fn
		#[some_attr]
		fn my_method();
	};

	let expected_attrs: Vec<Attribute> = vec![
		parse_quote!(/// Doc comment for trait fn
		),
		parse_quote!(#[some_attr]),
	];

	assert_eq!(*trait_item_fn.attrs_mut().unwrap(), expected_attrs);
}

#[test]
fn attrs_mut_trait_item_type() {
	let mut trait_item_type: TraitItem = parse_quote! {
		/// Doc comment for trait type
		#[some_attr]
		type MyType;
	};

	let expected_attrs: Vec<Attribute> = vec![
		parse_quote!(/// Doc comment for trait type
		),
		parse_quote!(#[some_attr]),
	];

	assert_eq!(*trait_item_type.attrs_mut().unwrap(), expected_attrs);
}

#[test]
fn attrs_mut_trait_item_macro() {
	let mut trait_item_macro: TraitItem = parse_quote! {
		/// Doc comment for trait macro
		#[some_attr]
		dummy! { () => {} }
	};

	let expected_attrs: Vec<Attribute> = vec![
		parse_quote!(/// Doc comment for trait macro
		),
		parse_quote!(#[some_attr]),
	];

	assert_eq!(*trait_item_macro.attrs_mut().unwrap(), expected_attrs);
}

#[test]
fn attrs_mut_trait_item_verbatim() {
	let mut trait_item_verbatim = TraitItem::Verbatim(parse_quote! {
		/// Doc comment for impl const
		#[some_attr]
		const CONST: i32 = 1;
	});

	assert!(trait_item_verbatim.attrs_mut().is_none());
}

#[test]
fn tt_without_docs_tt_with_docs() {
	let tt: Item = parse_quote! {
		/// This is a doc comment that should be removed.
		#[some_attr]
		struct MyStruct;
	};

	let tt = tt_without_docs(&tt);

	let expected_tt: Item = parse_quote! {
		#[some_attr]
		struct MyStruct;
	};

	assert_eq!(tt, expected_tt);
}

#[test]
fn tt_without_docs_tt_without_docs() {
	let tt = Item::Verbatim(parse_quote! {
		/// This is a doc comment that should be removed.
		#[some_attr]
		struct MyStruct;
	});

	let output = tt_without_docs(&tt);

	assert_eq!(output, tt);
}

#[test]
fn tt_without_attrs_tt_with_attrs() {
	let tt: Item = parse_quote! {
		/// This is a doc comment that should be removed.
		#[some_attr]
		struct MyStruct;
	};

	let tt = tt_without_attrs(&tt);

	let expected_tt: Item = parse_quote! {
		struct MyStruct;
	};

	assert_eq!(tt, expected_tt);
}

#[test]
fn tt_without_attrs_tt_without_attrs() {
	let tt = Item::Verbatim(parse_quote! {
		/// This is a doc comment that should be removed.
		#[some_attr]
		struct MyStruct;
	});

	let output = tt_without_attrs(&tt);

	assert_eq!(output, tt);
}
