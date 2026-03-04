use crate::api::error::ApiError;
use application::merchant::dto::MerchantResponse;
use application::merchant::list_merchants::ListMerchants;
use application::pagination::{PaginatedResponse, PaginationQuery};
use axum::extract::{Query, State};
use axum::Json;
use tracing::debug;
use crate::api::auth::extractor::AuthUser;

#[utoipa::path(get, tag="merchant",
    path = "/merchants",
    security(("bearer_auth" = [])),
    params(PaginationQuery),
    responses((status = OK, body = PaginatedResponse<MerchantResponse>)))]
pub async fn list_merchants_handler(
    AuthUser(user): AuthUser,
    Query(query): Query<PaginationQuery>,
    State(use_case): State<ListMerchants>,)
    -> Result<Json<PaginatedResponse<MerchantResponse>>, ApiError> {
    debug!("{:?}", user);
    use_case
        .execute(query.page, query.page_size).await
        .map(Json)
        .map_err(|err| err.into())
}