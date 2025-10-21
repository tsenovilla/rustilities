// SPDX-License-Identifier: GPL-3.0

use super::*;
use std::{
	fs::{File, Permissions},
	io::ErrorKind,
	os::unix::fs::PermissionsExt,
	path::Component,
};
use tempfile::TempDir;

struct TestBuilder {
	tempdir: TempDir,
	crate_paths: Vec<PathBuf>,
	non_crate_paths: Vec<PathBuf>,
	crate_manifest: PathBuf,
	workspace_manifest: PathBuf,
	crate_depencencies_table: Option<Table>,
	tempdir_is_workspace: bool,
	with_crate: bool,
	with_non_crate: bool,
	with_read_only_manifest: bool,
	calling_dir_override: Option<CallingDirOverride>,
	calling_dir_override_path: Option<PathBuf>,
	workspace_dir_from_overrided_calling_dir: Option<PathBuf>,
	crate_dir_from_overrided_calling_dir: Option<PathBuf>,
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
			crate_depencencies_table: None,
			tempdir_is_workspace: false,
			with_crate: false,
			with_non_crate: false,
			with_read_only_manifest: false,
			calling_dir_override: None,
			calling_dir_override_path: None,
			workspace_dir_from_overrided_calling_dir: None,
			crate_dir_from_overrided_calling_dir: None,
		}
	}
}

enum CallingDirOverride {
	WorkspaceRoot,
	CrateRoot,
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

	fn with_calling_dir_override(mut self, calling_dir: CallingDirOverride) -> Self {
		self.calling_dir_override = Some(calling_dir);
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

			let mut doc = std::fs::read_to_string(&self.crate_manifest)
				.expect("The crate manifest should be readable; qed;")
				.parse::<DocumentMut>()
				.expect("The crate manifest is a valid toml; qed;");

			self.crate_depencencies_table =
				if let Some(Item::Table(dependencies)) = doc.get_mut("dependencies") {
					Some(dependencies.clone())
				} else {
					unreachable!("Item defined in manifest; qed;")
				};
		}

		if self.with_non_crate {
			let non_crate_path = self.tempdir.path().join("somewhere");
			let non_crate_inner_path = non_crate_path.join("somewhere_deeper");
			std::fs::create_dir_all(&non_crate_inner_path).expect("This should be created; qed;");
			self.non_crate_paths.extend_from_slice(&[non_crate_path, non_crate_inner_path]);
		}

		if let Some(calling_dir_override) = &self.calling_dir_override {
			match calling_dir_override {
				CallingDirOverride::WorkspaceRoot => {
					self.calling_dir_override_path = Some(self.tempdir.path().to_path_buf());
					self.workspace_dir_from_overrided_calling_dir = Some(
						<Component<'_> as AsRef<Path>>::as_ref(&Component::CurDir)
							.join("Cargo.toml"),
					);
					self.crate_dir_from_overrided_calling_dir = Some(
						<Component<'_> as AsRef<Path>>::as_ref(&Component::CurDir)
							.join("crate")
							.join("Cargo.toml"),
					);
				},
				CallingDirOverride::CrateRoot if !self.crate_paths.is_empty() => {
					self.calling_dir_override_path = Some(self.crate_paths[0].clone());
					self.crate_dir_from_overrided_calling_dir = Some(
						<Component<'_> as AsRef<Path>>::as_ref(&Component::CurDir)
							.join("Cargo.toml"),
					);
				},
				_ => panic!(
					"If calling dir is crate, the builder needs to be built using `with_crate`"
				),
			}

			self.crate_paths = self
				.crate_paths
				.iter()
				.map(|path| {
					path.strip_prefix(&self.calling_dir_override_path.as_ref().unwrap())
						.unwrap_or(path)
						.to_path_buf()
				})
				.collect();

			self.non_crate_paths = self
				.non_crate_paths
				.iter()
				.map(|path| {
					path.strip_prefix(&self.calling_dir_override_path.as_ref().unwrap())
						.unwrap_or(path)
						.to_path_buf()
				})
				.collect();
		}

		self
	}

	fn execute<F>(&mut self, test: F)
	where
		F: Fn(&mut Self) -> (),
	{
		if let Some(calling_dir) = &self.calling_dir_override_path {
			let original_dir = std::env::current_dir().unwrap();

			std::env::set_current_dir(calling_dir).unwrap();
			test(self);
			std::env::set_current_dir(original_dir).unwrap();
		} else {
			test(self);
		}
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
fn find_innermost_manifest_finds_manifest_from_different_parts_of_a_crate_if_called_from_crate_root()
 {
	TestBuilder::default()
		.with_crate()
		.with_calling_dir_override(CallingDirOverride::CrateRoot)
		.build()
		.execute(|builder| {
            builder.crate_paths.iter().for_each(|path|{
			assert!(matches!(
				find_innermost_manifest(path),
				Some(ref manifest_path) if manifest_path == builder.crate_dir_from_overrided_calling_dir.as_ref().unwrap()
			));
		});});
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
fn find_innermost_manifest_finds_right_manifest_from_different_parts_of_a_workspace_if_called_from_crate_root()
 {
	TestBuilder::default()
		.tempdir_is_workspace()
		.with_crate()
		.with_calling_dir_override(CallingDirOverride::CrateRoot)
		.build()
		.execute(|builder| {
			// For crate paths, the innermost manifest is the crate manifest
			builder.crate_paths.iter().for_each(|path| {
				assert!(matches!(
					find_innermost_manifest(path),
					Some(ref manifest_path) if manifest_path == builder.crate_dir_from_overrided_calling_dir.as_ref().unwrap()
				));
			});
		})
}

#[test]
fn find_innermost_manifest_finds_right_manifest_from_different_parts_of_a_workspace_if_called_from_workspace_root()
 {
	TestBuilder::default()
		.tempdir_is_workspace()
		.with_crate()
		.with_non_crate()
		.with_calling_dir_override(CallingDirOverride::WorkspaceRoot)
		.build()
		.execute(|builder| {
			// For crate paths, the innermost manifest is the crate manifest
			builder.crate_paths.iter().for_each(|path| {
				assert!(matches!(
					find_innermost_manifest(path),
					Some(ref manifest_path) if manifest_path == builder.crate_dir_from_overrided_calling_dir.as_ref().unwrap()
				));
			});

			// While for these paths, the innermost manifest is the workspace manifest
			builder.non_crate_paths.iter().for_each(|path| {
				assert!(matches!(
					find_innermost_manifest(path),
					Some(ref manifest_path) if manifest_path == builder.workspace_dir_from_overrided_calling_dir.as_ref().unwrap()
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
fn find_workspace_manifest_finds_manifest_from_different_parts_of_a_workspace_if_called_from_the_workspace_root()
 {
	TestBuilder::default()
		.tempdir_is_workspace()
		.with_crate()
		.with_non_crate()
		.with_calling_dir_override(CallingDirOverride::WorkspaceRoot)
		.build()
		.execute(|builder| {
			builder.crate_paths.iter().for_each(|path| {
				assert!(matches!(
					find_workspace_manifest(path),
					Some(ref manifest_path) if manifest_path == builder.workspace_dir_from_overrided_calling_dir.as_ref().unwrap()
				));
			});

			builder.non_crate_paths.iter().for_each(|path| {
				assert!(matches!(
					find_workspace_manifest(path),
					Some(ref manifest_path) if manifest_path == builder.workspace_dir_from_overrided_calling_dir.as_ref().unwrap()
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
fn add_dependency_to_dependencies_table_workspace_dependency() {
	TestBuilder::default().with_crate().build().execute(|builder| {
		let dependencies =
			builder.crate_depencencies_table.as_mut().expect("This should be Some; qed;");

		add_dependency_to_dependencies_table(
			dependencies,
			"dependency",
			ManifestDependencyConfig::new(
				ManifestDependencyOrigin::workspace(),
				true,
				vec![],
				false,
			),
		);

		assert_eq!(dependencies.to_string(), "dependency = { workspace = true }\n");
	});
}

#[test]
fn add_dependency_to_dependencies_table_crates_io_dependency() {
	TestBuilder::default().with_crate().build().execute(|builder| {
		let dependencies =
			builder.crate_depencencies_table.as_mut().expect("This should be Some; qed;");

		add_dependency_to_dependencies_table(
			dependencies,
			"dependency",
			ManifestDependencyConfig::new(
				ManifestDependencyOrigin::crates_io("1.0.0"),
				true,
				vec![],
				false,
			),
		);

		assert_eq!(dependencies.to_string(), "dependency = { version = \"1.0.0\" }\n");
	});
}

#[test]
fn add_dependency_to_dependencies_table_git_dependency() {
	TestBuilder::default().with_crate().build().execute(|builder| {
		let dependencies =
			builder.crate_depencencies_table.as_mut().expect("This should be Some; qed;");

		add_dependency_to_dependencies_table(
			dependencies,
			"dependency",
			ManifestDependencyConfig::new(
				ManifestDependencyOrigin::git("https://some_url.com", "stable"),
				true,
				vec![],
				false,
			),
		);

		assert_eq!(
			dependencies.to_string(),
			"dependency = { git = \"https://some_url.com\", branch = \"stable\" }\n"
		);
	});
}

#[test]
fn add_dependency_to_dependencies_table_local_dependency() {
	TestBuilder::default().with_crate().build().execute(|builder| {
		let dependencies =
			builder.crate_depencencies_table.as_mut().expect("This should be Some; qed;");

		add_dependency_to_dependencies_table(
			dependencies,
			"dependency",
			ManifestDependencyConfig::new(
				ManifestDependencyOrigin::local("../path".as_ref()),
				true,
				vec![],
				false,
			),
		);

		assert_eq!(dependencies.to_string(), "dependency = { path = \"../path\" }\n");
	});
}

#[test]
fn add_dependency_to_dependencies_table_dependency_no_default_features() {
	TestBuilder::default().with_crate().build().execute(|builder| {
		let dependencies =
			builder.crate_depencencies_table.as_mut().expect("This should be Some; qed;");

		add_dependency_to_dependencies_table(
			dependencies,
			"dependency",
			ManifestDependencyConfig::new(
				ManifestDependencyOrigin::local("../path".as_ref()),
				false,
				vec![],
				false,
			),
		);

		assert_eq!(
			dependencies.to_string(),
			"dependency = { path = \"../path\", default-features = false }\n"
		);
	});
}

#[test]
fn add_dependency_to_dependencies_table_dependency_with_features() {
	TestBuilder::default().with_crate().build().execute(|builder| {
		let dependencies =
			builder.crate_depencencies_table.as_mut().expect("This should be Some; qed;");

		add_dependency_to_dependencies_table(
			dependencies,
			"dependency",
			ManifestDependencyConfig::new(
				ManifestDependencyOrigin::local("../path".as_ref()),
				true,
				vec!["feature_a", "feature_b"],
				false,
			),
		);

		assert_eq!(
			dependencies.to_string(),
			"dependency = { path = \"../path\", features = [\"feature_a\", \"feature_b\"] }\n"
		);
	});
}

#[test]
fn add_dependency_to_dependencies_table_optional_dependency() {
	TestBuilder::default().with_crate().build().execute(|builder| {
		let dependencies =
			builder.crate_depencencies_table.as_mut().expect("This should be Some; qed;");

		add_dependency_to_dependencies_table(
			dependencies,
			"dependency",
			ManifestDependencyConfig::new(
				ManifestDependencyOrigin::local("../path".as_ref()),
				true,
				vec![],
				true,
			),
		);

		assert_eq!(
			dependencies.to_string(),
			"dependency = { path = \"../path\", optional = true }\n"
		);
	});
}

#[test]
fn add_crate_to_dependencies_crate_manifest_with_dependencies_section() {
	TestBuilder::default().with_crate().build().execute(|builder| {
		assert!(
			add_crate_to_dependencies(
				&builder.crate_manifest,
				"dependency",
				ManifestDependencyConfig::new(
					ManifestDependencyOrigin::local("../path".as_ref()),
					true,
					vec![],
					false
				)
			)
			.is_ok()
		);

		assert_eq!(
			std::fs::read_to_string(&builder.crate_manifest)
				.expect("This should be readable; qed;"),
			r#"
[package]
name = "test"
version = "0.1.0"
edition = "2021"

[dependencies]
dependency = { path = "../path" }
        "#
		);
	});
}

#[test]
fn add_crate_to_dependencies_workspace_manifest_with_dependencies_section() {
	TestBuilder::default().tempdir_is_workspace().build().execute(|builder| {
		assert!(
			add_crate_to_dependencies(
				&builder.workspace_manifest,
				"dependency",
				ManifestDependencyConfig::new(
					ManifestDependencyOrigin::local("../path".as_ref()),
					true,
					vec![],
					false
				)
			)
			.is_ok()
		);

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
fn add_crate_to_dependencies_crate_manifest_without_dependencies_section() {
	TestBuilder::default().with_crate().build().execute(|builder| {
		std::fs::write(
			&builder.crate_manifest,
			r#"
[package]
name = "test"
version = "0.1.0"
edition = "2021"
"#,
		)
		.expect("Manifest should be writable; qed;");
		assert!(
			add_crate_to_dependencies(
				&builder.crate_manifest,
				"dependency",
				ManifestDependencyConfig::new(
					ManifestDependencyOrigin::workspace(),
					true,
					vec![],
					false
				)
			)
			.is_ok()
		);
		assert_eq!(
			std::fs::read_to_string(&builder.crate_manifest)
				.expect("This should be readable; qed;"),
			r#"
[package]
name = "test"
version = "0.1.0"
edition = "2021"

[dependencies]
dependency = { workspace = true }
"#
		);
	});
}

#[test]
fn add_crate_to_dependencies_workspace_manifest_without_dependencies_section() {
	TestBuilder::default().tempdir_is_workspace().build().execute(|builder| {
		std::fs::write(
			&builder.workspace_manifest,
			r#"
[workspace]
resolver = "2"
members = ["crate"]
"#,
		)
		.expect("Manifest should be writable; qed;");

		assert!(
			add_crate_to_dependencies(
				&builder.workspace_manifest,
				"dependency",
				ManifestDependencyConfig::new(
					ManifestDependencyOrigin::crates_io("0.1.0"),
					true,
					vec![],
					false
				)
			)
			.is_ok()
		);
		assert_eq!(
			std::fs::read_to_string(&builder.workspace_manifest)
				.expect("This should be readable; qed;"),
			r#"
[workspace]
resolver = "2"
members = ["crate"]

[workspace.dependencies]
dependency = { version = "0.1.0" }
"#
		);
	});
}

#[test]
fn add_crate_to_dependencies_works_for_empty_manifest() {
	TestBuilder::default().with_crate().build().execute(|builder| {
		std::fs::write(&builder.crate_manifest, "").expect("Manifest should be writable; qed;");
		assert!(
			add_crate_to_dependencies(
				&builder.crate_manifest,
				"dependency",
				ManifestDependencyConfig::new(
					ManifestDependencyOrigin::workspace(),
					true,
					vec![],
					false
				)
			)
			.is_ok()
		);
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
