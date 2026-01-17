# Collections (Scratchpad)

The **Collections** view provides a lightweight workspace for quick API experimentation without the overhead of a full Service directory.

## When to Use Collections
- **Ad‑hoc testing**: Trying out an endpoint before deciding to create a permanent Service.
- **Prototyping**: Drafting request payloads and environment variables on the fly.
- **Sharing snippets**: Exporting a collection of requests to teammates for review.

## Core Features
- **Search (Cmd+K)**: Quickly filter collections and endpoints.
- **Context Menu**: Right‑click an endpoint to migrate it to a Service via the **Add to Service** dialog.
- **Endpoint Creation**: Create new endpoints directly within a collection.
- **Import/Export**: Swagger/OpenAPI import is available from the toolbar.

## UI Elements
- **Search Bar** – top‑right, placeholder "Search scratchpad...".
- **Add Buttons** – New Collection, New Endpoint, Import from Directory/Swagger.
- **Service Tree** – Displays collections and their endpoints.
- **Tabs** – Each endpoint opens in a request tab similar to Services.

::: tip
Collections are stored in `collections.yaml` files under the application data directory and are **not** version‑controlled by Git unless you explicitly move them into a Service.
:::

[Add screenshots of the Collections view placeholder]
