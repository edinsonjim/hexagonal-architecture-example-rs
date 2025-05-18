use lumx_axum::axum;
use lumx_axum::plugin::WebPlugin;
use lumx_axum::router::ProgramRoutable;
use lumx_core::plugable::plugin::Plugin;
use lumx_core::program::{Program, ProgramBuilder};
use lumx_sea_orm::plugin::SeaOrmPlugin;
use lumx_sea_orm::sea_orm::DatabaseConnection;
use passport::composable::PassportPlugin;
use people::composable::PeoplePlugin;
use portal_migration::{Migrator, MigratorTrait};
use portal_service_rs::routes;
use std::env;
use warehouses::composable::warehouse::WarehousePlugin;

pub struct MigratorPlugin;

#[axum::async_trait]
impl Plugin for MigratorPlugin {
    async fn build(&self, app: &mut ProgramBuilder) {
        let conn = app.get_expect_component::<DatabaseConnection>();

        Migrator::up(conn.as_ref(), None).await.unwrap();
    }

    fn dependencies(&self) -> Vec<&str> {
        vec![std::any::type_name::<SeaOrmPlugin>()]
    }
}

pub async fn configure() -> std::sync::Arc<Program> {
    env::set_var("DATABASE_URL", "sqlite::memory:");

    let program = Program::new()
        .add_plugin(SeaOrmPlugin)
        .add_plugin(WebPlugin)
        .add_plugin(MigratorPlugin)
        .add_plugin(PeoplePlugin)
        .add_plugin(WarehousePlugin)
        .add_plugin(PassportPlugin)
        .add_router(routes())
        .build()
        .await;

    program
}
