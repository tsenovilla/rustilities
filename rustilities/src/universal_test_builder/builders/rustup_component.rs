// SPDX-License-Identifier: GPL-3.0

use crate::universal_test_builder::Builder;
use std::process::Command;

pub struct RustupComponentArgs {
	pub component: String,
	pub toolchain: Option<String>,
	/// true = install, false = remove.
	pub install: bool,
}

pub struct RustupComponentOutput {
	pub component: String,
	pub toolchain: Option<String>,
	/// Whether the component was installed *before* build ran.
	was_installed: bool,
}

pub struct RustupComponent;

impl Builder for RustupComponent {
	type Args = RustupComponentArgs;
	type Output = RustupComponentOutput;

	fn build(args: Self::Args) -> Self::Output {
		let was_installed = is_component_installed(&args.component, args.toolchain.as_deref());

		let action = if args.install { "add" } else { "remove" };
		let mut cmd = Command::new("rustup");
		cmd.arg("component").arg(action).arg(&args.component);
		if let Some(ref toolchain) = args.toolchain {
			cmd.arg("--toolchain").arg(toolchain);
		}
		cmd.output().unwrap();

		RustupComponentOutput {
			component: args.component,
			toolchain: args.toolchain,
			was_installed,
		}
	}

	fn on_drop(output: Self::Output) {
		let is_installed = is_component_installed(&output.component, output.toolchain.as_deref());

		let action = match (output.was_installed, is_installed) {
			(true, false) => Some("add"),
			(false, true) => Some("remove"),
			_ => None,
		};

		if let Some(action) = action {
			let mut cmd = Command::new("rustup");
			cmd.arg("component").arg(action).arg(&output.component);
			if let Some(ref toolchain) = output.toolchain {
				cmd.arg("--toolchain").arg(toolchain);
			}
			let _ = cmd.output();
		}
	}
}

fn is_component_installed(component: &str, toolchain: Option<&str>) -> bool {
	let mut cmd = Command::new("rustup");
	cmd.arg("component").arg("list").arg("--installed");
	if let Some(toolchain) = toolchain {
		cmd.arg("--toolchain").arg(toolchain);
	}
	let output = cmd.output().unwrap();
	let stdout = String::from_utf8_lossy(&output.stdout);
	stdout.lines().any(|line| line.starts_with(component))
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::universal_test_builder::universal_test_builder;

	#[universal_test_builder({builder = RustupComponent})]
	struct RustupTestBuilder;

	#[test]
	fn install_component_and_restore() {
		let component = "rustfmt".to_string();
		let was_installed_before = is_component_installed(&component, None);

		RustupTestBuilder::default()
			.with_rustup_component(RustupComponentArgs {
				component: component.clone(),
				toolchain: None,
				install: true,
			})
			.build()
			.execute(|context| {
				let output = context.rustup_component.as_ref().unwrap();
				assert_eq!(output.component, "rustfmt");
				assert!(is_component_installed(&component, None));
			});

		// After on_drop, state should match the original.
		assert_eq!(is_component_installed(&component, None), was_installed_before);
	}
}
