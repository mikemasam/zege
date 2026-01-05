-- Add up migration script here
CREATE TABLE IF NOT EXISTS zege_events (
  id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  event_ui VARCHAR(100),
  event_organization_id BIGINT NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
  event_bucket_id BIGINT NOT NULL REFERENCES buckets(id) ON DELETE CASCADE,
  event_created_at TIMESTAMPTZ NOT NULL,

  timestamp TIMESTAMPTZ NOT NULL,
  severity TEXT,
  message TEXT,
  event_name TEXT,
  event_type TEXT,

  -- Error info
  error_type TEXT,
  stack_trace TEXT,

  -- Application / deployment info
  app_instance_id TEXT,
  build_commit TEXT,
  build_id TEXT,

  -- Service info
  service VARCHAR(250),
  version TEXT,
  environment TEXT,

  -- Host info
  hostname TEXT,
  host_ip TEXT,

  -- Tracing
  trace_id TEXT,
  span_id TEXT,
  transaction_id TEXT,
  request_id TEXT,

  -- User info
  user_id TEXT,
  user_name TEXT,
  user_email TEXT,
  session_id TEXT,

  -- HTTP context
  http_method TEXT,
  http_path TEXT,
  http_url TEXT,
  http_status INT,
  client_ip TEXT,

  tags JSONB, 
  data JSONB  
);
ALTER TABLE zege_events ENABLE ROW LEVEL SECURITY;
ALTER TABLE zege_events FORCE ROW LEVEL SECURITY;

CREATE POLICY zege_events_organization_isolation ON zege_events
FOR ALL USING (event_organization_id = NULLIF(current_setting('app.organization_id', true), '0')::BIGINT);

DROP ROLE IF EXISTS zege_events_read_user;
CREATE ROLE zege_events_read_user NOLOGIN NOBYPASSRLS;
GRANT USAGE ON SCHEMA public TO zege_events_read_user;
GRANT SELECT ON public.zege_events TO zege_events_read_user;

