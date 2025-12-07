use crate::{
    api::reports::ZegeReport,
    ctx::{appcontext::AppContext, dbmanager::DatabasePool},
    utils::http::AppResponse,
};
use axum::{
    Json,
    extract::{Extension, Path},
    response::IntoResponse,
};
use std::sync::Arc;
use tokio::sync::Mutex;

fn sqlite() {}
pub async fn report_view_route(
    Extension(appcontext): Extension<Arc<Mutex<AppContext>>>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let app = appcontext.lock().await;
    let configdb = app.storage.as_ref().unwrap();
    let db = configdb.lock().await;
    println!("{}", id);
    let sql = "SELECT * FROM zg_reports WHERE id = ?";
    let report = match db.pool.as_ref().unwrap() {
        DatabasePool::Sqlite(pool) => {
            let q = sqlx::query_as::<_, ZegeReport>(sql).bind(id);
            q.fetch_one(pool).await
        }
        DatabasePool::Postgres(pool) => {
            let q = sqlx::query_as::<_, ZegeReport>(sql).bind(id);
            q.fetch_one(pool).await
        }
    };

    if report.is_err() {
        return Json(AppResponse {
            status: 400,
            message: "Report not found".to_string(),
            data: None,
        });
    }
    //println!("{:?}", report);
    Json(AppResponse {
        status: 200,
        message: String::new(),
        data: report.ok(),
    })
}

