use crate::domain::merchant::Merchant;
use toasty::Db;
use uuid::Uuid;

#[derive(Clone)]
pub struct DeleteMerchant {
    db: Db,
}

impl DeleteMerchant {
    pub fn new(db: Db) -> Self {
        Self { db }
    }

    pub async fn execute(&self, id: Uuid) -> anyhow::Result<()> {
        let mut db = self.db.clone();

        // Verify existence first — delete_by_id in Toasty may error on not-found
        match Merchant::filter_by_id(id).first().exec(&mut db).await? {
            Some(_) => {}
            None => return Err(anyhow::anyhow!("Merchant not found")),
        };

        Merchant::filter_by_id(id).delete().exec(&mut db).await?;

        Ok(())
    }
}
