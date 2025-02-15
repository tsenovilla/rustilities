// SPDX-License-Identifier: GPL-3.0

#[cfg(test)]
mod tests;

use std::path::{Component, Path, PathBuf};

/// Transforms a path without prefix into a relative path starting at the current directory.
/// If the path's already prefixed, this function doesn't have any effect.
///
/// ## Example
///
/// ```
/// use std::path::Path;
///
/// let path1 = Path::new("path/1");
/// let path2 = Path::new("../path/2");
///
/// assert_eq!(rustilities::paths::prefix_with_current_dir(path1), Path::new("./path/1"));
/// assert_eq!(rustilities::paths::prefix_with_current_dir(path2), path2);
/// ```
pub fn prefix_with_current_dir<P: AsRef<Path>>(path: P) -> PathBuf {
	fn do_prefix_with_current_dir(path: &Path) -> PathBuf {
		let components = path.components().collect::<Vec<Component>>();
		if !components.is_empty() {
			// If the first component is a normal component, we prefix the path with the current dir
			if let Component::Normal(_) = components[0] {
				return <Component<'_> as AsRef<Path>>::as_ref(&Component::CurDir).join(path);
			}
		}
		path.to_path_buf()
	}
	do_prefix_with_current_dir(path.as_ref())
}
