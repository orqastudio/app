// SeaORM entity for the `settings` table.
//
// Settings are key-value pairs keyed by (key, scope). The composite primary key
// is modelled using PrimaryKeyTrait. Values are stored as JSON strings; the
// repository layer handles deserialization.

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

/// SeaORM entity model for a row in the `settings` table.
///
/// The table uses a composite primary key (key, scope). SeaORM represents this
/// with a tuple in the `PrimaryKey` associated type below.
#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "settings")]
pub struct Model {
    /// Setting name key (part of composite PK).
    #[sea_orm(primary_key, auto_increment = false)]
    pub key: String,
    /// JSON-encoded setting value.
    pub value: String,
    /// Scope qualifier (e.g. "app", "project:123") (part of composite PK).
    #[sea_orm(primary_key, auto_increment = false)]
    pub scope: String,
    /// ISO-8601 last-updated timestamp.
    pub updated_at: String,
}

/// Relations from `settings` to other tables.
///
/// Settings have no foreign-key relations; they are a flat key-value store.
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
