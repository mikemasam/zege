use crate::ctx::dbmanager::{DbPoolManager, DbManagerConnectOptions};
use crate::utils::appenv::AppLogger;
use core::panic;
use tokio::runtime::Handle;
use tokio::time::{self, Instant};

pub async fn start_scheduler() {
    let mut interval = time::interval(time::Duration::from_secs(60 * 5));
    AppLogger::log(format!(
        "Starting Background Job #{:?}.",
        std::thread::current().id()
    ));
    loop {
        interval.tick().await;
        let start_time = Instant::now();
        let _ = tokio::task::spawn_blocking(|| {
            let rt = tokio::runtime::Handle::current();
            rt.block_on(async {
                //rotate_events().await;
            });
        })
        .await;
        let elapsed_time = start_time.elapsed();
        let workers_count = Handle::current().metrics().num_workers();
        AppLogger::log(format!("# rotation time: {elapsed_time:?}, workers: {workers_count}"));
    }
}

/*
async fn rotate_events() {
    let mut backup_db_name = String::new();
    {
        let _bk_db = DbManager::connect(DbManagerConnectOptions {
            migrate: true,
            backup: true,
        })
        .await
        .map_err(|err| {
            panic!("Failed to open {backup_db_name} with error {:?}", err);
        })
        .unwrap();
        backup_db_name = _bk_db.clone().id;
        _bk_db.close_db().await;
    }
    let rotation_db = DbManager::connect(DbManagerConnectOptions {
        migrate: true,
        backup: false,
    })
    .await
    .unwrap_or_else(|err| {
        panic!("Failed to open events db with error {:?}",err);
    });
    let mut conn = rotation_db
        .pool
        .as_ref()
        .unwrap()
        .acquire()
        .await
        .unwrap_or_else(|err| panic!(" Failed to acquire connection for events rotation {err}"));
    sqlx::query(format!("ATTACH DATABASE '{backup_db_name}' AS db2;",).as_str())
        .execute(&mut *conn)
        .await
        .unwrap_or_else(|err| {
            panic!("Failed to attach {backup_db_name} with error {err}");
        });
    let rows_affected = sqlx::query(
        "
            INSERT INTO db2.evt_events
            SELECT a.*
            FROM main.evt_events AS a
            LEFT JOIN db2.evt_events AS b
            ON a.ui = b.ui
            WHERE b.ui IS NULL
            AND a._time < datetime('now', '-2 minute');
        ",
    )
    .execute(&mut *conn)
    .await
    .unwrap_or_else(|err| {
        panic!("Failed to copy in events rotation with error {err}");
    });
    let rows_deleted = sqlx::query(
        "
            DELETE FROM main.evt_events as a 
            WHERE EXISTS (SELECT b.ui from db2.evt_events as b where b.ui = a.ui);
        ",
    )
    .execute(&mut *conn)
    .await
    .unwrap_or_else(|err| {
        AppEnv::log(format!("> {} events were archived.", rows_affected.rows_affected()));
        panic!("Failed to delete in events rotation with error {err}");
    });
    AppEnv::log(format!(
        "> {} archived & {} cleared.",
        rows_affected.rows_affected(),
        rows_deleted.rows_affected()
    ));

    let live_row: (i64,) = sqlx::query_as(
        r#"
            SELECT COUNT(a.ui) FROM main.evt_events as a 
        "#,
    )
    .fetch_one(&mut *conn)
    .await
    .unwrap_or_else(|err| {
        panic!("Failed to read stats in events rotation with error {err}");
    });
    let archive_row: (i64,) = sqlx::query_as(
        r#"
            SELECT COUNT(b.ui) FROM db2.evt_events as b 
        "#,
    )
    .fetch_one(&mut *conn)
    .await
    .unwrap_or_else(|err| {
        panic!("Failed to read stats in events rotation with error {err}");
    });
    AppEnv::log(format!("> Stats: {} live, {} archives .", live_row.0, archive_row.0));
    let _ = conn.close().await;
}
*/
