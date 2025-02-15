// SPDX-License-Identifier: GPL-3.0

#[cfg(test)]
mod tests;

use cargo_toml::Manifest;
use std::path::{Path, PathBuf};

/// Given a path, this function finds the manifest corresponding to the innermost crate/workspace
/// containing that path if there's any.
///
/// # Examples
/// ```
/// use std::fs::File;
///
/// let tempdir = tempfile::tempdir().unwrap();
///
/// let crate_path = tempdir.path().join("crate");
/// let manifest_path = crate_path.join("Cargo.toml");
/// let src_path = crate_path.join("src");
/// let main_path = src_path.join("main.rs");
/// let lib_path = src_path.join("lib.rs");
/// std::fs::create_dir_all(&src_path).unwrap();
/// File::create(&manifest_path).unwrap();
/// File::create(&main_path).unwrap();
/// File::create(&lib_path).unwrap();
/// std::fs::write(
///     &manifest_path,
///     r#"
///      [package]
///      name = "test"
///      version = "0.1.0"
///      edition = "2021"
///
///      [dependencies]
///      "#,
///  ).unwrap();
///
/// assert_eq!(rustilities::manifest::find_innermost_manifest(&main_path), Some(manifest_path.clone()));
/// assert_eq!(rustilities::manifest::find_innermost_manifest(&lib_path), Some(manifest_path.clone()));
/// assert_eq!(rustilities::manifest::find_innermost_manifest(&src_path), Some(manifest_path.clone()));
/// assert_eq!(rustilities::manifest::find_innermost_manifest(&manifest_path), Some(manifest_path.clone()));
/// assert_eq!(rustilities::manifest::find_innermost_manifest(&crate_path), Some(manifest_path.clone()));
///
/// let non_crate_path = tempdir.path().join("somewhere");
/// let non_crate_inner_path = non_crate_path.join("somewhere_deeper");
/// std::fs::create_dir_all(&non_crate_inner_path).unwrap();
///
/// assert_eq!(rustilities::manifest::find_innermost_manifest(&non_crate_inner_path), None);
/// ```
pub fn find_innermost_manifest<P: AsRef<Path>>(path: P) -> Option<PathBuf> {
	let mut path = path.as_ref();
	// If the target itself contains a manifest, return it
	let cargo_toml_path = path.join("Cargo.toml");
	match Manifest::from_path(&cargo_toml_path) {
		Ok(manifest) if manifest.package.is_some() || manifest.workspace.is_some() =>
			return Some(cargo_toml_path),
		_ => (),
	}

	// Otherwise, search in the parent dirs
	while let Some(parent) = path.parent() {
		let cargo_toml_path = parent.join("Cargo.toml");
		match Manifest::from_path(&cargo_toml_path) {
			Ok(manifest) if manifest.package.is_some() || manifest.workspace.is_some() =>
				return Some(cargo_toml_path),
			_ => path = parent,
		}
	}
	None
}

/// Given a path, this function finds the manifest corresponding to the workspace
/// containing that path if there's any.
///
/// # Examples
/// ```
/// use std::fs::File;
///
/// let tempdir = tempfile::tempdir().unwrap();
///
/// let workspace_manifest_path = tempdir.path().join("Cargo.toml");
/// let crate_path = tempdir.path().join("crate");
/// let manifest_path = crate_path.join("Cargo.toml");
/// let src_path = crate_path.join("src");
/// let main_path = src_path.join("main.rs");
/// let lib_path = src_path.join("lib.rs");
/// std::fs::create_dir_all(&src_path).unwrap();
/// File::create(&workspace_manifest_path).unwrap();
/// File::create(&manifest_path).unwrap();
/// File::create(&main_path).unwrap();
/// File::create(&lib_path).unwrap();
/// std::fs::write(
///     &manifest_path,
///     r#"
///      [package]
///      name = "test"
///      version = "0.1.0"
///      edition = "2021"
///
///      [dependencies]
///      "#,
///  ).unwrap();
///
/// std::fs::write(
///        &workspace_manifest_path,
///        r#"
///         [workspace]
///         resolver = "2"
///         members = ["crate"]
///
///         [dependencies]
///         "#,
///  ).unwrap();
//
/// assert_eq!(
///     rustilities::manifest::find_workspace_manifest(&main_path),
///     Some(workspace_manifest_path)
/// );
/// ```
pub fn find_workspace_manifest<P: AsRef<Path>>(path: P) -> Option<PathBuf> {
	let mut path = path.as_ref();
	// If the target itself contains a manifest, return it
	let cargo_toml_path = path.join("Cargo.toml");
	match Manifest::from_path(&cargo_toml_path) {
		Ok(manifest) if manifest.workspace.is_some() => return Some(cargo_toml_path),
		_ => (),
	}

	// Otherwise, search in the parent dirs
	while let Some(parent) = path.parent() {
		let cargo_toml_path = parent.join("Cargo.toml");
		match Manifest::from_path(&cargo_toml_path) {
			Ok(manifest) if manifest.workspace.is_some() => return Some(cargo_toml_path),
			_ => path = parent,
		}
	}
	None
}
