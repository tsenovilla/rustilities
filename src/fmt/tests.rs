// SPDX-License-Identifier: GPL-3.0

use super::*;
use std::{fs::File, io::ErrorKind, path::PathBuf};
use tempfile::TempDir;

struct TestBuilder {
	tempdir: TempDir,
	with_invalid_code: bool,
	with_nightly_component: bool,
	fmt_code_path: PathBuf,
	not_fmt_code_path: PathBuf,
}

impl Default for TestBuilder {
	fn default() -> Self {
		let tempdir = tempfile::tempdir().expect("The tempdir should be created; qed;");
		let manifest_path = tempdir.path().join("Cargo.toml");
		let src_path = tempdir.path().join("src");
		let lib_path = src_path.join("lib.rs");
		let fmt_code_path = src_path.join("fmt_code_path.rs");
		let not_fmt_code_path = src_path.join("not_fmt_code_path.rs");

		File::create(&manifest_path).expect("The file should be created; qed;");
		std::fs::create_dir(&src_path).expect("The directory should be created");
		File::create(&lib_path).expect("The file should be created; qed;");
		File::create(&fmt_code_path).expect("The file should be created; qed;");
		File::create(&not_fmt_code_path).expect("The file should be created; qed;");

		// Write the manifest so the tempdir becomes a Rust directory
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
		.expect("The path should be writable; qed;");

		// Create a lib containing both modules, so they will be formatted
		std::fs::write(&lib_path, "mod fmt_code_path; mod not_fmt_code_path;")
			.expect("The file should be writable; qed;");

		// Add the fmt code
		std::fs::write(
			&fmt_code_path,
			r#"
pub enum A {
	A,
	B,
	C,
}"#,
		)
		.expect("The file should be writable; qed;");

		Self {
			tempdir,
			with_invalid_code: false,
			with_nightly_component: false,
			fmt_code_path,
			not_fmt_code_path,
		}
	}
}

impl TestBuilder {
	fn with_invalid_code(mut self) -> Self {
		self.with_invalid_code = true;
		self
	}

	fn with_nightly_component(mut self) -> Self {
		self.with_nightly_component = true;
		self
	}

	fn build(self) -> Self {
		// Build the non fmt code to be valid rust depending on self.with_invalid_code
		if self.with_invalid_code {
			std::fs::write(&self.not_fmt_code_path, "pub enum A {A,B,C};")
				.expect("The file should be writable; qed;");
		} else {
			std::fs::write(&self.not_fmt_code_path, "pub enum A {A,B,C}")
				.expect("The file should be writable; qed;");
		}

		// Install the nightly fmt component if needed, or remove it if not needed
		if self.with_nightly_component {
			// Add the nightly fmt if possible
			let _ = Command::new("rustup")
				.arg("component")
				.arg("add")
				.arg("rustfmt")
				.arg("--toolchain")
				.arg("nightly")
				.output();
		} else {
			// Remove the nightly fmt if possible
			let _ = Command::new("rustup")
				.arg("component")
				.arg("remove")
				.arg("rustfmt")
				.arg("--toolchain")
				.arg("nightly")
				.output();
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
fn format_dir_works_if_nightly_available() {
	TestBuilder::default().with_nightly_component().build().execute(|builder| {
		assert!(format_dir(builder.tempdir.path()).is_ok());
		assert_eq!(
			std::fs::read_to_string(&builder.fmt_code_path)
				.expect("The file should be readable; qed;"),
			std::fs::read_to_string(&builder.not_fmt_code_path)
				.expect("The file should be readable; qed;")
		)
	});
}

#[test]
fn format_dir_also_if_nightly_fmt_fails() {
	TestBuilder::default().build().execute(|builder| {
		assert!(format_dir(builder.tempdir.path()).is_ok());
		assert_eq!(
			std::fs::read_to_string(&builder.fmt_code_path)
				.expect("The file should be readable; qed;"),
			std::fs::read_to_string(&builder.not_fmt_code_path)
				.expect("The file should be readable; qed;")
		)
	});
}

#[test]
fn format_dir_fails_if_the_dir_cannot_be_formatted() {
	TestBuilder::default().with_invalid_code().build().execute(|builder| {
		match format_dir(builder.tempdir.path()) {
			Err(Error::Descriptive(msg)) => {
				// The msg contains the file where the issue is
				assert!(msg.contains(&format!("{}", builder.not_fmt_code_path.display())));
			},
			_ => panic!("Unexpected error"),
		}
	});
}

#[test]
fn format_dir_fails_if_the_path_isnt_a_rust_dir() {
	let tempdir = tempfile::tempdir().expect("tempdir should be created; qed;");
	let dir = tempdir.path().join("dir");
	std::fs::create_dir(&dir).expect("The dir should be created; qed;");
	match format_dir(&dir) {
		Err(Error::Descriptive(msg)) => {
			assert!(msg.contains("error: could not find `Cargo.toml`"));
		},
		_ => panic!("Unexpected error"),
	}
}

#[test]
fn format_dir_fails_with_nightly_available_if_io_error() {
	TestBuilder::default().with_nightly_component().build().execute(|builder| {
		match format_dir(builder.tempdir.path().join("dir")) {
			Err(Error::IO(err)) => {
				assert_eq!(err.kind(), ErrorKind::NotFound);
			},
			_ => panic!("Unexpected error"),
		}
	});
}

#[test]
fn format_dir_fails_without_nightly_available_if_io_error() {
	TestBuilder::default().build().execute(|builder| {
		match format_dir(builder.tempdir.path().join("dir")) {
			Err(Error::IO(err)) => {
				assert_eq!(err.kind(), ErrorKind::NotFound);
			},
			_ => panic!("Unexpected error"),
		}
	});
}
