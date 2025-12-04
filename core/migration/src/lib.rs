pub use sea_orm_migration::prelude::*;

mod m20251113_223834_initial_core;
mod m20251114_202622_initial_birthday;
mod m20251118_211931_initial_games_rps;
mod m20251120_144534_initial_apod;
mod m20251122_111720_initial_dashboard;
mod m20251125_203557_initial_youtube;
mod m20251204_132745_initial_inventory;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20251113_223834_initial_core::Migration),
            Box::new(m20251114_202622_initial_birthday::Migration),
            Box::new(m20251118_211931_initial_games_rps::Migration),
            Box::new(m20251120_144534_initial_apod::Migration),
            Box::new(m20251122_111720_initial_dashboard::Migration),
            Box::new(m20251125_203557_initial_youtube::Migration),
            Box::new(m20251204_132745_initial_inventory::Migration),
        ]
    }
}
