# Quick Start Guide for WhatsApp Lite

Get started with **WhatsApp Lite** - the lightweight, privacy-focused WhatsApp desktop client in under 2 minutes!

## 🚀 Installation Methods

### Method 1: Download Pre-built Installer (Recommended)

1. **Download**: Go to [Releases](https://github.com/bittu-the-coder/whatsapp-lite/releases) and download the latest `.exe` file

2. **Install**:
   - Double-click the downloaded `.exe` file
   - Follow the installation wizard
   - No administrator rights required!

3. **Launch**:
   - Find "WhatsApp Lite" in Start Menu
   - Or use desktop shortcut (if created)

4. **First Use**:
   - Scan QR code with your phone
   - You're ready to chat!

**Installation Size**: ~5 MB
**Installation Time**: <30 seconds

---

### Method 2: Build from Source (Developers)

#### Prerequisites

- [Node.js](https://nodejs.org/) v16 or later
- [Rust](https://www.rust-lang.org/tools/install)
- Windows 10/11

#### Steps

```bash
# 1. Clone repository
git clone https://github.com/bittu-the-coder/whatsapp-lite.git
cd whatsapp-lite

# 2. Install dependencies
npm install

# 3. Run in development mode (optional)
npm run tauri dev

# 4. Build production installer
npm run tauri build
```

**Output Location**: `src-tauri/target/release/bundle/nsis/`

**Build Time**: 5-10 minutes (first build)

---

## 🎯 First Launch Checklist

After installation:

- ✅ Open WhatsApp Lite from Start Menu
- ✅ Scan QR code with WhatsApp mobile app
- ✅ Check system tray icon appears (bottom-right)
- ✅ Test minimize to tray (click X button)
- ✅ Verify notifications are working

---

## ⚙️ Configuration

WhatsApp Lite works out-of-the-box with zero configuration needed!

### System Tray Behavior

- **Close (X) Button**: Minimizes to system tray
- **Minimize Button**: Sends to taskbar
- **Tray Icon Right-Click**: Show menu options

### UI Customization

The following are automatically hidden:

- Communities tab
- Channels tab
- Meta AI tab
- Download/promotion banners

---

## 🔧 Troubleshooting

### App won't start

```bash
# Clear cache
del %APPDATA%\whatsapp-lite

# Reinstall the application
```

### No system tray icon

- Check Windows Settings > System > Notifications & actions
- Ensure "WhatsApp Lite" is allowed

### Blank screen on startup

- Check internet connection
- Try refreshing (Ctrl+R)
- Reinstall if issue persists

---

## 🆘 Getting Help

- 📖 Read [FAQ](../README.md#-troubleshooting--faq)
- 🐛 [Report Bug](https://github.com/bittu-the-coder/whatsapp-lite/issues)
- 💬 [Ask Question](https://github.com/bittu-the-coder/whatsapp-lite/discussions)

---

## 🎉 What's Next?

- ⭐ **Star this repo** to show support
- 🔔 **Watch** for updates
- 📢 **Share** with friends who need lightweight WhatsApp
- 🤝 **Contribute** improvements

---

**Enjoy your lightweight WhatsApp experience!** 🚀
