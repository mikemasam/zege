use crate::{
    api::reports::ZegeReport,
    ctx::appcontext::AppContext,
    utils::{dbutil::rows_to_json_vec, http::AppResponse},
};
use axum::{
    extract::{Extension, Path},
    response::IntoResponse,
};
use serde::Serialize;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Serialize)]
struct ReadOutput {
    data: Option<Vec<Value>>,
    report: ZegeReport,
}
pub async fn report_read_route(
    Extension(appcontext): Extension<Arc<Mutex<AppContext>>>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let app = appcontext.lock().await;
    let db = app.storage.as_ref().unwrap().lock().await;
    let out = sqlx::query_as::<_, ZegeReport>("SELECT * FROM zg_reports where id = ?")
        .bind(id)
        .fetch_one(db.pool.as_ref().unwrap())
        .await;
    if out.is_err() {
        return AppResponse::error("Report not found", None);
    }
    let report = out.ok().unwrap();
    let rows = sqlx::query(report.report_sql.as_str())
        //let rows = sqlx::query(evt_events)
        .fetch(db.pool.as_ref().unwrap());
    let data = rows_to_json_vec(rows).await;
    let output = ReadOutput {
        data: data.ok(),
        report,
    };
    AppResponse::ok(Some(output), None)
}
