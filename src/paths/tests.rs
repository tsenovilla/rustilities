// SPDX-License-Identifier: GPL-3.0

use super::*;

#[test]
fn prefix_with_current_dir_works() {
	assert_eq!(
		prefix_with_current_dir::<&Path>("my/path".as_ref()),
		PathBuf::from("./my/path/".to_string())
	);
	assert_eq!(
		prefix_with_current_dir::<&Path>("./my/path".as_ref()),
		PathBuf::from("./my/path/".to_string())
	);
	assert_eq!(
		prefix_with_current_dir::<&Path>("../my/path".as_ref()),
		PathBuf::from("../my/path/".to_string())
	);
	assert_eq!(
		prefix_with_current_dir::<&Path>("/my/path".as_ref()),
		PathBuf::from("/my/path/".to_string())
	);
	assert_eq!(prefix_with_current_dir::<&Path>("".as_ref()), PathBuf::from("".to_string()));
}
