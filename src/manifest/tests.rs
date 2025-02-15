// SPDX-License-Identifier: GPL-3.0

use super::*;
use std::fs::File;
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

            [dependencies]
        "#,
			)
			.expect("The manifest should be writable; qed;");
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
