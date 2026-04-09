//! Migration binary entry point for orqa-storage-migration.
//
// Connects to the orqa.db database at the path specified by ORQA_DB_PATH
// (or .state/orqa.db relative to the current directory), runs the bridge for
// any existing legacy _migrations data, then applies all pending SeaORM
// migrations via Migrator::up.
//
// Usage:
//   ORQA_DB_PATH=/path/to/orqa.db cargo run
//   cargo run  # defaults to .state/orqa.db

use orqa_storage_migration::{bridge_legacy_migrations, Migrator};
use sea_orm::Database;
use sea_orm_migration::MigratorTrait;

#[tokio::main]
async fn main() {
    // Determine database path from environment or use default.
    let db_path = std::env::var("ORQA_DB_PATH")
        .unwrap_or_else(|_| ".state/orqa.db".to_owned());

    let db_url = format!("sqlite://{db_path}?mode=rwc");

    println!("Connecting to database: {db_path}");

    let db = Database::connect(&db_url)
        .await
        .unwrap_or_else(|e| panic!("failed to connect to database at {db_path}: {e}"));

    println!("Running bridge for legacy _migrations table...");
    bridge_legacy_migrations(&db)
        .await
        .unwrap_or_else(|e| panic!("bridge failed: {e}"));

    println!("Applying pending migrations...");
    Migrator::up(&db, None)
        .await
        .unwrap_or_else(|e| panic!("migration failed: {e}"));

    println!("Migrations complete.");
}
