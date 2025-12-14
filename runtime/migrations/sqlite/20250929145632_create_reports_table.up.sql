-- Add up migration script here
CREATE TABLE IF NOT EXISTS zg_reports (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  report_name TEXT,
  report_type TEXT,
  report_sql TEXT
);
