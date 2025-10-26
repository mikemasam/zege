-- Add up migration script here
CREATE TABLE zg_reports (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  timestamp TEXT NOT NULL,
  report_name TEXT,
  report_type TEXT,
  report_sql TEXT
);
