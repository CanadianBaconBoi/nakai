use rusqlite_migration::{Migrations, M};
use crate::MigrationTrait;

pub struct Migration;

impl MigrationTrait for Migration {
    fn migration() -> M<'static> {
        M::up(
            r#"CREATE TABLE IF NOT EXISTS `posts` (
                `id` INTEGER PRIMARY KEY AUTOINCREMENT,
                `title` TEXT NOT NULL,
                `text` TEXT NOT NULL,
            )
            "#
        ).down("DROP TABLE IF EXISTS `posts`;")
    }
}