// SPDX-License-Identifier: GPL-3.0

use super::*;

#[test]
fn test_builder_default_works() {
	let builder = UniversalTestBuilder::default();

	assert_eq!(builder._state_mask, 0);
	assert_eq!(
		format!("{}", std::any::type_name_of_val(&builder._marker)),
		"core::marker::PhantomData<rustilities::universal_test_builder::UniversalTestBuilderInit>"
	);
}
