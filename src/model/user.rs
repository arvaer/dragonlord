use crate::ctx::Ctx;
use crate::model::base::{self, DbBMC};
use crate::model::ModelManager;
use crate::model::{Error, Result};
use serde::{Deserialize, Serialize};
use sqlb::{Fields, HasFields};
use sqlx::postgres::PgRow;
use sqlx::{FromRow, Pool, Postgres};
use uuid::Uuid;


// types
// serialize because we're sending stuff
#[derive(Clone, Fields, FromRow, Debug, Serialize)]
pub struct User {
    pub id: i64,
    pub username: String,
}
// We are sending this back, so no password

#[derive(Deserialize)]
pub struct UserForCreate {
    pub username: String,
    pub pwd_clear: String,
}

#[derive(Fields)]
struct UserForInsert {
    username: String,
}

#[derive(Clone, FromRow, Fields, Debug)]
pub struct UserForLogin {
    pub id: i64,
    pub username: String,
    pub pwd_encrypted: Option<String>,
    pub pwd_salt: Uuid,
    pub token_salt: Uuid,
}

#[derive(Clone, FromRow, Fields, Debug)]
pub struct UserForAuth {
    pub id: i64,
    pub username: String,
    pub token_salt: Uuid,
}

// end types
// trait
pub trait UserBy: HasFields + for<'r> FromRow<'r, PgRow> + Unpin + Send {}
impl UserBy for User {}
impl UserBy for UserForLogin {}
impl UserBy for UserForAuth {}

// end trait
//bmc
pub struct UserBMC;

impl DbBMC for UserBMC {
    const TABLE: &'static str = "user";
}

impl UserBMC {
    pub async fn get<E>(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<E>
    where
        E: UserBy,
    {
        super::base::get::<UserBMC, E>(ctx, mm, id).await
    }

    pub async fn first_by_username<E>(
        _ctx: &Ctx,
        mm: &ModelManager,
        username: &str,
    ) -> Result<Option<E>>
    where
        E: UserBy,
    {
        let db = mm.db();

        let user = sqlb::select()
            .table(Self::TABLE)
            .and_where("username", "=", username)
            .fetch_optional::<_, E>(db)
            .await?;

        Ok(user)
    }
}
//end bmc

//testks
#[cfg(test)]
mod tests {
    use super::*;
    use crate::_dev_utils;
    use anyhow::{Context, Result};
    use serial_test::serial;
    use tokio::*;

    #[serial]
    #[tokio::test]
    async fn test_first_ok_demo1() -> Result<()> {
        // setuo & fixtures
        let mm = _dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        //this is seeded in dev utils
        let fx_username = "demo1";

        // exec
        let user: User = UserBMC::first_by_username(&ctx, &mm, fx_username)
            .await?
            .context("Should have user demo1")?;

        assert_eq!(user.username, fx_username);

        Ok(())
    }
}

//end tests
