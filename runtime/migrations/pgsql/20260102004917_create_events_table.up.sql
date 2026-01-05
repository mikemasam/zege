-- Add up migration script here
CREATE TABLE IF NOT EXISTS zege_events (
  id BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
  event_ui VARCHAR(100),
  event_organization_id BIGINT NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
  event_bucket_id BIGINT NOT NULL REFERENCES buckets(id) ON DELETE CASCADE,
  event_created_at TIMESTAMPTZ NOT NULL,

  timestamp TIMESTAMPTZ NOT NULL,
  event_name VARCHAR(250),
  event_type VARCHAR(250),
  host VARCHAR(250),
  service VARCHAR(250),
  version VARCHAR(250),
  message TEXT,
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


CREATE INDEX zege_events_service_index ON zege_events (service);
CREATE INDEX zege_events_event_name_index ON zege_events (event_name);
CREATE INDEX zege_events_host_index ON zege_events (host);
CREATE INDEX zege_events_message_index ON zege_events (message);
CREATE INDEX zege_events_timestamp_index ON zege_events (timestamp);
CREATE INDEX zege_events_event_organization_id_index ON zege_events (event_organization_id);
CREATE INDEX zege_events_event_bucket_id_index ON zege_events (event_bucket_id);


