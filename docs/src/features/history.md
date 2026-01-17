# History

xrest keeps a comprehensive history of your request executions, allowing you to debug past issues and reuse previous payloads.

## Global History

The History panel (accessible via the sidebar) shows a chronological list of all requests made across all services. 

Each history entry includes:
- **Status Code**: (e.g., 200 OK, 404 Not Found)
- **Method & URL**: The exact request executed.
- **Timestamp**: When the request was sent.
- **Duration**: How long the request took to complete.

## Inspecting Past Requests

Clicking on a history item opens a read-only view of exactly what was sent and received:
- **Full Request**: Exact headers and body sent (after variable interpolation).
- **Full Response**: Headers and body received from the server.

## Restoring from History

You can quickly restore a previous request configuration by clicking "Restore". This will open the request in a new tab, allowing you to modify it and run it again.

## Persistent Storage

History is stored in a local SQLite database within the xrest application data directory, ensuring it remains fast even with thousands of entries.

[Screenshot of History Sidebar]
