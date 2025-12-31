# WhatsApp Lite (Unofficial)

A lightweight, clean, and optimized wrapper for WhatsApp Web, built with Tauri. This application provides a native-like experience on Windows with enhanced privacy and UI customizations.

## Features

*   **Clean Interface**: Automatically hides "Communities", "Channels", and "Meta AI" tabs to keep your workspace focused on chats.
*   **Banner Removal**: Removes the annoying "Get WhatsApp for Windows" and "Download" banners.
*   **System Tray Integration**: Minimizes to the system tray instead of closing, ensuring you never miss a message.
*   **Custom Scrollbar**: Features a sleek, minimal scrollbar that blends perfectly with the dark/light theme.
*   **Lightweight**: Built on Tauri v2, ensuring minimal resource usage compared to the official Electron-based app.
*   **Privacy Focused**: No tracking or data collection; it's just a direct wrapper around the official WhatsApp Web.

## Installation

### Download
You can download the latest `.exe` installer from the [Releases](https://github.com/bittu-the-coder/whatsapp-lite/releases) page. (Note: You will need to create this after building).

### Build from Source

Prerequisites:
*   [Node.js](https://nodejs.org/) (v16 or later)
*   [Rust](https://www.rust-lang.org/tools/install)
*   [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites)

1.  **Clone the repository**
    ```bash
    git clone https://github.com/bittu-the-coder/whatsapp-lite.git
    cd whatsapp-lite
    ```

2.  **Install dependencies**
    ```bash
    npm install
    ```

3.  **Run in Development Mode**
    ```bash
    npm run tauri dev
    ```

4.  **Build for Production**
    To create the `.exe` installer:
    ```bash
    npm run tauri build
    ```
    The output installer will be located in:
    `src-tauri/target/release/bundle/nsis/`

## Keyboard Shortcuts

*   **Close Window (X)**: Hides the application to the system tray.
*   **Tray Icon**: Right-click or Left-click to Show/Hide or Quit the application.

## Technologies Used

*   [Tauri](https://tauri.app/) - For building the lightweight desktop application.
*   [Rust](https://www.rust-lang.org/) - For the backend system tray and window management.
*   JavaScript - For DOM manipulation and UI cleanup.

## Disclaimer

This project is an **unofficial** wrapper and is not affiliated with, associated with, authorized by, endorsed by, or in any way officially connected with WhatsApp or Meta Platforms, Inc. The official WhatsApp website can be found at [https://www.whatsapp.com](https://www.whatsapp.com).
