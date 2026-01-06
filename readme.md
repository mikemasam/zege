
Ship better logs. 📦
# Zege — a simple, flexible, fast ⚡ event logging service for observability, debugging, and analytics written in Rust.
JSON in → Insights out. 
### More Docs available /docs 
---

## Prerequisites

- Docker installed
- A running PostgreSQL instance
- A configuration file on the host machine

---

## Configuration

Create a configuration file named `config.yaml`:

```yaml
verbose: all

feature:
  jwt_secret: default-secret

database:
  name: zege03
  host: <HOST_IP>
  username: postgres
  password: postgres
```

---

## Running service

```bash
sudo docker run -d \
  --name zege \
  --restart unless-stopped \
  -p 3432:3432 \
  -v /absolute/path/to/config.yaml:/app/config.yaml \
  masamtz/zege:latest
```

## Access the Service

### Web UI & API:
```bash
http://localhost:3432
```
---

### Logging Event API 
```bash
http://localhost:3432/api/v1/events/i?bucket_key=zgb019b8f8.....
```

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
  "meta": {
    "jwt": ""
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
| `data`      | object           | Additional custom metadata.                            |

---


