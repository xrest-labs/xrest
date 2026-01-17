# App Configuration

The global application configuration is stored in `settings.yaml` within the xrest data directory.

## Location

- **macOS**: `~/Library/Application Support/io.ak.xrest/settings.yaml`
- **Windows**: `%APPDATA%/io.ak.xrest/settings.yaml`
- **Linux**: `~/.config/io.ak.xrest/settings.yaml`

## File Structure

```yaml
theme: "system" # options: "light", "dark", "system"
services:
  - id: "s-1700000000000"
    name: "User Management Service"
    directory: "/Users/dev/projects/user-service-api"
  - id: "s-1700000000001"
    name: "Inventory API"
    directory: "/Users/dev/projects/inventory-api"
```

## Properties

| Property | Type | Description |
| --- | --- | --- |
| `theme` | String | The application's visual theme. |
| `services` | Array | A list of registered services. |
| `services.id` | String | A unique identifier for the service (usually a timestamp). |
| `services.name` | String | Display name shown in the sidebar. |
| `services.directory`| String | **Absolute path** to the service's data directory. |

---

### Managing Services
While you can edit this file manually, it is recommended to manage services through the xrest UI to ensure IDs are generated correctly and directories are initialized properly.
