# Configuration

xrest is highly configurable through human-readable YAML files. There are three levels of configuration:

## [App Configuration](./app-config)
Global settings like theme, window state, and the list of managed services.

## [Service Configuration](./service-config)
Settings specific to a Service, including environment variables and Git details.

## [Endpoint Configuration](./endpoint-config)
Individual API request templates, including pre-flight auth and versioning.

---

### Why YAML?

Traditional API clients often hide their configuration in proprietary databases or complex JSON blobs. xrest uses YAML because it is:
1. **Readable**: Easy for humans to understand and edit.
2. **Versionable**: Ideal for Git diffs and merges.
3. **Hand-Editable**: You can change settings in your IDE if you prefer, and xrest will reflect those changes immediately.
