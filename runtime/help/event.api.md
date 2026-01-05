---
# Event API

This API accepts an event payload in JSON format. Each field is optional unless specified. Events are used for logging, observability, and analytics.

**Content-Type:** `application/json`
**Method:** `POST`
---

## JSON Structure

```json
{
  "timestamp": "2025-09-12T10:00:00Z",
  "service": "app1",
  "event_name": "test1",
  "message": "Something went wrong",
  "version": "1.2.3",
  "environment": "production",
  "host": "server1",
  "tracing": {
    "trace_id": "abc-def-123",
    "span_id": "span-001",
    "transaction_id": "txn-123"
  },
  "meta": {
    "jwt": ""
  },
  "user": {
    "id": "user-01",
    "name": "Alice",
    "email": "alice@example.com",
    "session_id": "sess-123"
  },
  "data": { "custom_field": "value" }
}
```

---

## Field Descriptions

### Event

| Field       | Type             | Description                                            |
| ----------- | ---------------- | ------------------------------------------------------ |
| `timestamp` | string (RFC3339) | When the event occurred. **Required**.                 |
| `service`   | string           | version of service . **Required**.                     |
| `version`   | string           | environment of service . **Required**.                 |
| `message`   | string           | Human-readable description of the event.               |
| `host`      | string           | Host/machine name/id.                  |
| `tracing`   | object           | Distributed tracing identifiers (see **TracingInfo**). |
| `data`      | object           | Additional custom metadata.                            |

---

### Objects

#### TracingInfo

| Field            | Type   | Description               |
| ---------------- | ------ | ------------------------- |
| `trace_id`       | string | Unique trace ID.          |
| `span_id`        | string | Span ID within the trace. |
| `transaction_id` | string | Transaction identifier.   |

---
