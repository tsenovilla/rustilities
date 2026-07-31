// SPDX-License-Identifier: MIT OR Apache-2.0

// Allow dead code as this builder isn't used in the rustilities crate
#![allow(dead_code)]

use crate::universal_test_builder::Builder;
use testcontainers_modules::{
	postgres::Postgres,
	testcontainers::{Container, ImageExt, runners::SyncRunner},
};

pub struct PostgresDbArgs {
	/// Tag of the `postgres` Docker image.
	pub tag: String,
}

pub struct PostgresDbOutput {
	/// Handle of the disposable container: it is removed when this drops.
	pub container: Container<Postgres>,
	/// Connection URL reaching the container from the host.
	pub url: String,
}

/// Spins up a disposable Postgres container (default credentials `postgres`/`postgres`,
/// database `postgres`) and hands back its connection URL. Client-agnostic: pair it
/// with any Postgres client. Requires a running Docker daemon.
pub struct PostgresDb;

impl Builder for PostgresDb {
	type Args = PostgresDbArgs;
	type Output = PostgresDbOutput;

	fn build(args: Self::Args) -> Self::Output {
		let container = Postgres::default()
			.with_tag(args.tag)
			.start()
			.expect("failed to start the postgres container");
		let port = container.get_host_port_ipv4(5432).expect("failed to resolve the mapped port");
		let url = format!("postgres://postgres:postgres@127.0.0.1:{port}/postgres");
		PostgresDbOutput { container, url }
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::universal_test_builder::universal_test_builder;

	#[universal_test_builder({builder = PostgresDb})]
	struct PostgresDbTestBuilder;

	#[test]
	fn build_starts_a_reachable_container() {
		PostgresDbTestBuilder::default()
			.with_postgres_db(PostgresDbArgs { tag: "17".to_string() })
			.build()
			.execute(|context| {
				let output = context.postgres_db();
				assert!(output.url.starts_with("postgres://postgres:postgres@127.0.0.1:"));
				assert!(output.url.ends_with("/postgres"));
			});
	}
}
