use crate::adapters::repository::family::ProductFamilySeaRepository;
use crate::adapters::repository::product::ProductSeaRepository;
use crate::domain::services::product::ProductService;
use lumx_core::plugable::plugin::Plugin;
use lumx_core::program::ProgramBuilder;
use lumx_sea_orm::plugin::SeaOrmPlugin;
use lumx_sea_orm::sea_orm::DatabaseConnection;

pub struct WarehousePlugin;

impl WarehousePlugin {
    fn expose_repos(&self, app: &mut ProgramBuilder) {
        let db_conn = app.get_expect_component::<DatabaseConnection>();

        let product_repo = ProductSeaRepository::new(&db_conn);
        let family_repo = ProductFamilySeaRepository::new(&db_conn);

        app.add_component(product_repo);
        app.add_component(family_repo);
    }

    fn expose_services(&self, app: &mut ProgramBuilder) {
        let product_repo = app.get_expect_component::<ProductSeaRepository>();
        let family_repo = app.get_expect_component::<ProductFamilySeaRepository>();

        let product_service = ProductService::new(product_repo, family_repo);

        app.add_component(product_service);
    }
}

#[async_trait::async_trait]
impl Plugin for WarehousePlugin {
    async fn build(&self, app: &mut ProgramBuilder) {
        self.expose_repos(app);
        self.expose_services(app);
    }

    fn dependencies(&self) -> Vec<&str> {
        vec![std::any::type_name::<SeaOrmPlugin>()]
    }
}
