-- Add up migration script here
CREATE TABLE IF NOT EXISTS reports (
  id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  report_name TEXT,
  report_type TEXT,
  report_sql TEXT,
  organization_id BIGINT NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
  user_id BIGINT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  created_at TIMESTAMPTZ NOT NULL,
  updated_at TIMESTAMPTZ NOT NULL
);
