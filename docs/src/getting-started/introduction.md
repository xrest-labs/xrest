# Introduction

xrest is a service-first REST client designed specifically for modern microservice architectures. While traditional API clients focus on individual, ad-hoc requests, xrest treats APIs as first-class **Services**.

## Why xrest?

Modern development involves interacting with dozens of internal and external APIs. Traditional tools often lead to a "workspace sprawl" where configurations are hard to share, environments are messy, and production accidents are just one click away.

xrest solves this by:

- **Enforcing a Service Hierarchy**: Every request belongs to a Service, ensuring logical organization.
- **Git-Native Storage**: All data (services, environments, endpoints) is stored in YAML, making it easy to version control and share via Git.
- **Microservice Focus**: Built-in defaults for environment-switching and pre-flight authentication.
- **Safety First**: Visual cues and confirmation steps for "Unsafe" (Production) environments.

## How it Works

1. **Create a Service**: Define a service and point it to a directory (ideally a Git repo).
2. **Configure Environments**: Define variables for `DEV`, `STAGE`, and `PROD`.
3. **Define Endpoints**: Create requests that use environment variables.
4. **Sync**: Push your changes to Git so your team is always in sync with the latest API definitions.

::: info :::
xrest is currently in active development. We recommend using it for internal tool development and API testing.
:::
