use sea_orm_migration::prelude::*;

use portal_migration::Migrator;

#[async_std::main]
async fn main() {
    cli::run_cli(Migrator).await;
}
