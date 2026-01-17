# Endpoint Configuration

Endpoints are the heart of xrest. Each endpoint is stored in its own YAML file within the `endpoints/` subdirectory of a service.

## File Format

Endpoint files are designed to be self-contained and versioned.

```yaml
id: "e-1700000000000"
serviceId: "s-1700000000000"
name: "Get User Details"
method: "GET"
url: "/users/{{ userId }}"
authenticated: true
authType: "bearer"
params:
  - name: "fields"
    value: "full"
headers:
  - name: "Accept"
    value: "application/json"
body: ""
preflight:
  enabled: true
  method: "POST"
  url: "https://auth.api.com/token"
  body: '{"client_id": "{{ CLIENT_ID }}", "client_secret": "{{ CLIENT_SECRET }}"}'
  tokenKey: "access_token"
  tokenHeader: "Authorization"
lastVersion: 1
versions:
  - version: 1
    config:
      # ... full copy of the request config at this version
    lastUpdated: 1705495200000
```

## Key Properties

| Property | Description |
| --- | --- |
| `id` | Unique identifier for the endpoint. |
| `name` | Human-readable name shown in the sidebar. |
| `method` | HTTP method (GET, POST, PUT, DELETE, etc.). |
| `url` | The path or full URL. Supports `{{ VARIABLE }}` syntax. |
| `authenticated`| Boolean. If true, xrest will ensure a token is present. |
| `preflight` | Configuration for the automated authentication request. |
| `versions` | An array of historic configurations for this endpoint. |

## Request Versioning

xrest automatically tracks changes to your endpoint configuration. When you make a significant change and save, xrest creates a new version in the `versions` array. This allows you to:
- See how the API contract has evolved over time.
- Switch back to an older version of the request if needed.
- Compare the current configuration with a previous one.
