// SPDX-License-Identifier: GPL-3.0

use super::*;
use crate::universal_test_builder::{
	UniversalTestBuilder,
	builders::{
		ProjectKind, RustupComponentArgs, TempRustProjectArgs,
	},
};
use std::{io::ErrorKind, path::PathBuf};

const FMT_CODE: &str = r#"
pub enum A {
	A,
	B,
	C,
}"#;

fn make_crate_args(invalid_code: bool) -> TempRustProjectArgs {
	let not_fmt_code =
		if invalid_code { "pub enum A {A,B,C};" } else { "pub enum A {A,B,C}" };
	TempRustProjectArgs {
		kind: ProjectKind::Crate {
			files: vec![
				(
					PathBuf::from("src/lib.rs"),
					"mod fmt_code_path; mod not_fmt_code_path;".to_string(),
				),
				(PathBuf::from("src/fmt_code_path.rs"), FMT_CODE.to_string()),
				(PathBuf::from("src/not_fmt_code_path.rs"), not_fmt_code.to_string()),
			],
		},
		..Default::default()
	}
}

fn nightly_rustfmt_args(install: bool) -> RustupComponentArgs {
	RustupComponentArgs {
		component: "rustfmt".to_string(),
		toolchain: Some("nightly".to_string()),
		install,
	}
}

#[test]
fn format_dir_works_if_nightly_available() {
	UniversalTestBuilder::default()
		.with_temp_rust_project(make_crate_args(false))
		.with_rustup_component(nightly_rustfmt_args(true))
		.build()
		.execute(|context| {
			let project = context.temp_rust_project.as_ref().unwrap();
			assert!(format_dir(&project.project_path).is_ok());
			assert_eq!(
				std::fs::read_to_string(project.project_path.join("src/fmt_code_path.rs"))
					.expect("The file should be readable; qed;"),
				std::fs::read_to_string(project.project_path.join("src/not_fmt_code_path.rs"))
					.expect("The file should be readable; qed;")
			)
		});
}

#[test]
fn format_dir_works_if_nightly_fmt_not_available() {
	UniversalTestBuilder::default()
		.with_temp_rust_project(make_crate_args(false))
		.with_rustup_component(nightly_rustfmt_args(false))
		.build()
		.execute(|context| {
			let project = context.temp_rust_project.as_ref().unwrap();
			assert!(format_dir(&project.project_path).is_ok());
			assert_eq!(
				std::fs::read_to_string(project.project_path.join("src/fmt_code_path.rs"))
					.expect("The file should be readable; qed;"),
				std::fs::read_to_string(project.project_path.join("src/not_fmt_code_path.rs"))
					.expect("The file should be readable; qed;")
			)
		});
}

#[test]
fn format_dir_fails_if_the_dir_cannot_be_formatted() {
	UniversalTestBuilder::default()
		.with_temp_rust_project(make_crate_args(true))
		.with_rustup_component(nightly_rustfmt_args(false))
		.build()
		.execute(|context| {
			let project = context.temp_rust_project.as_ref().unwrap();
			let not_fmt_code_path = project.project_path.join("src/not_fmt_code_path.rs");
			match format_dir(&project.project_path) {
				Err(Error::Descriptive(msg)) => {
					assert!(msg.contains(&format!("{}", not_fmt_code_path.display())));
				},
				_ => panic!("Unexpected error"),
			}
		});
}

#[test]
fn format_dir_fails_with_nightly_available_if_io_error() {
	UniversalTestBuilder::default()
		.with_temp_rust_project(make_crate_args(false))
		.with_rustup_component(nightly_rustfmt_args(true))
		.build()
		.execute(|context| {
			let project = context.temp_rust_project.as_ref().unwrap();
			match format_dir(project.project_path.join("dir")) {
				Err(Error::IO(err)) => {
					assert_eq!(err.kind(), ErrorKind::NotFound);
				},
				_ => panic!("Unexpected error"),
			}
		});
}

#[test]
fn format_dir_fails_without_nightly_available_if_io_error() {
	UniversalTestBuilder::default()
		.with_temp_rust_project(make_crate_args(false))
		.with_rustup_component(nightly_rustfmt_args(false))
		.build()
		.execute(|context| {
			let project = context.temp_rust_project.as_ref().unwrap();
			match format_dir(project.project_path.join("dir")) {
				Err(Error::IO(err)) => {
					assert_eq!(err.kind(), ErrorKind::NotFound);
				},
				_ => panic!("Unexpected error"),
			}
		});
}
