// SPDX-License-Identifier: GPL-3.0

//! # Description
//!
//! This crate offers several functionalities (not necessarily related to each other).
//! Please refer to the specific documentation for each part of the crate to learn more about it.
//!
//! # Features
//!
//! The crate splits is functionalities into several features, allowing to compile only the
//! parts that are needed.

#![cfg_attr(docsrs, feature(doc_cfg))]

mod error;

#[cfg(feature = "paths")]
#[cfg_attr(docsrs, doc(cfg(feature = "paths")))]
pub mod paths;

#[cfg(feature = "fmt")]
#[cfg_attr(docsrs, doc(cfg(feature = "fmt")))]
pub mod fmt;

#[cfg(feature = "manifest")]
#[cfg_attr(docsrs, doc(cfg(feature = "manifest")))]
pub mod manifest;

#[cfg(feature = "parsing")]
#[cfg_attr(docsrs, doc(cfg(feature = "parsing")))]
pub mod parsing;

pub use error::Error;
