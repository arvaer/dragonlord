use crate::ctx::Ctx;
use crate::model::ModelManager;
use crate::model::{Error, Result};
use crate::model::base::{DbBMC, create, update, get, list, delete};

use serde::{Deserialize, Serialize};
use sqlb::Fields;
use sqlx::FromRow;


// region: Task Types
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Task {
    pub id: i64,
    pub title: String,
}

#[derive(Fields, Deserialize, Debug)]
pub struct TaskForCreate {
    pub title: String,
}

#[derive(Fields, Deserialize)]
pub struct TaskForUpdate {
    pub title: Option<String>,
}
// end region: Task Tpyes

//region: TaskBMC
pub struct TaskBMC;

impl DbBMC for TaskBMC{
    const TABLE: &'static str = "task";
}

impl TaskBMC {
    pub async fn create(ctx: &Ctx, mm: &ModelManager, task_c: TaskForCreate) -> Result<i64> {
        create::<TaskBMC, TaskForCreate>(ctx, mm, task_c).await
    }

    pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Task> {
        get::<TaskBMC, Task>(ctx, mm, id).await
    }

    pub async fn list(ctx: &Ctx, mm: &ModelManager) -> Result<Vec<Task>> {
        list::<TaskBMC, Task>(ctx, mm).await
    }

    pub async fn update(ctx: &Ctx, mm: &ModelManager, id: i64, task_u: TaskForUpdate) -> Result<()> {
        update::<TaskBMC, TaskForUpdate>(ctx, mm, id, task_u).await
    }

    pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
        delete::<TaskBMC>(ctx, mm, id).await
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
    async fn test_update_ok() -> Result<()> {
        //setup
        let mm = crate::_dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_title = "test_update_ok title";
        let fx_title_updated = "test_update_ok title updated";

        // Exec
        let task_c: TaskForCreate = TaskForCreate { title: String::from(fx_title) };
        let task_u: TaskForUpdate = TaskForUpdate { title: Some(String::from(fx_title_updated)) };

        let id = TaskBMC::create(&ctx, &mm, task_c).await?;
        //println!("TASK CREATED {}", id);
        //println!("TASK FROM BE TITLE: {}", TaskBMC::get(&ctx, &mm, id).await?.title);
        TaskBMC::update(&ctx, &mm, id, task_u).await?;
        //println!("TASK JUST UPDATED: {}", id);
        //println!("TASK FROM BE TITLE: {}", TaskBMC::get(&ctx, &mm, id).await?.title);

        //Check
        assert_eq!(TaskBMC::get(&ctx, &mm, id).await?.title, fx_title_updated);

        //cleanup
        TaskBMC::delete(&ctx, &mm, id).await?;
        Ok(())

    }


    #[serial]
    #[tokio::test]
    async fn test_get_err_not_found() -> Result<()> {
        //setup
        let mm = crate::_dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_id = 100_i64;

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
    async fn test_update_err_not_found() -> Result<()> {
        //setup
        let mm = crate::_dev_utils::init_test().await;
        let ctx = Ctx::root_ctx();
        let fx_id = 100_i64;
        let task_u = TaskForUpdate { title: Some("test_update_err_not_found title".to_string()) };

        //Exec
        let res = TaskBMC::update(&ctx, &mm, fx_id, task_u).await;

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
