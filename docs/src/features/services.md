# Service Client

In xrest, a **Service** is the primary unit of organization. Unlike other clients that use "Workspaces" or "Collections", a Service in xrest is designed to map 1:1 with an actual microservice or API product.

## Creating a Service

When you create a service, you define:
1. **Name**: A friendly name for the service.
2. **Directory**: The local directory where all service data will be stored. 

::: tip Tip
It is highly recommended to choose a directory that is already a Git repository or initialize one immediately after creation.
:::

## Service Structure

Once a service is created, xrest initializes a specific directory structure:

```text
your-service-directory/
├── service.yaml       # Core service configuration
├── environments.yaml  # Environment variables (DEV, STAGE, PROD)
└── endpoints/         # All API requests for this service
    ├── get-user.yaml
    └── create-order.yaml
```

## Benefits of Service-Centric Design

- **Context Isolation**: When you work on a service, you only see its environments and tokens.
- **Portability**: You can zip up a service directory and send it to someone, or better yet, just share the Git URL.
- **Microservice Mapping**: Easily switch between different microservices without environment variable naming conflicts.

## Importing Specs

xrest supports importing service definitions from:
- **OpenAPI / Swagger**: Import your existing API definitions directly into a xrest Service.
- (More import formats coming soon!)

[Screenshots of Service creation and tree view placeholder]
