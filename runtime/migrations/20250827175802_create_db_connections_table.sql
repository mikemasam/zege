-- Up: apply migration
CREATE TABLE IF NOT EXISTS db_connections (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    db_type TEXT NOT NULL,
    host TEXT,
    port TEXT,
    dbname TEXT,
    username TEXT,
    password TEXT
);

-- Down: rollback migration
DROP TABLE IF EXISTS db_connections;
