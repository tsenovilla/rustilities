// SPDX-License-Identifier: GPL-3.0

#[cfg(test)]
mod tests;

use crate::Error;
use std::{path::Path, process::Command};

const expect_msg: &str = "If cargo fmt were to fail with an IO error, it would have already failed with 'cargo +nightly fmt --all'; qed;";

/// Given a path, this function firstly tries to:
/// - Apply `cargo +nightly fmt --all` to it.
/// - In case of failure, it tries to apply `cargo fmt --all` to it.
/// - Otherwise it returns an error explaining why the command failed.
/// ## Errors:
/// - If neither `cargo +nightly fmt --all` nor `cargo fmt --all` successfully can be successfully
///   applied to the path.
pub fn format_dir<P: AsRef<Path>>(path: P) -> Result<(), Error> {
	Command::new("cargo")
		.arg("+nightly")
		.arg("fmt")
		.arg("--all")
		.current_dir(path.as_ref())
		.output()
		.and_then(|output| {
			if output.status.success() {
				Ok(output)
			} else {
				Ok(Command::new("cargo")
					.arg("fmt")
					.arg("--all")
					.current_dir(path.as_ref())
					.output()
					.expect(expect_msg))
			}
		})
		.map_or_else(
			|err| err.into(),
			|output| {
				if output.status.success() {
					Ok(())
				} else {
					Err(Error::Descriptive(
						String::from_utf8_lossy(&output_fallback.stderr).into_owned(),
					))
				}
			},
		)
}
