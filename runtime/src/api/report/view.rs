use crate::{
    api::report::list::ZegeReport,
    api_ensure,
    ctx::{appcontext::AppContext, dbmanager::DatabasePool},
    lib::auth::user::papers::UserPaper,
    utils::http::{AppResponse, AppResult},
};
use axum::extract::{Extension, Path};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn report_view_route(
    Extension(app): Extension<Arc<AppContext>>,
    Extension(paper): Extension<UserPaper>,
    Path(id): Path<i32>,
) -> AppResult<ZegeReport> {
    let sql = "SELECT * FROM reports WHERE id = $1 AND organization_id = $2";
    let report = match app.storage.pool.as_ref().unwrap() {
        DatabasePool::Postgres(pool) => {
            sqlx::query_as::<_, ZegeReport>(sql)
                .bind(id)
                .bind(paper.organization.map(|o| o.id).unwrap())
                .fetch_one(pool)
                .await
        }
        _ => todo!("report_view_route"),
    };
    api_ensure!(report.is_ok(), "Report not found");
    AppResponse::ok(report.ok(), None)
}
