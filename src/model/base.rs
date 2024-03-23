use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::{Error, Result};
use sqlb::HasFields;
use sqlx::postgres::PgRow;
use sqlx::FromRow;

pub trait DbBMC {
    const TABLE: &'static str;
}

pub async fn create<MC, E>(_ctx: &Ctx, mm: &ModelManager, data: E) -> Result<i64>
where
    MC: DbBMC,
    E: HasFields,
{
    let db = mm.db();

    let fields = data.not_none_fields();
    let (id,) = sqlb::insert()
        .table(MC::TABLE)
        .data(fields)
        .returning(&["id"])
        .fetch_one::<_, (i64,)>(db)
        .await?;

    Ok(id)
}

pub async fn get<MC, Entity>(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Entity>
where
    MC: DbBMC,
    Entity: for<'r> FromRow<'r, PgRow> + Unpin + Send,
{
    let db = mm.db();

    let entity: Entity = sqlb::select()
        .table(MC::TABLE)
        .and_where("id", "=", id)
        .fetch_optional(db)
        .await?
        .ok_or(Error::EntityNotFound {
            entity: MC::TABLE,
            id,
        })?;

    Ok(entity)
}

pub async fn list<MC, Entity>(_ctx: &Ctx, mm: &ModelManager) -> Result<Vec<Entity>>
where
    MC: DbBMC,
    Entity: for<'r> FromRow<'r, PgRow> + Unpin + Send,
{
    let db = mm.db();

    let entity: Vec<Entity> = sqlb::select()
        .table(MC::TABLE)
        .order_by("id")
        .fetch_all(db)
        .await?;

    Ok(entity)
}

pub async fn update<MC, E>(_ctx: &Ctx, mm: &ModelManager, id: i64, data: E) -> Result<()>
where
    MC: DbBMC,
    E: HasFields,
{
    let db = mm.db();

    let fields = data.not_none_fields();
    let count = sqlb::update()
        .table(MC::TABLE)
        .and_where("id", "=", id)
        .data(fields)
        .exec(db)
        .await?;
    if count == 0 {
        return Err(Error::EntityNotFound {
            entity: MC::TABLE,
            id,
        });
    }

    Ok(())
}

pub async fn delete<MC>(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()>
where
    MC: DbBMC,
{
    let db = mm.db();
    let count = sqlb::delete()
        .table(MC::TABLE)
        .and_where("id", "=", id)
        .exec(db)
        .await?;

    if count == 0 {
        return Err(Error::EntityNotFound {
            entity: MC::TABLE,
            id,
        });
    }

    Ok(())
}
