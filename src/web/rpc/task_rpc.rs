use crate::ctx::Ctx;
use crate::model::task::{Task, TaskBMC, TaskForCreate, TaskForUpdate};
use crate::model::ModelManager;
use crate::web::Result;

use super::{ParamsById, ParamsForCreate, ParamsForUpdate};

pub async fn create_task(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForCreate<TaskForCreate>,
) -> Result<Task> {
    let ParamsForCreate { data } = params;
    let id = TaskBMC::create(&ctx, &mm, data).await?;
    let task = TaskBMC::get(&ctx, &mm, id).await?;
    Ok(task)
}

pub async fn list_tasks(ctx: Ctx, mm: ModelManager) -> Result<Vec<Task>> {
    Ok(TaskBMC::list(&ctx, &mm).await?)
}

pub async fn update_task(
    ctx: Ctx,
    mm: ModelManager,
    params: ParamsForUpdate<TaskForUpdate>,
) -> Result<()> {
    let ParamsForUpdate { id, data } = params;
    TaskBMC::update(&ctx, &mm, id, data).await?;
    Ok(())
}

pub async fn delete_task(ctx: Ctx, mm: ModelManager, params: ParamsById) -> Result<(Task)> {
    let ParamsById { id } = params;
    let task = TaskBMC::get(&ctx, &mm, id).await?;
    TaskBMC::delete(&ctx, &mm, id).await?;
    Ok(task)
}
