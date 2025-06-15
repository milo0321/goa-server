use super::core::ModuleFactory;
use crate::db::AppState;
use axum::Router;
use inventory;
use tokio::task::JoinHandle;

pub fn load_plugins(state: AppState) -> (Router<AppState>, Vec<JoinHandle<()>>) {
    let mut router = Router::new();
    let mut tasks: Vec<JoinHandle<()>> = vec![];

    for factory in inventory::iter::<ModuleFactory> {
        // 使用工厂函数实例化模块
        let module = factory.0(state.clone());
        println!("Registering module: {}", module.name());

        router = router.nest("/api", module.route());

        // 启动模块的后台任务（如果有）
        if let Some(task) = module.init() {
            tasks.push(task);
        }
    }

    (router, tasks)
}
