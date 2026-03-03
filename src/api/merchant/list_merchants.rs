use crate::api::error::ApiError;
use application::merchant::dto::MerchantResponse;
use application::merchant::list_merchants::ListMerchants;
use application::pagination::{PaginatedResponse, PaginationQuery};
use axum::extract::{Query, State};
use axum::Json;
use crate::api::auth::extractor::AuthUser;

#[utoipa::path(get, tag="merchant", path = "/merchants", security(("bearerAuth" = [])), params(("page" = i64, Query),("page_size" = i64, Query)), responses((status = OK, body = PaginatedResponse<MerchantResponse>)))]
pub async fn list_merchants_handler(AuthUser(user): AuthUser, Query(query): Query<PaginationQuery>,
                                    State(use_case): State<ListMerchants>
                                  )
                                  -> Result<Json<PaginatedResponse<MerchantResponse>>, ApiError> {
    println!("{:?}", user);
    let res: Result<Json<PaginatedResponse<MerchantResponse>>, ApiError> = use_case
        .execute(query.page, query.page_size).await
        .map(Json)
        .map_err(|err| err.into());
    res
}