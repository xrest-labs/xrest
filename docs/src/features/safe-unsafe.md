# Safe vs Unsafe

Accidentally hitting a `DELETE` endpoint on Production is a nightmare for any developer. xrest introduces a concept of **Unsafe Environments** to mitigate this risk.

## Defining Safety

In your `environments.yaml`, each environment has an `isUnsafe` property. By default:
- `DEV` and `STAGE` are considered **Safe**.
- `PROD` is considered **Unsafe**.

## Visual Indicators

When you switch to an Unsafe environment:
1. **Red Accents**: The environment selector and the "Send" button will turn red.
2. **StatusBar Alert**: A clear indicator shows that you are operating on a production-grade environment.

## Execution Guard

When you click "Send" while in an Unsafe environment, xrest does not execute the request immediately. Instead:
1. A **Confirmation Dialog** appears.
2. You must explicitly confirm that you want to proceed with the request.
3. (Optional) A timeout might automatically dismiss the request if no action is taken.

## Customizing Safety

You can toggle the safety status of any environment in the Service Settings. For example, if you have a performance testing environment that is destructive, you can mark it as `isUnsafe: true` to enable these protections.

[Screenshot of Red Send Button in PROD]
