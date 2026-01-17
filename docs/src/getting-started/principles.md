# Principles

xrest is guided by a few core principles that differentiate it from other REST clients.

## 1. Service-First, Not Request-First
In xrest, you don't just "make a request." You create a **Service**. A Service is a collection of endpoints that share a base configuration, common environments, and authentication logic. This mirrors how microservices are actually built and deployed.

## 2. Configuration is Code
Everything in xrest is stored as **YAML files**. 
- No proprietary binary formats.
- No opaque databases.
- Just human-readable files that you can diff, review, and merge.

## 3. Git as the Source of Truth
Instead of building a custom cloud sync solution, xrest leverages **Git**. By linking a Service to a Git repository, you get:
- Team collaboration for free.
- Full audit history of API changes.
- Branching and merging workflows for API experimental features.

## 4. Safety by Default
We believe that interacting with production environments should feel different than interacting with development environments.
- **Visual Distinction**: Unsafe environments (PROD) have distinct UI markers.
- **Active Confirmation**: Requests to unsafe environments require an extra confirmation step to prevent "fat-finger" errors.

## 5. Automated Pre-flight
Microservices often require complex authentication steps (e.g., fetching an OAuth2 token from an identity provider). xrest's **Pre-flight** system automates this by executing an authentication request before the main request, caching the token, and injecting it into headers automatically.
