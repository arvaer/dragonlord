// region:    --- Modules

mod error;
mod store;
pub mod task; // <- this is going to be our task model controller

pub use self::error::{Error, Result};
use self::store::{new_db_pool, Db};


// endregion: --- Modules

#[derive(Clone)]
pub struct ModelManager {
    db: Db
}

impl ModelManager {
    pub async fn new() -> Result<Self> {
        let db = new_db_pool().await?;
        Ok(ModelManager {
            db
        })
    }

    // Returns the Db pool by reference to the underlying model layer.
    // The rest of the code base can access the model layer, and the model layer access's the db
    // pool
    pub(in crate::model) fn db(&self) -> &Db {
        &self.db
    }
}
