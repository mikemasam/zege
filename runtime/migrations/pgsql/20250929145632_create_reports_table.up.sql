-- Add up migration script here
CREATE TABLE IF NOT EXISTS zg_reports (
  id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  created_at TIMESTAMPTZ NOT NULL,
  updated_at TIMESTAMPTZ NOT NULL,
  report_name TEXT,
  report_type TEXT,
  report_sql TEXT
);
