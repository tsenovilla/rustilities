// SPDX-License-Identifier: GPL-3.0

use super::*;

#[test]
fn external_works() {
	let version = "1.0.0";
	let external_dep = ManifestDependencyConfig::external(version);
	assert_eq!(external_dep, ManifestDependencyConfig::External { version });
}

#[test]
fn local_works() {
	let relative_path = "../some/path";
	let local_dep = ManifestDependencyConfig::local(relative_path.as_ref());
	assert_eq!(local_dep, ManifestDependencyConfig::Local { relative_path: relative_path.as_ref() })
}

#[test]
fn workspace_works() {
	assert_eq!(ManifestDependencyConfig::workspace(), ManifestDependencyConfig::Workspace);
}
