# Installation

xrest is cross-platform and available for macOS, Windows, and Linux.

## Download

Currently, xrest is in a pre-release state. You can find the latest builds on our [GitHub Releases](https://github.com/xrest-labs/xrest/releases) page.

### macOS
- Download the `.dmg` file.
- Drag xrest to your `Applications` folder.
- *Note: You might need to right-click and select "Open" the first time to bypass unidentified developer warnings.*

### Windows
- Download the `.msi` or `.exe` installer.
- Run the installer and follow the prompts.

### Linux
- We provide `.AppImage` and `.deb` packages for most distributions.

## Building from Source

If you prefer to build xrest yourself, you will need:
- **Rust** (latest stable)
- **Node.js** (v18+)
- **pnpm**

```bash
# Clone the repository
git clone https://github.com/xrest-labs/xrest.git

# Install dependencies
pnpm install

# Run in development mode
pnpm tauri dev

# Build production version
pnpm tauri build
```

## Storage Locations

xrest stores its global configuration and tab states in the following directories:

| OS | Path |
| --- | --- |
| **macOS** | `~/Library/Application Support/io.ak.xrest/` |
| **Windows**| `%APPDATA%/io.ak.xrest/` |
| **Linux**  | `~/.config/io.ak.xrest/` |

Each service you create will store its endpoints and environments in the directory you choose during setup.
