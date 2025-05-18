use crate::{
    adapters::repository::{
        person::PersonSeaRepository, person_document_type::PersonDocumentTypeSeaRepository,
        person_gender::PersonGenderSeaRepository,
    },
    domain::services::{person::PersonService, person_gender::PersonGenderService},
};
use async_trait::async_trait;
use lumx_core::{plugable::plugin::Plugin, program::ProgramBuilder};
use lumx_sea_orm::plugin::SeaOrmPlugin;
use lumx_sea_orm::sea_orm::DatabaseConnection;

pub struct PeoplePlugin;

impl PeoplePlugin {
    fn expose_repos(&self, app: &mut ProgramBuilder) {
        let db = app.get_expect_component::<DatabaseConnection>();

        let person_repo = PersonSeaRepository::new(&db);
        let person_document_type_repo = PersonDocumentTypeSeaRepository::new(&db);
        let person_gender_repo = PersonGenderSeaRepository::new(&db);

        app.add_component(person_repo);
        app.add_component(person_document_type_repo);
        app.add_component(person_gender_repo);
    }

    fn expose_services(&self, app: &mut ProgramBuilder) {
        let person_repo = app.get_expect_component::<PersonSeaRepository>();
        let person_document_type_repo =
            app.get_expect_component::<PersonDocumentTypeSeaRepository>();
        let person_gender_repo = app.get_expect_component::<PersonGenderSeaRepository>();

        let person_service = PersonService::new(
            person_repo,
            person_document_type_repo,
            person_gender_repo.clone(),
        );

        let person_gender_service = PersonGenderService::new(person_gender_repo.clone());

        app.add_component(person_service);
        app.add_component(person_gender_service);
    }
}

#[async_trait]
impl Plugin for PeoplePlugin {
    async fn build(&self, app: &mut ProgramBuilder) {
        self.expose_repos(app);
        self.expose_services(app);
    }

    fn dependencies(&self) -> Vec<&str> {
        vec![std::any::type_name::<SeaOrmPlugin>()]
    }
}
