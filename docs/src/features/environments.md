# Environments

xrest provides a robust environment management system that allows you to swap configuration sets with a single click.

## Built-in Environments

By default, every new service comes with three environments:
- **DEV**: For local development or sandbox testing.
- **STAGE**: For pre-production validation.
- **PROD**: For live production traffic.

## Managing Variables

Each environment consists of a list of key-value pairs. 

### Interpolation Syntax
Use the double curly brace syntax `{{ VARIABLE_NAME }}` in your:
- URL
- Headers
- Query Parameters
- Request Body

Example: `{{ BASE_URL }}/v1/users`

## Common Variables

It's common practice to define a `BASE_URL` in every environment. When you switch environments using the selector in the top bar, xrest immediately updates all active tabs to reflect the variables of the new environment.

## Hierarchy & Resolution

Variables are resolved in the following order:
1. **Endpoint Overrides**: (Future feature)
2. **Current Service Environment**: Variables defined in the currently selected environment of the active service.

[Screenshot of Environment Editor]
