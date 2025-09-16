-- Add up migration script here
CREATE TABLE evt_events (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  timestamp TEXT NOT NULL,
  severity TEXT,
  message TEXT,
  event_name TEXT,

  -- Error info
  error_type TEXT,
  error_message TEXT,
  stack_trace TEXT,

  -- Application / deployment info
  app_instance_id TEXT,
  build_commit TEXT,
  build_id TEXT,
  app_region TEXT,

  -- Service info
  service_name TEXT,
  service_version TEXT,
  environment TEXT,

  -- Host info
  hostname TEXT,
  host_ip TEXT,
  host_region TEXT,
  host_provider TEXT,

  -- Tracing
  trace_id TEXT,
  span_id TEXT,
  transaction_id TEXT,

  -- User info
  user_id TEXT,
  user_name TEXT,
  user_email TEXT,
  session_id TEXT,

  -- HTTP context
  http_method TEXT,
  http_path TEXT,
  http_url TEXT,
  http_origin TEXT,
  http_status INT,
  client_ip TEXT,
  user_agent TEXT,

  -- Request info
  request_id TEXT,
  referrer TEXT,
  protocol TEXT,
  response_size_bytes BIGINT,

  tags JSONB, 
  labels JSONB,
  meta JSONB  
);
