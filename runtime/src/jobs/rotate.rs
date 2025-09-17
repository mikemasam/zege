use core::panic;

use crate::ctx::dbmanager::DbManager;

pub async fn rotate_events() {
    let backup_db_name = "data/backup2.events.db";
    {
        let _bk_db = DbManager::connect_to_event_db_with_name(backup_db_name, true)
            .await
            .map_err(|err| {
                panic!("Failed to open {backup_db_name} with error {err}");
            })
            .unwrap();
        _bk_db.close_db().await;
    }
    let rotation_db = DbManager::connect_to_event_db(true)
        .await
        .unwrap_or_else(|err| {
            panic!("Failed to open events db with error {err}",);
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
            SELECT * FROM main.evt_events as a 
            WHERE datetime(a.timestamp) < datetime('now', '-2 minute') 
            AND NOT EXISTS (SELECT b.ui from db2.evt_events as b where b.ui = a.ui);
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
        println!("> {} events were archived.", rows_affected.rows_affected());
        panic!("Failed to delete in events rotation with error {err}");
    });
    println!(
        "> {} archived & {} cleared.",
        rows_affected.rows_affected(),
        rows_deleted.rows_affected()
    );

    let checker = r#"
            SELECT COUNT(DISTINCT a.ui), COUNT(DISTINCT b.ui) FROM main.evt_events as a, db2.evt_events as b 
        "#;
    let row: (i64, i64) = sqlx::query_as(checker)
        .fetch_one(&mut *conn)
        .await
        .unwrap_or_else(|err| {
            panic!("Failed to read stats in events rotation with error {err}");
        });
    println!("> Stats: {} live, {} archives .", row.0, row.1);
    let _ = conn.close().await;
}
