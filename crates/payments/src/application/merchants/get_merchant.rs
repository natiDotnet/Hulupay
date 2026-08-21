use crate::application::cache_service::CacheService;
use crate::{cache_get, cache_set};
use merchant::Merchant;
use merchant::domain::merchant::Merchant as MerchantModel;
use toasty::Db;
use tracing::debug;
use uuid::Uuid;

pub async fn get_merchant(db: &Db, cache: &dyn CacheService, name: Uuid) -> Option<Merchant> {
    let cache_key = format!("merchant:{name}");
    if let Ok(Some(cached)) = cache_get!(cache, Merchant, &cache_key).await {
        return Some(cached);
    }
    debug!(?name, "Getting merchant from database");
    let mut db_mut = db.clone();
    let merchant = MerchantModel::filter_by_id(name)
        // filter(MerchantModel::fields().name().eq(name.to_owned()))
        .first()
        .exec(&mut db_mut)
        .await
        .ok()?
        .map(Into::into);

    if let Some(ref merchant) = merchant {
        let _ = cache_set!(cache, &cache_key, merchant, 3600).await;
    }

    merchant
}
