// SPDX-License-Identifier: GPL-3.0

use super::*;

#[test]
fn prefix_with_current_dir_works() {
	let current_dir_component = <Component<'_> as AsRef<Path>>::as_ref(&Component::CurDir);
	let parent_dir_component = <Component<'_> as AsRef<Path>>::as_ref(&Component::ParentDir);
	let root_dir_component = <Component<'_> as AsRef<Path>>::as_ref(&Component::RootDir);

	assert_eq!(
		prefix_with_current_dir(Path::new("my").join("path")),
		current_dir_component.join("my").join("path")
	);
	assert_eq!(
		prefix_with_current_dir(current_dir_component.join("my").join("path")),
		current_dir_component.join("my").join("path")
	);
	assert_eq!(
		prefix_with_current_dir(root_dir_component.join("my").join("path")),
		root_dir_component.join("my").join("path")
	);
	assert_eq!(
		prefix_with_current_dir(parent_dir_component.join("my").join("path")),
		parent_dir_component.join("my").join("path")
	);
	assert_eq!(prefix_with_current_dir::<&Path>("".as_ref()), current_dir_component);
}
