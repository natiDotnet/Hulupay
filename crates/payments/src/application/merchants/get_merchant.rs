use crate::application::cache_service::CacheService;
use crate::{cache_get, cache_set};
use merchant::{domain, Merchant};
use sea_orm::DatabaseConnection;

pub async fn get_merchant(
    db: &DatabaseConnection,
    cache: &dyn CacheService,
    name: &str,
) -> Option<Merchant> {
    let cache_key = format!("merchant:{name}");
    if let Ok(Some(cached)) = cache_get!(cache, Merchant, &cache_key).await {
        return Some(cached);
    }
    let merchant = domain::merchant::Entity::find_by_name(name.to_owned())
        .one(db)
        .await
        .ok()?
        .map(Into::into);

    if let Some(ref merchant) = merchant {
        let _ = cache_set!(cache, &cache_key, merchant, 3600).await;
    }

    merchant
}
