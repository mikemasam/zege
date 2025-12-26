use crate::{
    api::report::list::ZegeReport, api_ensure, ctx::{appcontext::AppContext, dbmanager::DatabasePool}, utils::http::{AppResponse, AppResult}
};
use axum::extract::{Extension, Path};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn report_view_route(
    Extension(app): Extension<Arc<AppContext>>,
    Path(id): Path<i32>,
) -> AppResult<ZegeReport> {
    //let configdb = app.storage.as_ref().unwrap();
    //let db = configdb.lock().await;
    println!("{}", id);
    let sql = "SELECT * FROM reports WHERE id = ?";
    let report = match app.storage.pool.as_ref().unwrap() {
        DatabasePool::Sqlite(pool) => {
            let q = sqlx::query_as::<_, ZegeReport>(sql).bind(id);
            q.fetch_one(pool).await
        }
        DatabasePool::Postgres(pool) => {
            let q = sqlx::query_as::<_, ZegeReport>(sql).bind(id);
            q.fetch_one(pool).await
        }
    };
    api_ensure!(report.is_ok(), "Report not found");
    AppResponse::ok(report.ok(), None)
}
