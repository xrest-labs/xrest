# Git Integration

Sharing API collections has always been a pain point. xrest solves this by using **Git** as its native synchronization engine.

## Automatic Commits

xrest is designed to keep your local files in sync with your service configuration. Every time you:
- Save an endpoint
- Update an environment variable
- Modify service settings

xrest performs a local commit (optional configuration coming soon) to ensure your history is preserved.

## Syncing with Remote

The Service Settings page includes a **Sync** capability:
1. **Pull**: Fetch and merge the latest API definitions from your team.
2. **Push**: Share your updated endpoint templates or new environments.

## Handling Conflicts

Since everything is stored in YAML, Git handles most merges automatically. If a conflict occurs during a sync, xrest will prompt you to resolve it using your standard Git workflow tools.

## Why no Cloud Sync?

By using Git, xrest ensures:
- **Data Privacy**: Your API keys and internal URL patterns stay within your own Git infrastructure.
- **Workflow Integration**: API changes can follow the same PR/Code Review process as your application code.
- **Offline Access**: Everything is stored locally, so xrest works perfectly even without an internet connection.

[Screenshot of Git Status in Service Settings]
