// SPDX-License-Identifier: GPL-3.0

#![cfg_attr(docsrs, feature(doc_cfg))]

mod error;

#[cfg(any(feature = "paths", feature = "full"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "paths", feature = "full"))))]
pub mod paths;

#[cfg(any(feature = "fmt", feature = "full"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "fmt", feature = "full"))))]
pub mod fmt;

pub use error::Error;
