use rusqlite_migration::M;
use crate::MigrationTrait;

pub struct Migration;

impl MigrationTrait for Migration {
    fn migration() -> M<'static> {
        M::up(r#"
            CREATE TABLE IF NOT EXISTS `files` (
                `id` INTEGER PRIMARY KEY AUTOINCREMENT,
                `name` TEXT NOT NULL,
                `date` INTEGER NOT NULL,
                `hash` TEXT NOT NULL,
                `ip` INTEGER NOT NULL,
                `size` INTEGER NOT NULL,
            );

            CREATE TABLE IF NOT EXISTS `deny_list` (
                `id` INTEGER PRIMARY KEY AUTOINCREMENT,
                `name` TEXT NOT NULL,
                `date` INTEGER NOT NULL,
                `hash` TEXT NOT NULL,
            );

            CREATE TABLE IF NOT EXISTS `rate_limit` (
                `id` INTEGER PRIMARY KEY AUTOINCREMENT,
                `ip_hash` TEXT NOT NULL,
                `files` INTEGER NOT NULL,
                `date` INTEGER NOT NULL,
            );
        "#
        ).down(r#"
            DROP TABLE IF EXISTS `files`;
            DROP TABLE IF EXISTS `deny_list`;
            DROP TABLE IF EXISTS `rate_limit`;
        "#)
    }
}