// SPDX-License-Identifier: GPL-3.0

use super::*;
use crate::universal_test_builder::{
	UniversalTestBuilder,
	builders::{CallingDirOverride, ProjectKind, ProjectMember, TempRustProjectArgs},
};
use std::{io::ErrorKind, path::Component};

fn crate_args() -> TempRustProjectArgs {
	TempRustProjectArgs {
		kind: ProjectKind::Crate {
			files: vec![(PathBuf::from("src/main.rs"), "use std::fs;".to_string())],
		},
		..Default::default()
	}
}

fn workspace_args(
	with_crate: bool,
	with_non_crate: bool,
	read_only_manifest: bool,
	calling_dir_override: Option<CallingDirOverride>,
) -> TempRustProjectArgs {
	let mut members = Vec::new();
	if with_crate {
		members.push(ProjectMember {
			name: "crate".to_string(),
			files: vec![(PathBuf::from("src/main.rs"), "use std::fs;".to_string())],
			read_only_manifest: false,
		});
	}

	let non_crate_paths = if with_non_crate {
		vec![PathBuf::from("somewhere"), PathBuf::from("somewhere/somewhere_deeper")]
	} else {
		vec![]
	};

	TempRustProjectArgs {
		kind: ProjectKind::Workspace { members, non_crate_paths },
		read_only_manifest,
		calling_dir_override,
	}
}

#[test]
fn find_innermost_manifest_finds_manifest_from_different_parts_of_a_crate() {
	UniversalTestBuilder::default()
		.with_temp_rust_project(crate_args())
		.build()
		.execute(|context| {
			let project = context.temp_rust_project.as_ref().unwrap();
			let paths = [
				project.project_path.clone(),
				project.manifest_path.clone(),
				project.project_path.join("src"),
				project.project_path.join("src/main.rs"),
				project.project_path.join("src/lib.rs"),
			];
			for path in &paths {
				assert!(matches!(
					find_innermost_manifest(path),
					Some(ref manifest_path) if manifest_path == &project.manifest_path
				));
			}
		})
}

#[test]
fn find_innermost_manifest_finds_manifest_from_different_parts_of_a_crate_if_called_from_crate_root()
 {
	UniversalTestBuilder::default()
		.with_temp_rust_project(TempRustProjectArgs {
			kind: ProjectKind::Crate {
				files: vec![(PathBuf::from("src/main.rs"), "use std::fs;".to_string())],
			},
			calling_dir_override: Some(CallingDirOverride::ProjectRoot),
			..Default::default()
		})
		.build()
		.execute(|context| {
			let expected =
				<Component<'_> as AsRef<Path>>::as_ref(&Component::CurDir).join("Cargo.toml");
			let paths: Vec<PathBuf> = vec![
				".".into(),
				"Cargo.toml".into(),
				"src".into(),
				"src/main.rs".into(),
				"src/lib.rs".into(),
			];
			for path in &paths {
				assert!(matches!(
					find_innermost_manifest(path),
					Some(ref manifest_path) if manifest_path == &expected
				));
			}
		})
}

#[test]
fn find_innermost_manifest_finds_right_manifest_from_different_parts_of_a_workspace() {
	UniversalTestBuilder::default()
		.with_temp_rust_project(workspace_args(true, true, false, None))
		.build()
		.execute(|context| {
			let project = context.temp_rust_project.as_ref().unwrap();
			let crate_manifest = &project.member_manifests[0];
			let crate_path = &project.member_paths[0];

			// For crate paths, the innermost manifest is the crate manifest
			let crate_paths = [
				crate_path.clone(),
				crate_manifest.clone(),
				crate_path.join("src"),
				crate_path.join("src/main.rs"),
				crate_path.join("src/lib.rs"),
			];
			for path in &crate_paths {
				assert!(matches!(
					find_innermost_manifest(path),
					Some(ref m) if m == crate_manifest
				));
			}

			// While for these paths, the innermost manifest is the workspace manifest
			let non_crate_paths = [project.project_path.clone(), project.manifest_path.clone()];
			for path in non_crate_paths.iter().chain(project.non_crate_paths.iter()) {
				assert!(matches!(
					find_innermost_manifest(path),
					Some(ref m) if m == &project.manifest_path
				));
			}
		})
}

#[test]
fn find_innermost_manifest_finds_right_manifest_from_different_parts_of_a_workspace_if_called_from_crate_root()
 {
	UniversalTestBuilder::default()
		.with_temp_rust_project(workspace_args(
			true,
			false,
			false,
			Some(CallingDirOverride::MemberCrate("crate".to_string())),
		))
		.build()
		.execute(|_context| {
			let expected =
				<Component<'_> as AsRef<Path>>::as_ref(&Component::CurDir).join("Cargo.toml");
			let paths: Vec<PathBuf> = vec![
				".".into(),
				"Cargo.toml".into(),
				"src".into(),
				"src/main.rs".into(),
				"src/lib.rs".into(),
			];
			for path in &paths {
				assert!(matches!(
					find_innermost_manifest(path),
					Some(ref m) if m == &expected
				));
			}
		})
}

#[test]
fn find_innermost_manifest_finds_right_manifest_from_different_parts_of_a_workspace_if_called_from_workspace_root()
 {
	UniversalTestBuilder::default()
		.with_temp_rust_project(workspace_args(
			true,
			true,
			false,
			Some(CallingDirOverride::ProjectRoot),
		))
		.build()
		.execute(|context| {
			let project = context.temp_rust_project.as_ref().unwrap();
			let crate_manifest_rel = <Component<'_> as AsRef<Path>>::as_ref(&Component::CurDir)
				.join("crate")
				.join("Cargo.toml");
			let workspace_manifest_rel =
				<Component<'_> as AsRef<Path>>::as_ref(&Component::CurDir).join("Cargo.toml");

			// For crate paths (relative to workspace root), innermost is crate manifest
			let crate_paths: Vec<PathBuf> = vec![
				"crate".into(),
				"crate/Cargo.toml".into(),
				"crate/src".into(),
				"crate/src/main.rs".into(),
				"crate/src/lib.rs".into(),
			];
			for path in &crate_paths {
				assert!(matches!(
					find_innermost_manifest(path),
					Some(ref m) if m == &crate_manifest_rel
				));
			}

			// For non-crate paths (relative to workspace root), innermost is workspace manifest
			let non_crate_rel: Vec<PathBuf> = project
				.non_crate_paths
				.iter()
				.map(|p| p.strip_prefix(&project.project_path).unwrap().to_path_buf())
				.collect();
			for path in &non_crate_rel {
				assert!(matches!(
					find_innermost_manifest(path),
					Some(ref m) if m == &workspace_manifest_rel
				));
			}
		})
}

#[test]
fn find_innermost_manifest_doesnt_find_manifest_if_not_rust_dir() {
	UniversalTestBuilder::default().with_temp_dir(()).build().execute(|context| {
		let dir = context.temp_dir.as_ref().unwrap();
		let non_crate_path = dir.path().join("somewhere");
		let non_crate_inner = non_crate_path.join("somewhere_deeper");
		std::fs::create_dir_all(&non_crate_inner).unwrap();
		for path in [&non_crate_path, &non_crate_inner] {
			assert!(find_innermost_manifest(path).is_none());
		}
	})
}

#[test]
fn find_workspace_manifest_finds_manifest_from_different_parts_of_a_workspace() {
	UniversalTestBuilder::default()
		.with_temp_rust_project(workspace_args(true, true, false, None))
		.build()
		.execute(|context| {
			let project = context.temp_rust_project.as_ref().unwrap();
			let crate_path = &project.member_paths[0];

			let crate_paths = [
				crate_path.clone(),
				project.member_manifests[0].clone(),
				crate_path.join("src"),
				crate_path.join("src/main.rs"),
				crate_path.join("src/lib.rs"),
			];
			for path in crate_paths.iter().chain(
				[project.project_path.clone(), project.manifest_path.clone()]
					.iter()
					.chain(project.non_crate_paths.iter()),
			) {
				assert!(matches!(
					find_workspace_manifest(path),
					Some(ref m) if m == &project.manifest_path
				));
			}
		});
}

#[test]
fn find_workspace_manifest_finds_manifest_from_different_parts_of_a_workspace_if_called_from_the_workspace_root()
 {
	UniversalTestBuilder::default()
		.with_temp_rust_project(workspace_args(
			true,
			true,
			false,
			Some(CallingDirOverride::ProjectRoot),
		))
		.build()
		.execute(|context| {
			let project = context.temp_rust_project.as_ref().unwrap();
			let workspace_manifest_rel =
				<Component<'_> as AsRef<Path>>::as_ref(&Component::CurDir).join("Cargo.toml");

			let crate_paths: Vec<PathBuf> = vec![
				"crate".into(),
				"crate/Cargo.toml".into(),
				"crate/src".into(),
				"crate/src/main.rs".into(),
				"crate/src/lib.rs".into(),
			];
			let non_crate_rel: Vec<PathBuf> = project
				.non_crate_paths
				.iter()
				.map(|p| p.strip_prefix(&project.project_path).unwrap().to_path_buf())
				.collect();
			for path in crate_paths.iter().chain(non_crate_rel.iter()) {
				assert!(matches!(
					find_workspace_manifest(path),
					Some(ref m) if m == &workspace_manifest_rel
				));
			}
		});
}

#[test]
fn find_workspace_manifest_doesnt_find_manifest_if_not_workspace() {
	UniversalTestBuilder::default()
		.with_temp_rust_project(crate_args())
		.build()
		.execute(|context| {
			let project = context.temp_rust_project.as_ref().unwrap();
			// A standalone crate is not a workspace
			assert!(find_workspace_manifest(&project.project_path).is_none());
			assert!(find_workspace_manifest(&project.manifest_path).is_none());
			assert!(find_workspace_manifest(project.project_path.join("src")).is_none());
		})
}

#[test]
fn find_crate_name_finds_name_if_crate_manifest_path_used() {
	UniversalTestBuilder::default()
		.with_temp_rust_project(crate_args())
		.build()
		.execute(|context| {
			let project = context.temp_rust_project.as_ref().unwrap();
			assert_eq!(
				find_crate_name(&project.manifest_path).expect("This should be Some; qed;"),
				"test"
			);
		});
}

#[test]
fn find_crate_doesnt_finds_name_if_not_crate_manifest_path_used() {
	UniversalTestBuilder::default().with_temp_dir(()).build().execute(|context| {
		let dir = context.temp_dir.as_ref().unwrap();
		assert!(find_crate_name(&dir.path()).is_none());
	});
}

#[test]
fn add_dependency_to_dependencies_table_workspace_dependency() {
	UniversalTestBuilder::default()
		.with_temp_rust_project(crate_args())
		.build()
		.execute(|context| {
			let project = context.temp_rust_project.as_ref().unwrap();
			let mut doc = std::fs::read_to_string(&project.manifest_path)
				.unwrap()
				.parse::<DocumentMut>()
				.unwrap();
			let dependencies = doc["dependencies"].as_table_mut().unwrap();

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
	UniversalTestBuilder::default()
		.with_temp_rust_project(crate_args())
		.build()
		.execute(|context| {
			let project = context.temp_rust_project.as_ref().unwrap();
			let mut doc = std::fs::read_to_string(&project.manifest_path)
				.unwrap()
				.parse::<DocumentMut>()
				.unwrap();
			let dependencies = doc["dependencies"].as_table_mut().unwrap();

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
	UniversalTestBuilder::default()
		.with_temp_rust_project(crate_args())
		.build()
		.execute(|context| {
			let project = context.temp_rust_project.as_ref().unwrap();
			let mut doc = std::fs::read_to_string(&project.manifest_path)
				.unwrap()
				.parse::<DocumentMut>()
				.unwrap();
			let dependencies = doc["dependencies"].as_table_mut().unwrap();

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
	UniversalTestBuilder::default()
		.with_temp_rust_project(crate_args())
		.build()
		.execute(|context| {
			let project = context.temp_rust_project.as_ref().unwrap();
			let mut doc = std::fs::read_to_string(&project.manifest_path)
				.unwrap()
				.parse::<DocumentMut>()
				.unwrap();
			let dependencies = doc["dependencies"].as_table_mut().unwrap();

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
	UniversalTestBuilder::default()
		.with_temp_rust_project(crate_args())
		.build()
		.execute(|context| {
			let project = context.temp_rust_project.as_ref().unwrap();
			let mut doc = std::fs::read_to_string(&project.manifest_path)
				.unwrap()
				.parse::<DocumentMut>()
				.unwrap();
			let dependencies = doc["dependencies"].as_table_mut().unwrap();

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
	UniversalTestBuilder::default()
		.with_temp_rust_project(crate_args())
		.build()
		.execute(|context| {
			let project = context.temp_rust_project.as_ref().unwrap();
			let mut doc = std::fs::read_to_string(&project.manifest_path)
				.unwrap()
				.parse::<DocumentMut>()
				.unwrap();
			let dependencies = doc["dependencies"].as_table_mut().unwrap();

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
	UniversalTestBuilder::default()
		.with_temp_rust_project(crate_args())
		.build()
		.execute(|context| {
			let project = context.temp_rust_project.as_ref().unwrap();
			let mut doc = std::fs::read_to_string(&project.manifest_path)
				.unwrap()
				.parse::<DocumentMut>()
				.unwrap();
			let dependencies = doc["dependencies"].as_table_mut().unwrap();

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
	UniversalTestBuilder::default()
		.with_temp_rust_project(crate_args())
		.build()
		.execute(|context| {
			let project = context.temp_rust_project.as_ref().unwrap();
			assert!(
				add_crate_to_dependencies(
					&project.manifest_path,
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
				std::fs::read_to_string(&project.manifest_path)
					.expect("This should be readable; qed;"),
				"[package]\nname = \"test\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\ndependency = { path = \"../path\" }\n"
			);
		});
}

#[test]
fn add_crate_to_dependencies_workspace_manifest_with_dependencies_section() {
	UniversalTestBuilder::default()
		.with_temp_rust_project(workspace_args(false, false, false, None))
		.build()
		.execute(|context| {
			let project = context.temp_rust_project.as_ref().unwrap();
			assert!(
				add_crate_to_dependencies(
					&project.manifest_path,
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

			let manifest = std::fs::read_to_string(&project.manifest_path)
				.expect("This should be readable; qed;");
			assert!(manifest.contains("[workspace.dependencies]"));
			assert!(manifest.contains("dependency = { path = \"../path\" }"));
		});
}

#[test]
fn add_crate_to_dependencies_crate_manifest_without_dependencies_section() {
	UniversalTestBuilder::default()
		.with_temp_rust_project(crate_args())
		.build()
		.execute(|context| {
			let project = context.temp_rust_project.as_ref().unwrap();
			std::fs::write(
				&project.manifest_path,
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
					&project.manifest_path,
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
				std::fs::read_to_string(&project.manifest_path)
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
	UniversalTestBuilder::default()
		.with_temp_rust_project(workspace_args(false, false, false, None))
		.build()
		.execute(|context| {
			let project = context.temp_rust_project.as_ref().unwrap();
			std::fs::write(
				&project.manifest_path,
				r#"
[workspace]
resolver = "2"
members = []
"#,
			)
			.expect("Manifest should be writable; qed;");

			assert!(
				add_crate_to_dependencies(
					&project.manifest_path,
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
				std::fs::read_to_string(&project.manifest_path)
					.expect("This should be readable; qed;"),
				r#"
[workspace]
resolver = "2"
members = []

[workspace.dependencies]
dependency = { version = "0.1.0" }
"#
			);
		});
}

#[test]
fn add_crate_to_dependencies_works_for_empty_manifest() {
	UniversalTestBuilder::default()
		.with_temp_rust_project(crate_args())
		.build()
		.execute(|context| {
			let project = context.temp_rust_project.as_ref().unwrap();
			std::fs::write(&project.manifest_path, "").expect("Manifest should be writable; qed;");
			assert!(
				add_crate_to_dependencies(
					&project.manifest_path,
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
				std::fs::read_to_string(&project.manifest_path)
					.expect("This should be readable; qed;"),
				r#"[dependencies]
dependency = { workspace = true }
"#
			);
		});
}

#[test]
fn add_crate_to_dependencies_fails_if_manifest_path_isnt_readable() {
	UniversalTestBuilder::default().with_temp_dir(()).build().execute(|context| {
		let dir = context.temp_dir.as_ref().unwrap();
		assert!(matches!(
			add_crate_to_dependencies(
				dir.path().join("unexisting/path/Cargo.toml"),
				"dependency",
				ManifestDependencyConfig::new(
					ManifestDependencyOrigin::workspace(),
					false,
					vec![],
					false
				)
			),
			Err(Error::IO(err)) if err.kind() == ErrorKind::NotFound
		));
	});
}

#[test]
fn add_crate_to_dependencies_fails_if_manifest_path_cannot_be_parsed() {
	UniversalTestBuilder::default()
		.with_temp_rust_project(crate_args())
		.build()
		.execute(|context| {
			let project = context.temp_rust_project.as_ref().unwrap();
			// Use main.rs (not a valid manifest) as the manifest path
			assert!(matches!(
				add_crate_to_dependencies(
					project.project_path.join("src/main.rs"),
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
	UniversalTestBuilder::default()
		.with_temp_rust_project(workspace_args(false, false, true, None))
		.build()
		.execute(|context| {
			let project = context.temp_rust_project.as_ref().unwrap();
			assert!(matches!(
				add_crate_to_dependencies(
					&project.manifest_path,
					"dependency",
					ManifestDependencyConfig::new(
						ManifestDependencyOrigin::workspace(),
						false,
						vec![],
						false
					)
				),
				Err(Error::IO(err)) if err.kind() == ErrorKind::PermissionDenied
			));
		});
}

#[test]
fn add_crate_to_workspace_works_if_members_exist() {
	UniversalTestBuilder::default()
		.with_temp_rust_project(workspace_args(true, false, false, None))
		.build()
		.execute(|context| {
			let project = context.temp_rust_project.as_ref().unwrap();
			let crate_path = project.project_path.join("crate2");
			assert!(add_crate_to_workspace(&project.manifest_path, crate_path).is_ok());

			let doc = std::fs::read_to_string(&project.manifest_path)
				.unwrap()
				.parse::<DocumentMut>()
				.unwrap();
			if let Some(Item::Table(workspace_table)) = doc.get("workspace") {
				if let Some(Item::Value(members_array)) = workspace_table.get("members") {
					if let Value::Array(array) = members_array {
						assert!(array.into_iter().any(|member| {
							if let Value::String(member) = member {
								member.clone().into_value() == "crate2"
							} else {
								false
							}
						}));
					} else {
						panic!("Failed");
					}
				} else {
					panic!("Failed");
				}
			} else {
				panic!("Failed");
			}
		});
}

#[test]
fn add_crate_to_workspace_works_if_members_doesnt_exist() {
	UniversalTestBuilder::default()
		.with_temp_rust_project(workspace_args(true, false, false, None))
		.build()
		.execute(|context| {
			let project = context.temp_rust_project.as_ref().unwrap();
			std::fs::write(
				&project.manifest_path,
				r#"
[workspace]
resolver = "2"

[workspace.dependencies]
        "#,
			)
			.expect("The manifest should be writable; qed;");
			let crate_path = project.project_path.join("crate2");
			assert!(add_crate_to_workspace(&project.manifest_path, crate_path).is_ok());

			let doc = std::fs::read_to_string(&project.manifest_path)
				.unwrap()
				.parse::<DocumentMut>()
				.unwrap();
			if let Some(Item::Table(workspace_table)) = doc.get("workspace") {
				if let Some(Item::Value(members_array)) = workspace_table.get("members") {
					if let Value::Array(array) = members_array {
						assert!(array.into_iter().any(|member| {
							if let Value::String(member) = member {
								member.clone().into_value() == "crate2"
							} else {
								false
							}
						}));
					} else {
						panic!("Failed");
					}
				} else {
					panic!("Failed");
				}
			} else {
				panic!("Failed");
			}
		});
}

#[test]
fn add_crate_to_workspace_hasnt_effect_if_member_already_present() {
	UniversalTestBuilder::default()
		.with_temp_rust_project(workspace_args(true, false, false, None))
		.build()
		.execute(|context| {
			let project = context.temp_rust_project.as_ref().unwrap();
			let crate_path = project.project_path.join("crate");
			let manifest_before_call = std::fs::read_to_string(&project.manifest_path).unwrap();
			assert!(add_crate_to_workspace(&project.manifest_path, crate_path).is_ok());
			let manifest_after_call = std::fs::read_to_string(&project.manifest_path).unwrap();
			assert_eq!(manifest_before_call, manifest_after_call);
		});
}

#[test]
fn add_crate_to_workspace_fails_if_the_workspace_manifest_path_cannot_be_read() {
	UniversalTestBuilder::default().with_temp_dir(()).build().execute(|context| {
		let dir = context.temp_dir.as_ref().unwrap();
		assert!(matches!(
			add_crate_to_workspace(
				dir.path().join("unexisting/path/Cargo.toml"),
				"dependency",
			),
			Err(Error::IO(err)) if err.kind() == ErrorKind::NotFound
		));
	});
}

#[test]
fn add_crate_to_workspace_fails_if_manifest_path_cannot_be_parsed_as_manifest() {
	UniversalTestBuilder::default()
		.with_temp_rust_project(crate_args())
		.build()
		.execute(|context| {
			let project = context.temp_rust_project.as_ref().unwrap();
			assert!(matches!(
				add_crate_to_workspace(project.project_path.join("src/main.rs"), "dependency",),
				Err(Error::TomlEdit(_))
			));
		});
}

#[test]
fn add_crate_to_workspace_fails_if_crate_path_isnt_prefixed_by_workspace_path() {
	UniversalTestBuilder::default()
		.with_temp_rust_project(workspace_args(true, false, false, None))
		.build()
		.execute(|context| {
			let project = context.temp_rust_project.as_ref().unwrap();
			assert!(matches!(
				add_crate_to_workspace(&project.manifest_path, "dependency",),
				Err(Error::StripPrefixError(_))
			));
		});
}

#[test]
fn add_crate_to_workspace_fails_if_members_section_isnt_an_array() {
	UniversalTestBuilder::default()
		.with_temp_rust_project(workspace_args(true, false, false, None))
		.build()
		.execute(|context| {
			let project = context.temp_rust_project.as_ref().unwrap();
			std::fs::write(
				&project.manifest_path,
				r#"
[workspace]
resolver = "2"
members = "1"

[workspace.dependencies]
        "#,
			)
			.expect("The manifest should be writable; qed;");

			assert!(matches!(
				add_crate_to_workspace(
					&project.manifest_path,
					&project.project_path.join("dependency"),
				),
				Err(Error::Descriptive(expected_error)) if expected_error == "The provided manifest path members field is corrupted"
			));
		});
}

#[test]
fn add_crate_to_workspace_fails_if_the_target_manifest_isnt_a_workspace_manifest() {
	UniversalTestBuilder::default()
		.with_temp_rust_project(crate_args())
		.build()
		.execute(|context| {
			let project = context.temp_rust_project.as_ref().unwrap();
			assert!(matches!(
				add_crate_to_workspace(
					&project.manifest_path,
					&project.project_path.join("dependency"),
				),
				Err(Error::Descriptive(expected_error)) if expected_error == "The provided manifest path isn't a workspace manifest"
			));
		});
}

#[test]
fn add_crate_to_workspace_fails_if_manifest_path_cannot_be_written() {
	UniversalTestBuilder::default()
		.with_temp_rust_project(workspace_args(false, false, true, None))
		.build()
		.execute(|context| {
			let project = context.temp_rust_project.as_ref().unwrap();
			assert!(matches!(
				add_crate_to_workspace(
					&project.manifest_path,
					&project.project_path.join("dependency"),
				),
				Err(Error::IO(err)) if err.kind() == ErrorKind::PermissionDenied
			));
		});
}
