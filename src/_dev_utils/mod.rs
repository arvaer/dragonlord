mod dev_db;
use tokio::sync::OnceCell;
use tracing::info;

use crate::{model::{ModelManager, self, task::{Task, TaskBMC, TaskForCreate}}, ctx::Ctx};

pub async fn init_dev() {
    static INIT: OnceCell<()> = OnceCell::const_new();
    INIT.get_or_init(|| async {
        info!("{:12} - init_dev", "DEV-ONLY");
        dev_db::init_dev_db().await.unwrap();
    })
    .await;
}

pub async fn init_test() -> ModelManager {
    static INIT: OnceCell<ModelManager> = OnceCell::const_new();

    let mm = INIT.get_or_init(|| async {
        init_dev();
        ModelManager::new().await.unwrap()
    }).await;

    return mm.clone();

}

pub async fn seed_tasks(ctx: &Ctx, mm: &ModelManager, titles:&[&str]) -> model::Result<Vec<Task>> {
    let mut tasks:Vec<Task> = Vec::new();
    for title in titles.to_owned(){
        let id = TaskBMC::create( &ctx, &mm, TaskForCreate{title: String::from(title)}).await?;
        tasks.push(TaskBMC::get(&ctx, &mm, id).await?);
    }

    return Ok(tasks);
}
