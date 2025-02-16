// SPDX-License-Identifier: GPL-3.0

//! # Description
//!
//! This crate offers several functionalities (not necessarily related to each other). 
//! Please refer to the specific documentation for each part of the crate to learn more about it.
//!
//! # Features
//!
//! The crate splits is functionalities into several features, allowing to compile only the
//! parts that are needed. The `full` feature compiles the entire crate.

#![cfg_attr(docsrs, feature(doc_cfg))]

mod error;

#[cfg(any(feature = "paths", feature = "full"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "paths", feature = "full"))))]
pub mod paths;

#[cfg(any(feature = "fmt", feature = "full"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "fmt", feature = "full"))))]
pub mod fmt;

#[cfg(any(feature = "manifest", feature = "full"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "manifest", feature = "full"))))]
pub mod manifest;

pub use error::Error;
