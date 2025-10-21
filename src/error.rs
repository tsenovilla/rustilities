// SPDX-License-Identifier: GPL-3.0

use thiserror::Error;

/// Represents the various errors that can occur in the crate.
#[derive(Error, Debug)]
pub enum Error {
	#[error("{0}")]
	Descriptive(String),
	#[error("IO error: {0}")]
	IO(#[from] std::io::Error),
	#[cfg(feature = "manifest")]
	#[cfg_attr(docsrs, doc(cfg(feature = "manifest")))]
	#[error("StripPrefixError")]
	StripPrefixError(#[from] std::path::StripPrefixError),
	#[cfg(feature = "manifest")]
	#[cfg_attr(docsrs, doc(cfg(feature = "manifest")))]
	#[error("toml_edit error: {0}")]
	TomlEdit(#[from] toml_edit::TomlError),
}
