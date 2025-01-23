pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_user_table;
mod m20250122_133902_craete_role_table;
mod m20250122_135412_craete_account_status_table;
mod m20250122_141508_create_gender_table;
mod m20250122_150141_create_tag_table;
mod m20250122_151252_create_note_table;
mod m20250122_151926_create_note_hex_color;
mod m20250122_152447_create_note_status;
mod m20250122_164036_create_note_x_tag;
mod m20250122_165404_create_user_x_tag;

// Migrator sort
// 1 table with out any relation
// 2 table with one to many, many to one
// 3 table with many to many relation

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            
            Box::new(m20250122_133902_craete_role_table::Migration),
            Box::new(m20250122_151926_create_note_hex_color::Migration),
            Box::new(m20250122_135412_craete_account_status_table::Migration),
            Box::new(m20250122_141508_create_gender_table::Migration),
            Box::new(m20250122_150141_create_tag_table::Migration),
            Box::new(m20250122_152447_create_note_status::Migration),
            
            Box::new(m20220101_000001_create_user_table::Migration),
            Box::new(m20250122_151252_create_note_table::Migration),
            
            
            Box::new(m20250122_164036_create_note_x_tag::Migration),
            Box::new(m20250122_165404_create_user_x_tag::Migration),
            
        ]
    }
}
