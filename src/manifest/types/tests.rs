// SPDX-License-Identifier: GPL-3.0

use super::*;

#[test]
fn manifest_dependency_config_new_works() {
	let feature1 = "feature1";
	let feature2 = "feature2";
	let version = "1.0.0";
	let dependency_config = ManifestDependencyConfig::new(
		ManifestDependencyOrigin::crates_io(version),
		false,
		vec![feature1, feature2],
		true,
	);
	assert_eq!(dependency_config.origin, ManifestDependencyOrigin::crates_io(version));
	assert_eq!(dependency_config.default_features, false);
	assert_eq!(dependency_config.features, vec![feature1, feature2]);
	assert_eq!(dependency_config.optional, true);
}

#[test]
fn manifest_dependency_config_add_features_works() {
	let feature1 = "feature1";
	let feature2 = "feature2";
	let feature3 = "feature3";

	let mut dependency_config = ManifestDependencyConfig::new(
		ManifestDependencyOrigin::workspace(),
		false,
		vec![feature1],
		true,
	);
	assert_eq!(dependency_config.features, vec![feature1]);

	dependency_config.add_features(&[feature2, feature3]);
	assert_eq!(dependency_config.features, vec![feature1, feature2, feature3]);
}

#[test]
fn manifest_dependency_origin_crates_io_works() {
	let version = "1.0.0";
	let origin = ManifestDependencyOrigin::crates_io(version);
	assert_eq!(origin, ManifestDependencyOrigin::CratesIO { version });
}

#[test]
fn manifest_dependency_origin_git_works() {
	let url = "https:://some_url.com";
	let branch = "somestablebranch";
	let origin = ManifestDependencyOrigin::git(url, branch);
	assert_eq!(origin, ManifestDependencyOrigin::Git { url, branch });
}

#[test]
fn manifest_dependency_origin_local_works() {
	let relative_path = "../some/path";
	let origin = ManifestDependencyOrigin::local(relative_path.as_ref());
	assert_eq!(origin, ManifestDependencyOrigin::Local { relative_path: relative_path.as_ref() })
}

#[test]
fn manifest_dependency_origin_workspace_works() {
	assert_eq!(ManifestDependencyOrigin::workspace(), ManifestDependencyOrigin::Workspace);
}
