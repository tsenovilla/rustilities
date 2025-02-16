// SPDX-License-Identifier: GPL-3.0

use thiserror::Error;

/// Represents the various errors that can occur in the crate.
#[derive(Error, Debug)]
pub enum Error {
	#[error("IO error: {0}")]
	IO(#[from] std::io::Error),
	#[error("{0}")]
	Descriptive(String),
	#[cfg(any(feature = "manifest", feature = "full"))]
	#[cfg_attr(docsrs, doc(cfg(any(feature = "manifest", feature = "full"))))]
	#[error("toml_edit error: {0}")]
	TomlEdit(#[from] toml_edit::TomlError),
}
