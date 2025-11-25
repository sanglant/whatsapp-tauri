# WhatsApp Linux

A native desktop wrapper for WhatsApp Web built with Tauri, providing a seamless WhatsApp experience on Linux with system-level integration.

## Features

- **Native Desktop Application** - Standalone WhatsApp client for Linux
- **System Tray Integration** - Minimize to tray and quick access from system tray icon
- **Desktop Notifications** - Full notification support using Tauri's notification API
- **External Link Handling** - Opens external links in your default browser automatically
- **Minimize on Close** - Window minimizes to tray instead of closing
- **Lightweight** - Built with Tauri for minimal resource usage
- **Modern UI** - Clean interface with WhatsApp Web's familiar design

## Prerequisites

Before building this application, ensure you have the following installed:

- **Rust** (latest stable) - [Install Rust](https://www.rust-lang.org/tools/install)
- **Node.js** (19 or higher) - [Install Node.js](https://nodejs.org/)
- **npm** or **pnpm** - Comes with Node.js
- **System Dependencies** (Linux):

  ```bash
  # Debian/Ubuntu
  sudo apt install libwebkit2gtk-4.1-dev \
    build-essential \
    curl \
    wget \
    file \
    libxdo-dev \
    libssl-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev

  # Fedora
  sudo dnf install webkit2gtk4.1-devel \
    openssl-devel \
    curl \
    wget \
    file \
    libappindicator-gtk3-devel \
    librsvg2-devel

  # Arch Linux
  sudo pacman -S webkit2gtk-4.1 \
    base-devel \
    curl \
    wget \
    file \
    openssl \
    appmenu-gtk-module \
    gtk3 \
    libappindicator-gtk3 \
    librsvg
  ```

## Installation

### From Source

1. **Clone the repository**

   ```bash
   git clone <repository-url>
   cd whatsapp-linux
   ```

2. **Install dependencies**

   ```bash
   npm install
   ```

3. **Build the application**

   ```bash
   npm run tauri build
   ```

4. **Install the built package**

   The built packages will be available in `src-tauri/target/release/bundle/`:

   - **Debian/Ubuntu**: `.deb` package

     ```bash
     sudo dpkg -i src-tauri/target/release/bundle/deb/*.deb
     ```

   - **AppImage**: Portable executable
     ```bash
     chmod +x src-tauri/target/release/bundle/appimage/*.AppImage
     ./src-tauri/target/release/bundle/appimage/*.AppImage
     ```

## Development

To run the application in development mode:

```bash
npm run tauri dev
```

This will start the development server with hot-reload enabled.

## Project Structure

```
whatsapp-linux/
├── src/                    # Frontend assets
│   ├── index.html         # Main HTML file
│   ├── main.js            # JavaScript logic
│   ├── styles.css         # Styling
│   └── assets/            # Static assets
├── src-tauri/             # Tauri backend
│   ├── src/
│   │   └── lib.rs        # Main Rust logic
│   ├── Cargo.toml        # Rust dependencies
│   ├── tauri.conf.json   # Tauri configuration
│   ├── capabilities/      # Permission configuration
│   └── icons/            # Application icons
├── package.json           # Node.js dependencies
└── README.md             # This file
```

## Technical Details

### Key Technologies

- **[Tauri](https://tauri.app/)** - Framework for building desktop applications
- **Rust** - Backend logic and system integration
- **Vanilla JavaScript** - Frontend scripting
- **WebKit2GTK** - Web rendering engine on Linux

### Core Features Implementation

#### Notification Proxy

The application intercepts WhatsApp Web's notification requests and proxies them through Tauri's native notification system, ensuring notifications work correctly on Linux.

#### External Link Handler

External links are intercepted and opened in the system's default browser using Tauri's shell plugin, while keeping WhatsApp links within the application.

#### System Tray

The application includes a system tray icon with:

- Click to show/hide window
- Context menu with Show/Hide and Quit options
- Minimize to tray on window close

### Tauri Plugins Used

- `tauri-plugin-notification` - Desktop notifications
- `tauri-plugin-shell` - Opening external links
- `tauri-plugin-opener` - File/URL opening capabilities

## Configuration

The application configuration can be modified in [`src-tauri/tauri.conf.json`](src-tauri/tauri.conf.json):

- **Window size**: Currently set to 1200x800
- **Application ID**: `com.whatsapp.wrapper`
- **Product name**: `WhatsApp Linux`

## Troubleshooting

### Notifications not working

Ensure the notification plugin is properly initialized and permissions are granted. Check system notification settings.

### External links not opening

Verify that you have a default browser set in your system settings.

### Build fails

- Ensure all system dependencies are installed
- Update Rust: `rustup update`
- Clear build cache: `cargo clean` in the `src-tauri` directory

## License

This project is a desktop wrapper for WhatsApp Web and is not affiliated with, endorsed by, or connected to WhatsApp Inc. or Meta Platforms, Inc.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
