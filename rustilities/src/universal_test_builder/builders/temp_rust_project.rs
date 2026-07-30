// SPDX-License-Identifier: GPL-3.0

use crate::universal_test_builder::Builder;
use std::path::{Path, PathBuf};

pub enum ProjectKind {
	Crate { files: Vec<(PathBuf, String)> },
	Workspace { members: Vec<ProjectMember>, non_crate_paths: Vec<PathBuf> },
}

pub struct ProjectMember {
	pub name: String,
	pub files: Vec<(PathBuf, String)>,
	pub read_only_manifest: bool,
}

pub enum CallingDirOverride {
	ProjectRoot,
	MemberCrate(String),
}

pub struct TempRustProjectArgs {
	pub kind: ProjectKind,
	pub read_only_manifest: bool,
	pub calling_dir_override: Option<CallingDirOverride>,
}

impl Default for TempRustProjectArgs {
	fn default() -> Self {
		Self {
			kind: ProjectKind::Crate { files: Vec::new() },
			read_only_manifest: false,
			calling_dir_override: None,
		}
	}
}

pub struct TempRustProjectOutput {
	pub tempdir: tempfile::TempDir,
	pub project_path: PathBuf,
	pub manifest_path: PathBuf,
	/// Only populated for workspaces.
	pub member_paths: Vec<PathBuf>,
	pub member_manifests: Vec<PathBuf>,
	pub non_crate_paths: Vec<PathBuf>,
	original_dir: Option<PathBuf>,
	read_only_paths: Vec<PathBuf>,
}

pub struct TempRustProject;

impl Builder for TempRustProject {
	type Args = TempRustProjectArgs;
	type Output = TempRustProjectOutput;

	fn build(args: Self::Args) -> Self::Output {
		let tempdir = tempfile::tempdir().unwrap();
		let project_path = tempdir.path().to_path_buf();
		let manifest_path = project_path.join("Cargo.toml");
		let mut read_only_paths = Vec::new();
		let mut member_paths = Vec::new();
		let mut member_manifests = Vec::new();
		let mut non_crate_full_paths = Vec::new();

		match &args.kind {
			ProjectKind::Crate { files } => {
				build_crate(&project_path, files, "test");
			},
			ProjectKind::Workspace { members, non_crate_paths } => {
				let mut member_names = Vec::new();

				for member in members {
					let member_path = project_path.join(&member.name);
					let member_manifest = member_path.join("Cargo.toml");

					build_crate(&member_path, &member.files, &member.name);

					if member.read_only_manifest {
						set_read_only(&member_manifest);
						read_only_paths.push(member_manifest.clone());
					}

					member_names.push(member.name.clone());
					member_paths.push(member_path);
					member_manifests.push(member_manifest);
				}

				for path in non_crate_paths {
					let full_path = project_path.join(path);
					std::fs::create_dir_all(&full_path).unwrap();
					non_crate_full_paths.push(full_path);
				}

				let members_str = member_names
					.iter()
					.map(|n| format!("\"{}\"", n))
					.collect::<Vec<_>>()
					.join(", ");
				let workspace_toml = format!(
					"[workspace]\nresolver = \"2\"\nmembers = [{}]\n\n[workspace.dependencies]\n",
					members_str
				);
				std::fs::write(&manifest_path, workspace_toml).unwrap();
			},
		}

		if args.read_only_manifest {
			set_read_only(&manifest_path);
			read_only_paths.push(manifest_path.clone());
		}

		let original_dir = if let Some(ref override_) = args.calling_dir_override {
			let original = std::env::current_dir().ok();
			let target = match override_ {
				CallingDirOverride::ProjectRoot => project_path.clone(),
				CallingDirOverride::MemberCrate(name) => project_path.join(name),
			};
			std::env::set_current_dir(&target).unwrap();
			original
		} else {
			None
		};

		TempRustProjectOutput {
			tempdir,
			project_path,
			manifest_path,
			member_paths,
			member_manifests,
			non_crate_paths: non_crate_full_paths,
			original_dir,
			read_only_paths,
		}
	}

	fn on_drop(output: Self::Output) {
		if let Some(original_dir) = output.original_dir {
			let _ = std::env::set_current_dir(original_dir);
		}
		for path in &output.read_only_paths {
			restore_permissions(path);
		}
	}
}

/// Creates a single crate at `crate_path` with default manifest/lib.rs if not provided.
fn build_crate(crate_path: &Path, files: &[(PathBuf, String)], name: &str) {
	let src_path = crate_path.join("src");
	std::fs::create_dir_all(&src_path).unwrap();

	let has_manifest = files.iter().any(|(p, _)| p == Path::new("Cargo.toml"));
	if !has_manifest {
		let manifest = format!(
			"[package]\nname = \"{}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\n",
			name
		);
		std::fs::write(crate_path.join("Cargo.toml"), manifest).unwrap();
	}

	let has_lib = files.iter().any(|(p, _)| p == Path::new("src/lib.rs"));
	if !has_lib {
		std::fs::write(src_path.join("lib.rs"), "").unwrap();
	}

	for (relative_path, content) in files {
		let full_path = crate_path.join(relative_path);
		if let Some(parent) = full_path.parent() {
			std::fs::create_dir_all(parent).unwrap();
		}
		std::fs::write(&full_path, content).unwrap();
	}
}

#[cfg(unix)]
fn set_read_only(path: &Path) {
	use std::os::unix::fs::PermissionsExt;
	std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o444)).unwrap();
}

#[cfg(not(unix))]
fn set_read_only(path: &Path) {
	let mut perms = std::fs::metadata(path).unwrap().permissions();
	perms.set_readonly(true);
	std::fs::set_permissions(path, perms).unwrap();
}

#[cfg(unix)]
fn restore_permissions(path: &Path) {
	use std::os::unix::fs::PermissionsExt;
	let _ = std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o644));
}

#[cfg(not(unix))]
fn restore_permissions(path: &Path) {
	if let Ok(metadata) = std::fs::metadata(path) {
		let mut perms = metadata.permissions();
		perms.set_readonly(false);
		let _ = std::fs::set_permissions(path, perms);
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::universal_test_builder::universal_test_builder;

	#[universal_test_builder({builder = TempRustProject})]
	struct ProjectTestBuilder;

	#[test]
	fn build_standalone_crate() {
		ProjectTestBuilder::default()
			.with_temp_rust_project(TempRustProjectArgs::default())
			.build()
			.execute(|context| {
				let output = context.temp_rust_project.as_ref().unwrap();
				assert!(output.project_path.exists());
				assert!(output.manifest_path.exists());
				assert!(output.project_path.join("src/lib.rs").exists());
				let manifest = std::fs::read_to_string(&output.manifest_path).unwrap();
				assert!(manifest.contains("name = \"test\""));
				assert!(output.member_paths.is_empty());
			});
	}

	#[test]
	fn build_standalone_crate_with_custom_files() {
		let custom_manifest =
			"[package]\nname = \"my-crate\"\nversion = \"1.0.0\"\nedition = \"2021\"\n".to_string();
		let args = TempRustProjectArgs {
			kind: ProjectKind::Crate {
				files: vec![
					(PathBuf::from("Cargo.toml"), custom_manifest.clone()),
					(PathBuf::from("src/lib.rs"), "pub fn hello() {}".to_string()),
					(PathBuf::from("src/utils.rs"), "pub fn util() {}".to_string()),
				],
			},
			..Default::default()
		};

		ProjectTestBuilder::default()
			.with_temp_rust_project(args)
			.build()
			.execute(|context| {
				let output = context.temp_rust_project.as_ref().unwrap();
				let manifest = std::fs::read_to_string(&output.manifest_path).unwrap();
				assert!(manifest.contains("my-crate"));
				let lib = std::fs::read_to_string(output.project_path.join("src/lib.rs")).unwrap();
				assert!(lib.contains("pub fn hello()"));
				assert!(output.project_path.join("src/utils.rs").exists());
			});
	}

	#[test]
	fn build_workspace() {
		let args = TempRustProjectArgs {
			kind: ProjectKind::Workspace {
				members: vec![
					ProjectMember {
						name: "crate-a".to_string(),
						files: vec![],
						read_only_manifest: false,
					},
					ProjectMember {
						name: "crate-b".to_string(),
						files: vec![(PathBuf::from("src/lib.rs"), "pub fn b() {}".to_string())],
						read_only_manifest: false,
					},
				],
				non_crate_paths: vec![PathBuf::from("shared")],
			},
			..Default::default()
		};

		ProjectTestBuilder::default()
			.with_temp_rust_project(args)
			.build()
			.execute(|context| {
				let output = context.temp_rust_project.as_ref().unwrap();
				let manifest = std::fs::read_to_string(&output.manifest_path).unwrap();
				assert!(manifest.contains("[workspace]"));
				assert!(manifest.contains("crate-a"));
				assert!(manifest.contains("crate-b"));
				assert_eq!(output.member_paths.len(), 2);
				assert_eq!(output.member_manifests.len(), 2);
				assert!(output.member_paths[0].join("src/lib.rs").exists());
				let lib_b =
					std::fs::read_to_string(output.member_paths[1].join("src/lib.rs")).unwrap();
				assert!(lib_b.contains("pub fn b()"));
				assert_eq!(output.non_crate_paths.len(), 1);
				assert!(output.non_crate_paths[0].exists());
			});
	}

	#[test]
	fn build_with_read_only_manifest() {
		let args = TempRustProjectArgs {
			kind: ProjectKind::Crate { files: vec![] },
			read_only_manifest: true,
			..Default::default()
		};

		ProjectTestBuilder::default()
			.with_temp_rust_project(args)
			.build()
			.execute(|context| {
				let output = context.temp_rust_project.as_ref().unwrap();
				let perms = std::fs::metadata(&output.manifest_path).unwrap().permissions();
				assert!(perms.readonly());
			});
		// on_drop restores permissions — nothing to assert post-drop since tempdir is gone.
	}

	#[test]
	fn build_with_calling_dir_override() {
		let original_dir = std::env::current_dir().unwrap();

		ProjectTestBuilder::default()
			.with_temp_rust_project(TempRustProjectArgs {
				calling_dir_override: Some(CallingDirOverride::ProjectRoot),
				..Default::default()
			})
			.build()
			.execute(|context| {
				let output = context.temp_rust_project.as_ref().unwrap();
				let cwd = std::env::current_dir().unwrap();
				assert_eq!(cwd, output.project_path);
			});

		// on_drop restores CWD.
		assert_eq!(std::env::current_dir().unwrap(), original_dir);
	}
}
