use crate::adapters::repository::user::UserSeaRepository;
use jsonwebtoken::{DecodingKey, EncodingKey};
use lazy_static::lazy_static;
use lumx_core::{plugable::plugin::Plugin, program::ProgramBuilder};
use lumx_sea_orm::plugin::SeaOrmPlugin;
use lumx_sea_orm::sea_orm::DatabaseConnection;
use passport_auth::auth::UserAuthenticator;
use passport_jwt::auth::TokenAuthManager;
use passport_jwt::decoder::AccessTokenDecoder;
use passport_jwt::encoder::UserEncoder;
use std::env;

lazy_static! {
    static ref ENCODE_KEY: EncodingKey =
        EncodingKey::from_rsa_pem(include_bytes!("./../../../.certs/private.pem"))
            .expect("private key parse failed");
    static ref DECODING_KEY: DecodingKey =
        DecodingKey::from_rsa_pem(include_bytes!("./../../../.certs/public.pem"))
            .expect("public key parse failed");
}

pub struct PassportPlugin;

#[async_trait::async_trait]
impl Plugin for PassportPlugin {
    async fn build(&self, app: &mut ProgramBuilder) {
        self.expose_repos(app);
        self.expose_components(app);
        self.expose_managers(app);
    }

    fn dependencies(&self) -> Vec<&str> {
        vec![std::any::type_name::<SeaOrmPlugin>()]
    }
}

impl PassportPlugin {
    fn expose_repos(&self, app: &mut ProgramBuilder) {
        let db = app.get_expect_component::<DatabaseConnection>();

        let user_repo = UserSeaRepository::new(&db);

        app.add_component(user_repo);
    }

    fn expose_managers(&self, app: &mut ProgramBuilder) {
        let authenticator = app.get_expect_component::<UserAuthenticator>();
        let encoder = app.get_expect_component::<UserEncoder>();
        let auth_manager = TokenAuthManager::new(encoder, authenticator);

        app.add_component(auth_manager);
    }

    fn expose_components(&self, app: &mut ProgramBuilder) {
        let user_repo = app.get_expect_component::<UserSeaRepository>();
        let token_issuer =
            env::var("ACCESS_TOKEN_ISSUER").expect("ACCESS_TOKEN_ISSUER is not set in env");
        let token_audience =
            env::var("ACCESS_TOKEN_AUDIENCE").expect("ACCESS_TOKEN_AUDIENCE is not set in env");

        let decoder = AccessTokenDecoder::new(
            DECODING_KEY.to_owned(),
            token_issuer.to_owned(),
            token_audience.to_owned(),
        );
        let encoder = UserEncoder::new(
            ENCODE_KEY.to_owned(),
            token_issuer.to_owned(),
            token_audience.to_owned(),
        );

        let authenticator = UserAuthenticator::new(user_repo);

        app.add_component(authenticator);
        app.add_component(encoder);
        app.add_component(decoder);
    }
}
