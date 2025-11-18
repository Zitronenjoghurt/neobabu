pub use sea_orm_migration::prelude::*;

mod m20251113_223834_initial_core;
mod m20251114_202622_initial_birthday;
mod m20251118_211931_initial_games_rps;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20251113_223834_initial_core::Migration),
            Box::new(m20251114_202622_initial_birthday::Migration),
            Box::new(m20251118_211931_initial_games_rps::Migration),
        ]
    }
}
