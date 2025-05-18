use lumx_axum::axum::Router;

pub fn routes() -> Router {
    Router::new()
        .merge(people::adapters::rest::routers::person::router())
        .merge(people::adapters::rest::routers::person_gender::router())
        .merge(warehouses::adapters::rest::routers::product::router())
        .merge(passport::adapters::rest::routers::auth::router())
}
