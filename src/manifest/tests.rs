// SPDX-License-Identifier: GPL-3.0

use super::*;
use std::{
	fs::{File, Permissions},
	io::ErrorKind,
	os::unix::fs::PermissionsExt,
};
use tempfile::TempDir;

struct TestBuilder {
	tempdir: TempDir,
	crate_paths: Vec<PathBuf>,
	non_crate_paths: Vec<PathBuf>,
	crate_manifest: PathBuf,
	workspace_manifest: PathBuf,
	tempdir_is_workspace: bool,
	with_crate: bool,
	with_non_crate: bool,
	with_read_only_manifest: bool,
}

impl Default for TestBuilder {
	fn default() -> Self {
		let tempdir = tempfile::tempdir().expect("The tempdir should be created; qed;");
		Self {
			tempdir,
			crate_paths: Vec::new(),
			non_crate_paths: Vec::new(),
			crate_manifest: PathBuf::new(),
			workspace_manifest: PathBuf::new(),
			tempdir_is_workspace: false,
			with_crate: false,
			with_non_crate: false,
			with_read_only_manifest: false,
		}
	}
}

impl TestBuilder {
	fn with_crate(mut self) -> Self {
		self.with_crate = true;
		self
	}

	fn with_non_crate(mut self) -> Self {
		self.with_non_crate = true;
		self
	}

	fn tempdir_is_workspace(mut self) -> Self {
		self.tempdir_is_workspace = true;
		self
	}

	fn with_read_only_manifest(mut self) -> Self {
		self.with_read_only_manifest = true;
		self
	}

	fn build(mut self) -> Self {
		if self.tempdir_is_workspace {
			let workspace_manifest = self.tempdir.path().join("Cargo.toml");
			File::create(&workspace_manifest).expect("This should be created");
			std::fs::write(
				&workspace_manifest,
				r#"
[workspace]
resolver = "2"
members = ["crate"]

[workspace.dependencies]
        "#,
			)
			.expect("The manifest should be writable; qed;");

			if self.with_read_only_manifest {
				std::fs::set_permissions(&workspace_manifest, Permissions::from_mode(0o444))
					.expect("manifest permissions should be configurable; qed;");
			}

			self.workspace_manifest = workspace_manifest.clone();
			self.non_crate_paths
				.extend_from_slice(&[self.tempdir.path().to_path_buf(), workspace_manifest]);
		}

		if self.with_crate {
			let crate_path = self.tempdir.path().join("crate");
			let manifest_path = crate_path.join("Cargo.toml");
			let src_path = crate_path.join("src");
			let main_path = src_path.join("main.rs");
			let lib_path = src_path.join("lib.rs");
			std::fs::create_dir_all(&src_path).expect("This should be created; qed;");
			File::create(&manifest_path).expect("This should be created; qed;");
			File::create(&main_path).expect("This should be created; qed;");
			File::create(&lib_path).expect("This should be created; qed;");
			std::fs::write(
				&manifest_path,
				r#"
[package]
name = "test"
version = "0.1.0"
edition = "2021"

[dependencies]
        "#,
			)
			.expect("The manifest should be writable; qed;");
			std::fs::write(&main_path, "use std::fs;")
				.expect("The main.rs file should be writable; qed;");

			if self.with_read_only_manifest {
				std::fs::set_permissions(&manifest_path, Permissions::from_mode(0o444))
					.expect("manifest permissions should be configurable; qed;");
			}
			self.crate_manifest = manifest_path.clone();
			self.crate_paths.extend_from_slice(&[
				crate_path,
				manifest_path,
				src_path,
				main_path,
				lib_path,
			]);
		}

		if self.with_non_crate {
			let non_crate_path = self.tempdir.path().join("somewhere");
			let non_crate_inner_path = non_crate_path.join("somewhere_deeper");
			std::fs::create_dir_all(&non_crate_inner_path).expect("This should be created; qed;");
			self.non_crate_paths.extend_from_slice(&[non_crate_path, non_crate_inner_path]);
		}

		self
	}

	fn execute<F>(&self, test: F)
	where
		F: Fn(&Self) -> (),
	{
		test(self);
	}
}

#[test]
fn find_innermost_manifest_finds_manifest_from_different_parts_of_a_crate() {
	TestBuilder::default().with_crate().build().execute(|builder| {
		builder.crate_paths.iter().for_each(|path| {
			assert!(matches!(
				find_innermost_manifest(path),
				Some(ref manifest_path) if manifest_path == &builder.crate_manifest
			));
		});
	})
}

#[test]
fn find_innermost_manifest_finds_right_manifest_from_different_parts_of_a_workspace() {
	TestBuilder::default()
		.tempdir_is_workspace()
		.with_crate()
		.with_non_crate()
		.build()
		.execute(|builder| {
			// For crate paths, the innermost manifest is the crate manifest
			builder.crate_paths.iter().for_each(|path| {
				assert!(matches!(
					find_innermost_manifest(path),
					Some(ref manifest_path) if manifest_path == &builder.crate_manifest
				));
			});

			// While for these paths, the innermost manifest is the workspace manifest
			builder.non_crate_paths.iter().for_each(|path| {
				assert!(matches!(
					find_innermost_manifest(path),
					Some(ref manifest_path) if manifest_path == &builder.workspace_manifest
				));
			});
		})
}

#[test]
fn find_innermost_manifest_doesnt_find_manifest_if_not_rust_dir() {
	TestBuilder::default().with_non_crate().build().execute(|builder| {
		builder
			.non_crate_paths
			.iter()
			.for_each(|path| assert!(find_innermost_manifest(path).is_none()));
	})
}

#[test]
fn find_workspace_manifest_finds_manifest_from_different_parts_of_a_workspace() {
	TestBuilder::default()
		.tempdir_is_workspace()
		.with_crate()
		.with_non_crate()
		.build()
		.execute(|builder| {
			builder.crate_paths.iter().for_each(|path| {
				assert!(matches!(
					find_workspace_manifest(path),
					Some(ref manifest_path) if manifest_path == &builder.workspace_manifest
				));
			});

			builder.non_crate_paths.iter().for_each(|path| {
				assert!(matches!(
					find_workspace_manifest(path),
					Some(ref manifest_path) if manifest_path == &builder.workspace_manifest
				));
			});
		});
}

#[test]
fn find_workspace_manifest_doesnt_find_manifest_if_not_workspace() {
	TestBuilder::default().with_crate().with_non_crate().build().execute(|builder| {
		builder
			.crate_paths
			.iter()
			.for_each(|path| assert!(find_workspace_manifest(path).is_none()));

		builder
			.non_crate_paths
			.iter()
			.for_each(|path| assert!(find_workspace_manifest(path).is_none()));
	})
}

#[test]
fn find_crate_name_finds_name_if_crate_manifest_path_used() {
	TestBuilder::default().with_crate().build().execute(|builder| {
		assert_eq!(
			find_crate_name(&builder.crate_manifest).expect("This should be Some; qed;"),
			"test"
		);
	});
}

#[test]
fn find_crate_doesnt_finds_name_if_not_crate_manifest_path_used() {
	TestBuilder::default().build().execute(|builder| {
		assert!(find_crate_name(&builder.tempdir.path()).is_none());
	});
}

#[test]
fn add_crate_to_dependencies_adds_workspace_dependency_without_default_features_to_crate_manifest()
{
	TestBuilder::default().with_crate().build().execute(|builder| {
		assert!(add_crate_to_dependencies(
			&builder.crate_manifest,
			"dependency",
			ManifestDependencyConfig::new(
				ManifestDependencyOrigin::workspace(),
				false,
				vec![],
				false
			)
		)
		.is_ok());

		assert_eq!(
			std::fs::read_to_string(&builder.crate_manifest)
				.expect("This should be readable; qed;"),
			r#"
[package]
name = "test"
version = "0.1.0"
edition = "2021"

[dependencies]
dependency = { workspace = true, default-features = false }
        "#
		);
	});
}

#[test]
fn add_crate_to_dependencies_adds_crates_io_optional_dependency_to_workspace_manifest() {
	TestBuilder::default().tempdir_is_workspace().build().execute(|builder| {
		assert!(add_crate_to_dependencies(
			&builder.workspace_manifest,
			"dependency",
			ManifestDependencyConfig::new(
				ManifestDependencyOrigin::crates_io("1.0.0"),
				true,
				vec![],
				true
			)
		)
		.is_ok());

		assert_eq!(
			std::fs::read_to_string(&builder.workspace_manifest)
				.expect("This should be readable; qed;"),
			r#"
[workspace]
resolver = "2"
members = ["crate"]

[workspace.dependencies]
dependency = { version = "1.0.0", optional = true }
        "#
		);
	});
}

#[test]
fn add_crate_to_dependencies_adds_git_dependency_with_features_to_crate_manifest() {
	TestBuilder::default().with_crate().build().execute(|builder| {
		assert!(add_crate_to_dependencies(
			&builder.crate_manifest,
			"dependency",
			ManifestDependencyConfig::new(
				ManifestDependencyOrigin::git("https://some_url.com", "stable"),
				true,
				vec!["feature_a", "feature_b"],
				false
			)
		)
		.is_ok());

		assert_eq!(
			std::fs::read_to_string(&builder.crate_manifest)
				.expect("This should be readable; qed;"),
			r#"
[package]
name = "test"
version = "0.1.0"
edition = "2021"

[dependencies]
dependency = { git = "https://some_url.com", branch = "stable", features = ["feature_a", "feature_b"] }
        "#
		);
	});
}

#[test]
fn add_crate_to_dependencies_adds_local_dependency_to_workspace_manifest() {
	TestBuilder::default().tempdir_is_workspace().build().execute(|builder| {
		assert!(add_crate_to_dependencies(
			&builder.workspace_manifest,
			"dependency",
			ManifestDependencyConfig::new(
				ManifestDependencyOrigin::local("../path".as_ref()),
				true,
				vec![],
				false
			)
		)
		.is_ok());

		assert_eq!(
			std::fs::read_to_string(&builder.workspace_manifest)
				.expect("This should be readable; qed;"),
			r#"
[workspace]
resolver = "2"
members = ["crate"]

[workspace.dependencies]
dependency = { path = "../path" }
        "#
		);
	});
}

#[test]
fn add_crate_to_dependencies_works_for_empty_manifest() {
	TestBuilder::default().with_crate().build().execute(|builder| {
		std::fs::write(&builder.crate_manifest, "").expect("Manifest should be writable; qed;");
		assert!(add_crate_to_dependencies(
			&builder.crate_manifest,
			"dependency",
			ManifestDependencyConfig::new(
				ManifestDependencyOrigin::workspace(),
				true,
				vec![],
				false
			)
		)
		.is_ok());
		assert_eq!(
			std::fs::read_to_string(&builder.crate_manifest)
				.expect("This should be readable; qed;"),
			r#"[dependencies]
dependency = { workspace = true }
"#
		);
	});
}

#[test]
fn add_crate_to_dependencies_fails_if_manifest_path_isnt_readable() {
	TestBuilder::default().build().execute(|builder| {
		assert!(matches!(
			add_crate_to_dependencies(
			builder.tempdir.path().join("unexisting/path/Cargo.toml"),
				"dependency",
			ManifestDependencyConfig::new(ManifestDependencyOrigin::workspace(), false, vec![], false)
			),
			Err(Error::IO(err)) if err.kind() == ErrorKind::NotFound
		));
	});
}

#[test]
fn add_crate_to_dependencies_fails_if_manifest_path_cannot_be_parsed() {
	TestBuilder::default().with_crate().build().execute(|builder| {
		assert!(matches!(
			add_crate_to_dependencies(
				&builder.crate_paths[3], // main.rs path
				"dependency",
				ManifestDependencyConfig::new(
					ManifestDependencyOrigin::workspace(),
					false,
					vec![],
					false
				)
			),
			Err(Error::TomlEdit(_))
		));
	});
}

#[test]
fn add_crate_to_dependencies_fails_if_manifest_path_cannot_be_written() {
	TestBuilder::default()
		.tempdir_is_workspace()
		.with_read_only_manifest()
		.build()
		.execute(|builder| {
			assert!(matches!(
				add_crate_to_dependencies(
				&builder.workspace_manifest,
					"dependency",
			ManifestDependencyConfig::new(ManifestDependencyOrigin::workspace(), false, vec![], false)
				),
				Err(Error::IO(err)) if err.kind() == ErrorKind::PermissionDenied
			));
		});
}
