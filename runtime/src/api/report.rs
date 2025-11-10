use crate::{api::reports::ZegeReport, ctx::appcontext::AppContext, utils::http::AppResponse};
use axum::{
    Json,
    extract::{Extension, Path },
    response::IntoResponse,
};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn report_view_route(
    Extension(appcontext): Extension<Arc<Mutex<AppContext>>>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let app = appcontext.lock().await;
    let configdb = app.configdb.as_ref().unwrap();
    let db = configdb.lock().await;
    println!("{}", id);
    let reports = sqlx::query_as::<_, ZegeReport>("SELECT * FROM zg_reports where id = ?")
        .bind(id)
        .fetch_one(db.pool.as_ref().unwrap())
        .await;
    if reports.is_err() {
        return Json(AppResponse {
            status: 400,
            message: "Report not found".to_string(),
            data: None,
        });
    }
    let report = reports.ok();
    //println!("{:?}", report);
    Json(AppResponse {
        status: 200,
        message: String::new(),
        data: report,
    })
}
