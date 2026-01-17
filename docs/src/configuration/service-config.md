# Service Configuration

Each service manages its own configuration within its dedicated directory.

## `service.yaml`
This file contains the core identity and settings for the service.

```yaml
id: "s-1700000000000"
name: "My Service"
gitUrl: "https://github.com/my-org/my-service-api.git"
```

## `environments.yaml`
This file defines the environment sets and their variables.

```yaml
- name: DEV
  isUnsafe: false
  variables:
    - name: BASE_URL
      value: "http://localhost:3000"
- name: PROD
  isUnsafe: true
  variables:
    - name: BASE_URL
      value: "https://api.myapp.com"
```

## Environment Properties

| Property | Type | Description |
| --- | --- | --- |
| `name` | String | The environment name (e.g., DEV, STAGE, PROD). |
| `isUnsafe` | Boolean | If true, enables safety guards (red UI, confirmation dialog). |
| `variables` | Array | List of environment variables. |
| `variables.name`| String | The name used in templates: `{{ NAME }}`. |
| `variables.value`| String | The value to interpolate. |

---

### Best Practices

- **Avoid Secrets in Git**: While xrest supports storing variables in YAML, avoid committing sensitive production secrets (like private keys) to public repositories. We recommend using placeholders for secrets and having developers fill them in locally, or using separate "Secret" environments (roadmap).
- **Standardized Base URL**: Consistently use `BASE_URL` across all your services to make templates more portable.
