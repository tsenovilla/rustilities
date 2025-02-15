// SPDX-License-Identifier: GPL-3.0

use super::*;

#[test]
fn external_works() {
	let version = "1.0.0";
	let external_dep = ManifestDependency::external(version);
	assert_eq!(external_dep, ManifestDependency::External { version });
}

#[test]
fn local_works() {
	let relative_path = "../some/path";
	let local_dep = ManifestDependency::local(relative_path.as_ref());
	assert_eq!(local_dep, ManifestDependency::Local { relative_path: relative_path.as_ref() })
}

#[test]
fn workspace_works() {
	assert_eq!(ManifestDependency::workspace(), ManifestDependency::Workspace);
}
