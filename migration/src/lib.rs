use rusqlite_migration::M;

mod m20220120_000001_create_post_table;
mod m20241111_235503_hostess;

pub struct Migrator;

impl Migrator {
    pub fn migrations() -> Vec<M<'static>> {
        vec![
            m20220120_000001_create_post_table::Migration::migration(),
            m20241111_235503_hostess::Migration::migration(),
        ]
    }
}

trait MigrationTrait {
    fn migration() -> M<'static>;
}