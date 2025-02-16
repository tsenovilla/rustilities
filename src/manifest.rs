// SPDX-License-Identifier: GPL-3.0

#[cfg(test)]
mod tests;
mod types;

use crate::Error;
use cargo_toml::Manifest;
use std::path::{Path, PathBuf};
use toml_edit::{Array, DocumentMut, InlineTable, Item, Table};
pub use types::{ManifestDependencyConfig, ManifestDependencyOrigin};

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
/// [package]
/// name = "test"
/// version = "0.1.0"
/// edition = "2021"
///
/// [dependencies]
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
	fn do_find_innermost_manifest(path: &Path) -> Option<PathBuf> {
		let mut path = path;
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
	do_find_innermost_manifest(path.as_ref())
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
/// [package]
/// name = "test"
/// version = "0.1.0"
/// edition = "2021"
///
/// [dependencies]
///      "#,
///  ).unwrap();
///
/// std::fs::write(
///        &workspace_manifest_path,
///        r#"
/// [workspace]
/// resolver = "2"
/// members = ["crate"]
///
/// [dependencies]
///         "#,
///  ).unwrap();
//
/// assert_eq!(
///     rustilities::manifest::find_workspace_manifest(&main_path),
///     Some(workspace_manifest_path)
/// );
/// ```
pub fn find_workspace_manifest<P: AsRef<Path>>(path: P) -> Option<PathBuf> {
	fn do_find_workspace_manifest(path: &Path) -> Option<PathBuf> {
		let mut path = path;
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
	do_find_workspace_manifest(path.as_ref())
}

/// Given a path, this function tries to determine if it points to a crate's manifest and if that's
/// the case, returns the crate's name.
///
/// # Examples
/// ```
/// use std::fs::File;
///
/// let tempdir = tempfile::tempdir().unwrap();
///
/// let crate_path = tempdir.path().join("crate");
/// let manifest_path = crate_path.join("Cargo.toml");
/// std::fs::create_dir_all(&crate_path).unwrap();
/// File::create(&manifest_path).unwrap();
/// std::fs::write(
///     &manifest_path,
///     r#"
/// [package]
/// name = "test"
/// version = "0.1.0"
/// edition = "2021"
///
/// [dependencies]
///      "#,
///  ).unwrap();
///
/// assert_eq!(rustilities::manifest::find_crate_name(manifest_path).unwrap(), "test");
/// assert!(rustilities::manifest::find_crate_name(crate_path).is_none());
/// ```
pub fn find_crate_name<P: AsRef<Path>>(manifest_path: P) -> Option<String> {
	Manifest::from_path(manifest_path.as_ref())
		.ok()?
		.package
		.map(|package| package.name)
}

pub fn add_crate_to_dependencies<P: AsRef<Path>>(
	manifest_path: P,
	dependency_name: &str,
	dependency_config: ManifestDependencyConfig,
) -> Result<(), Error> {
	fn do_add_crate_to_dependencies(
		dependencies: &mut Table,
		dependency_name: &str,
		dependency_config: ManifestDependencyConfig,
	) {
		let mut dependency_declaration = InlineTable::new();
		match &dependency_config.origin {
			ManifestDependencyOrigin::Workspace => {
				dependency_declaration.insert(
					"workspace",
					toml_edit::value(true)
						.into_value()
						.expect("true is bool, so value(true) is Value::Boolean;qed;"),
				);
			},
			ManifestDependencyOrigin::Git { url, branch } => {
				dependency_declaration.insert(
					"git",
					toml_edit::value(url.to_owned())
						.into_value()
						.expect("url is String, so value(url) is Value::String; qed;"),
				);
				dependency_declaration.insert(
					"branch",
					toml_edit::value(branch.to_owned())
						.into_value()
						.expect("branch is String, so value(branch) is Value::String; qed;"),
				);
			},
			ManifestDependencyOrigin::CratesIO { version } => {
				dependency_declaration.insert(
					"version",
					toml_edit::value(version.to_owned())
						.into_value()
						.expect("version is String, so value(version) is Value::String; qed;"),
				);
			},
			ManifestDependencyOrigin::Local { relative_path } => {
				dependency_declaration.insert(
					"path",
					toml_edit::value(relative_path.to_string_lossy().into_owned())
						.into_value()
						.expect(
						"relative_path is String, so value(relative_path) is Value::String; qed;",
					),
				);
			},
		}

		if !dependency_config.default_features {
			dependency_declaration.insert(
				"default-features",
				toml_edit::value(false)
					.into_value()
					.expect("false is bool so value(false) is Value::Boolean; qed;"),
			);
		}

		if !dependency_config.features.is_empty() {
			let mut features = Array::new();
			dependency_config
				.features
				.iter()
				.for_each(|feature| features.push(feature.to_owned()));
			dependency_declaration.insert(
				"features",
				toml_edit::value(features)
					.into_value()
					.expect("features is Array, so value(features) is Value::Array; qed;"),
			);
		}

		if dependency_config.optional {
			dependency_declaration.insert(
				"optional",
				toml_edit::value(true)
					.into_value()
					.expect("true is bool so value(true) is Value::Boolean; qed;"),
			);
		}

		dependencies.insert(dependency_name, toml_edit::value(dependency_declaration));
	}

	let mut doc = std::fs::read_to_string(manifest_path.as_ref())?.parse::<DocumentMut>()?;
	if let Some(Item::Table(dependencies)) = doc.get_mut("dependencies") {
		do_add_crate_to_dependencies(dependencies, dependency_name, dependency_config);
	} else {
		let mut dependencies = Table::new();
		do_add_crate_to_dependencies(&mut dependencies, dependency_name, dependency_config);
		doc.insert("dependencies", Item::Table(dependencies));
	}

	std::fs::write(manifest_path, doc.to_string())?;

	Ok(())
}
