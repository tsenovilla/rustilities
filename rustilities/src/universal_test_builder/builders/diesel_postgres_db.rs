// SPDX-License-Identifier: MIT OR Apache-2.0

// Allow dead code as these builders aren't used in the rustilities crate
#![allow(dead_code)]

use crate::universal_test_builder::{
	Builder,
	builders::postgres_db::{PostgresDb, PostgresDbArgs},
};
use diesel::{Connection, PgConnection};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness};

use testcontainers_modules::{postgres::Postgres, testcontainers::Container};

pub struct DieselPostgresDbOutput {
	/// Handle of the disposable container: it is removed when this drops.
	pub container: Container<Postgres>,
	/// A connection to the container, ready to use.
	pub conn: PgConnection,
}

/// A disposable Postgres container plus a diesel connection to it. The database is
/// bare: the test brings its own DDL. Requires a running Docker daemon.
pub struct DieselPostgresDb;

impl Builder for DieselPostgresDb {
	type Args = PostgresDbArgs;
	type Output = DieselPostgresDbOutput;

	fn build(args: Self::Args) -> Self::Output {
		let container = PostgresDb::build(args);
		let conn =
			PgConnection::establish(&container.url).expect("failed to connect to the container");
		DieselPostgresDbOutput { container: container.container, conn }
	}
}

pub struct DieselMigratedPostgresDbArgs {
	/// Tag of the `postgres` Docker image.
	pub tag: String,
	/// The migrations to apply, as produced by `diesel_migrations::embed_migrations!`.
	pub migrations: EmbeddedMigrations,
}

/// A disposable Postgres container with the given embedded migrations applied: the
/// authentic schema of the project under test. Requires a running Docker daemon.
pub struct DieselMigratedPostgresDb;

impl Builder for DieselMigratedPostgresDb {
	type Args = DieselMigratedPostgresDbArgs;
	type Output = DieselPostgresDbOutput;

	fn build(args: Self::Args) -> Self::Output {
		let mut output = DieselPostgresDb::build(PostgresDbArgs { tag: args.tag });
		output
			.conn
			.run_pending_migrations(args.migrations)
			.expect("failed to run the migrations");
		output
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::universal_test_builder::universal_test_builder;
	use diesel::RunQueryDsl;

	const MIGRATIONS: EmbeddedMigrations =
		diesel_migrations::embed_migrations!("test_utils/migrations");

	#[universal_test_builder({builder = DieselPostgresDb}, {builder = DieselMigratedPostgresDb})]
	struct DieselPostgresDbTestBuilder;

	#[test]
	fn naive_db_accepts_queries() {
		DieselPostgresDbTestBuilder::default()
			.with_diesel_postgres_db(PostgresDbArgs { tag: "17".to_string() })
			.build()
			.execute(|mut context| {
				let output = context.diesel_postgres_db_mut();
				diesel::sql_query("CREATE TABLE naive_check (id SERIAL PRIMARY KEY)")
					.execute(&mut output.conn)
					.expect("the bare database must accept DDL");
			});
	}

	#[test]
	fn migrated_db_has_the_migrations_applied() {
		DieselPostgresDbTestBuilder::default()
			.with_diesel_migrated_postgres_db(DieselMigratedPostgresDbArgs {
				tag: "17".to_string(),
				migrations: MIGRATIONS,
			})
			.build()
			.execute(|mut context| {
				let output = context.diesel_migrated_postgres_db_mut();
				diesel::sql_query("INSERT INTO universal_test_builder_smoke (value) VALUES ('ok')")
					.execute(&mut output.conn)
					.expect("the migrated table must exist");
			});
	}
}
