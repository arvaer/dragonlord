use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::{Error, Result};

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// region: Task Types
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Task {
    pub id: i64,
    pub title: String,
}

#[derive(Deserialize)]
pub struct TaskForCreate {
    pub title: String,
}

#[derive(Deserialize)]
pub struct TaskForUpdate {
    pub title: Option<String>,
}
// end region: Task Tpyes

//region: TaskBMC
pub struct TaskBMC;

impl TaskBMC {
    pub async fn create(_ctx: &Ctx, mm: &ModelManager, task_c: TaskForCreate) -> Result<i64> {
        let db = mm.db();
        let (id,) =
            sqlx::query_as::<_, (i64,)>("INSERT INTO task (title) values ($1) returning id")
                .bind(task_c.title)
                .fetch_one(db)
                .await?;

        Ok(id)
    }

    pub async fn get(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Task> {
        let db = mm.db();
        let task: Task = sqlx::query_as("SELECT * from task where id = $1")
            .bind(id)
            .fetch_optional(db)
            .await?
            .ok_or(Error::EntityNotFound { entity: "task", id })?;

        return Ok(task);
    }

    pub async fn delete(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
        let db = mm.db();
        let count = sqlx::query("DELETE from task where id = $1")
            .bind(id)
            .execute(db)
            .await?
            .rows_affected();
        if count == 0 {
            return Err(Error::EntityNotFound { entity: "task", id });
        }

        return Ok(());
    }

    pub async fn list(_ctx: &Ctx, mm: &ModelManager) -> Result<Vec<Task>> {
        let db = mm.db();
        let tasks: Vec<Task> = sqlx::query_as("SELECT * from task").fetch_all(db).await?;

        return Ok(tasks);
    }
}
//endregion: TaskBMC

// region: Tests
#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use serial_test::serial;

    #[serial]
    #[tokio::test]
    async fn test_create_ok() -> Result<()> {
        // setup
        let mm = crate::_dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_title = "test_create_ok title";

        // Exec
        let task_c = TaskForCreate {
            title: fx_title.to_string(),
        };
        let id = TaskBMC::create(&ctx, &mm, task_c).await?;

        // check
        let task = TaskBMC::get(&ctx, &mm, id).await?;
        assert_eq!(task.title, fx_title);

        TaskBMC::delete(&ctx, &mm, id).await;
        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_list_ok() -> Result<()> {
        //setup
        let mm = crate::_dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_titles = &["test_list_ok-task 01", "test_list_ok-task 02"];
        crate::_dev_utils::seed_tasks(&ctx, &mm, fx_titles).await?;

        //exec
        let tasks = TaskBMC::list(&ctx, &mm).await?;

        // -- check
        let tasks: Vec<Task> = tasks
            .into_iter()
            .filter(|t| t.title.starts_with("test_list_ok-task "))
            .collect();
        assert_eq!(tasks.len(), 2, "number of seeded tasks");


        // -clean
        for task in tasks.iter() {
            TaskBMC::delete(&ctx, &mm, task.id).await?;
        }
        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_get_err_not_found() -> Result<()> {
        //setup
        let mm = crate::_dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_id = 100 as i64;

        //Exec
        let res = TaskBMC::get(&ctx, &mm, fx_id).await;

        //check
        assert!(matches!(
            res,
            Err(Error::EntityNotFound {
                entity: "task",
                id: 100
            })
        ));
        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_delete_err_not_found() -> Result<()> {
        // setup
        let mm = crate::_dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_id = 100 as i64;

        //Exec
        let res = TaskBMC::delete(&ctx, &mm, fx_id).await;

        // check
        assert!(matches!(
            res,
            Err(Error::EntityNotFound {
                entity: "task",
                id: 100
            })
        ));
        Ok(())
    }

}

// end region: Tests
