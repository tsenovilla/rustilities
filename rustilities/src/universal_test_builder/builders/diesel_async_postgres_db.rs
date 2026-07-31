// SPDX-License-Identifier: MIT OR Apache-2.0

// Allow dead code as these builders aren't used in the rustilities crate
#![allow(dead_code)]

use crate::universal_test_builder::{
	AsyncBuilder, async_trait,
	builders::{diesel_postgres_db::DieselMigratedPostgresDbArgs, postgres_db::PostgresDbArgs},
};
use diesel::{Connection, PgConnection};
use diesel_async::{
	AsyncPgConnection,
	pooled_connection::{AsyncDieselConnectionManager, bb8::Pool},
};
use diesel_migrations::MigrationHarness;
use testcontainers_modules::{
	postgres::Postgres,
	testcontainers::{ContainerAsync, ImageExt, runners::AsyncRunner},
};

pub struct DieselAsyncPostgresDbOutput {
	/// Handle of the disposable container: it is removed when this drops.
	pub container: ContainerAsync<Postgres>,
	/// A bb8 pool of async connections to the database.
	pub pool: Pool<AsyncPgConnection>,
}

/// A disposable Postgres container plus a diesel-async bb8 pool to it. The database is
/// bare: the test brings its own DDL. Requires a running Docker daemon.
pub struct DieselAsyncPostgresDb;

#[async_trait]
impl AsyncBuilder for DieselAsyncPostgresDb {
	type Args = PostgresDbArgs;
	type Output = DieselAsyncPostgresDbOutput;

	async fn async_build(args: Self::Args) -> Self::Output {
		let (container, url) = start_postgres(args.tag).await;
		let pool = build_pool(&url).await;
		DieselAsyncPostgresDbOutput { container, pool }
	}
}

/// A disposable Postgres container with the given embedded migrations applied and a
/// diesel-async bb8 pool to it: the authentic schema for tests exercising async
/// database code. Requires a running Docker daemon.
pub struct DieselAsyncMigratedPostgresDb;

#[async_trait]
impl AsyncBuilder for DieselAsyncMigratedPostgresDb {
	type Args = DieselMigratedPostgresDbArgs;
	type Output = DieselAsyncPostgresDbOutput;

	async fn async_build(args: Self::Args) -> Self::Output {
		let (container, url) = start_postgres(args.tag).await;

		// diesel_migrations' harness is sync-only: a short-lived sync connection applies
		// the migrations, then the async pool serves the test itself.
		let mut sync_conn =
			PgConnection::establish(&url).expect("failed to connect to the container");
		sync_conn
			.run_pending_migrations(args.migrations)
			.expect("failed to run the migrations");

		let pool = build_pool(&url).await;
		DieselAsyncPostgresDbOutput { container, pool }
	}
}

async fn start_postgres(tag: String) -> (ContainerAsync<Postgres>, String) {
	let container = Postgres::default()
		.with_tag(tag)
		.start()
		.await
		.expect("failed to start the postgres container");
	let port = container
		.get_host_port_ipv4(5432)
		.await
		.expect("failed to resolve the mapped port");
	let url = format!("postgres://postgres:postgres@127.0.0.1:{port}/postgres");
	(container, url)
}

async fn build_pool(url: &str) -> Pool<AsyncPgConnection> {
	let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(url);
	Pool::builder().build(manager).await.expect("failed to build the pool")
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::universal_test_builder::universal_test_builder;
	use diesel_async::RunQueryDsl;
	use diesel_migrations::EmbeddedMigrations;

	const MIGRATIONS: EmbeddedMigrations =
		diesel_migrations::embed_migrations!("test_utils/migrations");

	#[universal_test_builder(
		{builder = DieselAsyncPostgresDb, async_builder},
		{builder = DieselAsyncMigratedPostgresDb, async_builder}
	)]
	struct DieselAsyncPostgresDbTestBuilder;

	#[tokio::test]
	async fn naive_db_accepts_queries() {
		DieselAsyncPostgresDbTestBuilder::default()
			.with_diesel_async_postgres_db(PostgresDbArgs { tag: "17".to_string() })
			.async_build()
			.await
			.async_execute(|context| async move {
				let mut conn = context
					.diesel_async_postgres_db()
					.pool
					.get()
					.await
					.expect("the pool must serve a connection");
				diesel::sql_query("CREATE TABLE naive_check (id SERIAL PRIMARY KEY)")
					.execute(&mut conn)
					.await
					.expect("the bare database must accept DDL");
			})
			.await;
	}

	#[tokio::test]
	async fn migrated_db_has_the_migrations_applied() {
		DieselAsyncPostgresDbTestBuilder::default()
			.with_diesel_async_migrated_postgres_db(DieselMigratedPostgresDbArgs {
				tag: "17".to_string(),
				migrations: MIGRATIONS,
			})
			.async_build()
			.await
			.async_execute(|context| async move {
				let mut conn = context
					.diesel_async_migrated_postgres_db()
					.pool
					.get()
					.await
					.expect("the pool must serve a connection");
				diesel::sql_query("INSERT INTO universal_test_builder_smoke (value) VALUES ('ok')")
					.execute(&mut conn)
					.await
					.expect("the migrated table must exist");
			})
			.await;
	}
}
