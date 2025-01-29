use rusqlite::Connection;
use rusqlite_migration::Migrations;
use migration::Migrator;

#[async_std::main]
async fn main(conn: &mut Connection) {
    let migrations = Migrations::new(Migrator::migrations());
    migrations.to_latest(conn).unwrap();
}
