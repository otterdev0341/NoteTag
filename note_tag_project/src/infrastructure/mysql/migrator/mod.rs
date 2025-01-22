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

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            // none relation table need to run 1st phase
            
            Box::new(m20250122_133902_craete_role_table::Migration),
            Box::new(m20250122_135412_craete_account_status_table::Migration),
            Box::new(m20250122_141508_create_gender_table::Migration),
            Box::new(m20250122_150141_create_tag_table::Migration),
            Box::new(m20250122_151926_create_note_hex_color::Migration),
            Box::new(m20250122_152447_create_note_status::Migration),
            // main table with relation need to run on 2nd phase
            Box::new(m20220101_000001_create_user_table::Migration),
            Box::new(m20250122_151252_create_note_table::Migration),
            // union table need to run on 3rd phase
            Box::new(m20250122_164036_create_note_x_tag::Migration),
            Box::new(m20250122_165404_create_user_x_tag::Migration),
            

        ]
    }
}
