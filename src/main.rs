use lumx_axum::plugin::WebPlugin;
use lumx_axum::router::ProgramRoutable;
use lumx_core::tokio;
use lumx_sea_orm::plugin::SeaOrmPlugin;
use portal_service_rs::routes;

#[tokio::main]
async fn main() {
    lumx_core::program::Program::new()
        .with_envs()
        .collect_tracing()
        .add_plugin(SeaOrmPlugin)
        .add_plugin(WebPlugin)
        .add_plugin(people::composable::PeoplePlugin)
        .add_plugin(warehouses::composable::warehouse::WarehousePlugin)
        .add_plugin(passport::composable::PassportPlugin)
        .add_router(routes())
        .run()
        .await
}
