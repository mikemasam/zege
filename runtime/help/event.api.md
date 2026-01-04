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
  "severity": "ERROR",
  "message": "Something went wrong",
  "error": {
    "error_type": "RuntimeError",
    "stack_trace": "..."
  },
  "app": {
    "instance_id": "app-01",
    "build_commit": "abc123",
    "build_id": "42",
    "region": "us-east-1"
  },
  "service": {
    "version": "1.2.3",
    "environment": "production"
  },
  "host": {
    "hostname": "server01",
    "host_ip": "10.0.0.1",
  },
  "tracing": {
    "trace_id": "abc-def-123",
    "span_id": "span-001",
    "transaction_id": "txn-123"
  },
  "user": {
    "id": "user-01",
    "name": "Alice",
    "email": "alice@example.com",
    "session_id": "sess-123"
  },
  "http": {
    "method": "POST",
    "path": "/api/events",
    "status": 200,
    "client_ip": "192.168.0.1",
  },
  "tags": ["backend", "critical"],
  "labels": { "feature": "login", "team": "auth" },
  "data": { "custom_field": "value" }
}
```

---

## Field Descriptions

### Top-level

| Field       | Type             | Description                                            |
| ----------- | ---------------- | ------------------------------------------------------ |
| `timestamp` | string (RFC3339) | When the event occurred. **Required**.                 |
| `service_name` | string           | name of service . **Required**.                    |
| `severity`  | string           | Severity level: `"INFO"`, `"WARN"`, `"ERROR"`, etc.    |
| `message`   | string           | Human-readable description of the event.               |
| `error`     | object           | Error details (see **ErrorInfo**).                     |
| `app`       | object           | Application/deployment information (see **AppInfo**).  |
| `service`   | object           | Service information (see **ServiceInfo**).             |
| `host`      | object           | Host/machine info (see **HostInfo**).                  |
| `tracing`   | object           | Distributed tracing identifiers (see **TracingInfo**). |
| `user`      | object           | User context (see **UserInfo**).                       |
| `http`      | object           | HTTP request/response context (see **HttpInfo**).      |
| `tags`      | array of strings | Optional tags for categorization.                      |
| `labels`    | object           | Flexible key/value labels.                             |
| `data`      | object           | Additional custom metadata.                            |

---

### Sub-Objects

#### ErrorInfo

| Field           | Type   | Description                         |
| --------------- | ------ | ----------------------------------- |
| `error_type`    | string | Error type or class.                |
| `stack_trace`   | string | Optional stack trace for debugging. |

#### AppInfo

| Field          | Type   | Description                         |
| -------------- | ------ | ----------------------------------- |
| `instance_id`  | string | Unique instance of the application. |
| `build_commit` | string | Git commit hash.                    |
| `build_id`     | string | Build identifier.                   |
| `region`       | string | Deployment region.                  |

#### ServiceInfo

| Field         | Type   | Description                                    |
| ------------- | ------ | ---------------------------------------------- |
| `name`        | string | Service name.                                  |
| `version`     | string | Service version.                               |
| `environment` | string | Environment: `"production"`, `"staging"`, etc. |

#### HostInfo

| Field            | Type   | Description              |
| ---------------- | ------ | ------------------------ |
| `hostname`       | string | Hostname of the machine. |
| `host_ip`        | string | IP address of the host.  |

#### TracingInfo

| Field            | Type   | Description               |
| ---------------- | ------ | ------------------------- |
| `trace_id`       | string | Unique trace ID.          |
| `span_id`        | string | Span ID within the trace. |
| `transaction_id` | string | Transaction identifier.   |

#### UserInfo

| Field        | Type   | Description         |
| ------------ | ------ | ------------------- |
| `id`         | string | User ID.            |
| `name`       | string | User name.          |
| `email`      | string | User email.         |
| `session_id` | string | Session identifier. |

#### HttpInfo

| Field        | Type    | Description                          |
| ------------ | ------- | ------------------------------------ |
| `method`     | string  | HTTP method: `"GET"`, `"POST"`, etc. |
| `path`       | string  | Request path.                        |
| `status`     | integer | HTTP response status code.           |
| `client_ip`  | string  | IP address of the client.            |

---


