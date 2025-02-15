// SPDX-License-Identifier: GPL-3.0

#[cfg(test)]
mod tests;

use std::path::Path;

/// The basic config for a manifest depedency.
#[derive(Debug, Clone, PartialEq)]
pub enum ManifestDependencyConfig<'a> {
	External { version: &'a str },
	Local { relative_path: &'a Path },
	Workspace,
}

impl<'a> ManifestDependencyConfig<'a> {
	/// Represents a dependency added from crates.io with a specified version.
	pub fn external(version: &'a str) -> Self {
		Self::External { version }
	}

	/// Represents a dependency added from a local path.
	pub fn local(relative_path: &'a Path) -> Self {
		Self::Local { relative_path }
	}

	/// Represents a dependency coming from the workspace.
	pub fn workspace() -> Self {
		Self::Workspace
	}
}
