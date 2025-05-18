use crate::adapters::rest::types::product::CreateProductParams;
use crate::domain::commands::product::CreateProductCommand;
use crate::domain::ports::api::product::CreateProductUseCase;
use crate::domain::services::product::ProductService;
use ids_std_rest_api::replier::Replier;
use ids_std_rest_api::types::created::Created;
use ids_std_rest_api::types::result::ApiResult;
use lumx_axum::axum::Json;
use lumx_axum::extractor::Component;

pub async fn create_product(
    Component(uc): Component<ProductService>,
    Json(payload): Json<CreateProductParams>,
) -> ApiResult<Created<i32>> {
    tracing::info!("creating product {:?}", payload);

    let create_product_cmd = CreateProductCommand {
        name: payload.name,
        summary: payload.summary,
        family_id: payload.family_id,
        purchasable: payload.purchasable,
        saleable: payload.saleable,
        saleable_without_stock: payload.saleable_without_stock,
    };
    let product_id = uc.create_product(&create_product_cmd).await?;

    Ok(Replier::ok(Created::new(product_id)))
}
