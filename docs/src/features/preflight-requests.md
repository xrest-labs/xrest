# Pre-flight Requests

Most modern APIs require some form of authentication, often involving short-lived Bearer tokens. Manually fetching these tokens and pasting them into headers is a tedious task. xrest's **Pre-flight** system automates this entire process.

## How it Works

1. **Pre-flight Check**: Before xrest sends your main request, it checks if the endpoint requires authentication.
2. **Token Validity**: It looks for a cached token and checks if it's still valid (based on your configuration).
3. **Execution**: If the token is missing or expired, xrest automatically executes a "Pre-flight Request" (e.g., an `/auth/login` call).
4. **Token Extraction**: xrest extracts the token from the response body using a configurable key.
5. **Injection**: The token is injected into the `Authorization` header of your main request.

## Configuration

In the Endpoint settings, you can define the Pre-flight configuration:

- **Method & URL**: The endpoint used to fetch the token.
- **Body & Headers**: Any data required for authentication (e.g., client credentials).
- **Token Key**: The path to the token in the JSON response (e.g., `access_token`).
- **Cache Token**: Whether to cache the token for subsequent requests.
- **Expiration**: How long the token is valid, or a key in the response that indicates expiration (e.g., `expires_in`).

## Benefits

- **Set and Forget**: Once configured, you never have to worry about token expiration again.
- **Clean Requests**: Your main request doesn't need hardcoded tokens; it just uses the `authenticated: true` flag.
- **Microservice Friendly**: Easily handle different auth mechanisms for different services.

[Screenshot of Pre-flight settings panel]
