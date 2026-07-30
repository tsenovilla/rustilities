// SPDX-License-Identifier: GPL-3.0

use crate::universal_test_builder::Builder;

pub struct TempDir;
impl Builder for TempDir {
	type Args = ();
	type Output = tempfile::TempDir;

	fn build(_args: Self::Args) -> Self::Output {
		tempfile::tempdir().unwrap()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::universal_test_builder::universal_test_builder;

	#[universal_test_builder({builder = TempDir})]
	struct TempDirTestBuilder;

	#[test]
	fn build_creates_temp_dir() {
		TempDirTestBuilder::default().with_temp_dir(()).build().execute(|context| {
			let dir = context.temp_dir.as_ref().unwrap();
			assert!(dir.path().exists());
			assert!(dir.path().is_dir());
		});
	}
}
