// SPDX-License-Identifier: GPL-3.0

#[cfg(test)]
mod tests;

use std::path::Path;

/// A struct representing how a dependency should look like in a Rust manifest.
#[derive(Debug, Clone, PartialEq)]
pub struct ManifestDependencyConfig<'a> {
	pub origin: ManifestDependencyOrigin<'a>,
	pub default_features: bool,
	pub features: Vec<&'a str>,
	pub optional: bool,
}

impl<'a> ManifestDependencyConfig<'a> {
	/// Creates a new instance of ManifestDependencyConfig specifying:
	/// - The dependency origin.
	/// - If the dependency should use its default features.
	/// - The features the dependency should use.
	/// - If the dependency is optional.
	pub fn new(
		origin: ManifestDependencyOrigin<'a>,
		default_features: bool,
		features: Vec<&'a str>,
		optional: bool,
	) -> Self {
		Self { origin, default_features, features, optional }
	}

	/// Add some features to an existing ManifestDependencyConfig
	pub fn add_features(&mut self, features: &[&'a str]) {
		self.features.extend_from_slice(features);
	}
}

/// Different origins available for a dependency in a Rust manifest.
#[derive(Debug, Clone, PartialEq)]
pub enum ManifestDependencyOrigin<'a> {
	CratesIO { version: &'a str },
	Git { url: &'a str, branch: &'a str },
	Local { relative_path: &'a Path },
	Workspace,
}

impl<'a> ManifestDependencyOrigin<'a> {
	/// Creates a dependency origin from a specific version in [crates.io](https://crates.io).
	pub fn crates_io(version: &'a str) -> Self {
		Self::CratesIO { version }
	}

	/// Creates a dependency origin from a specific branch in a git repository.
	pub fn git(url: &'a str, branch: &'a str) -> Self {
		Self::Git { url, branch }
	}

	/// Creates a dependency origin from a local path.
	pub fn local(relative_path: &'a Path) -> Self {
		Self::Local { relative_path }
	}

	/// Creates a dependency origin coming from the workspace.
	pub fn workspace() -> Self {
		Self::Workspace
	}
}
