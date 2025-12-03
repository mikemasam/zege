-- Add up migration script here
CREATE TABLE zg_reports (
  id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP NOT NULL,
  report_name TEXT,
  report_type TEXT,
  report_sql TEXT
);
