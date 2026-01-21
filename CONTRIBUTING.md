# Contributing to WhatsApp Lite

Thank you for considering contributing to **WhatsApp Lite** - the lightweight, privacy-focused WhatsApp desktop client! We welcome all contributions that help improve this open-source project.

## 🎯 How to Contribute

### Reporting Bugs

1. Check if the bug has already been reported in [Issues](https://github.com/bittu-the-coder/whatsapp-lite/issues)
2. Create a new issue with:
   - Clear, descriptive title
   - Steps to reproduce
   - Expected vs actual behavior
   - Screenshots if applicable
   - System information (Windows version, app version)

### Suggesting Features

1. Check [existing feature requests](https://github.com/bittu-the-coder/whatsapp-lite/issues?q=is%3Aissue+is%3Aopen+label%3Aenhancement)
2. Create a new issue with:
   - Clear description of the feature
   - Use cases and benefits
   - Possible implementation approach

### Code Contributions

#### Setup Development Environment

```bash
# Fork the repository on GitHub
# Clone your fork
git clone https://github.com/YOUR_USERNAME/whatsapp-lite.git
cd whatsapp-lite

# Add upstream remote
git remote add upstream https://github.com/bittu-the-coder/whatsapp-lite.git

# Install dependencies
npm install

# Run in development mode
npm run tauri dev
```

#### Making Changes

1. Create a feature branch: `git checkout -b feature/your-feature-name`
2. Make your changes following our coding standards
3. Test your changes thoroughly
4. Commit with clear, descriptive messages
5. Push to your fork: `git push origin feature/your-feature-name`
6. Open a Pull Request

#### Pull Request Guidelines

- Reference any related issues
- Provide clear description of changes
- Include screenshots for UI changes
- Ensure code builds without errors
- Update documentation if needed

## 💻 Development Guidelines

### Code Style

- **Rust**: Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- **JavaScript**: Use ES6+ features, clear variable names
- **Comments**: Explain "why", not "what"

### Commit Messages

```
feat: Add system notification preferences
fix: Resolve system tray icon not showing on startup
docs: Update installation instructions
style: Format code according to prettier config
refactor: Improve window management logic
test: Add unit tests for tray menu
```

### Testing

- Test on Windows 10 and 11
- Verify system tray functionality
- Check UI responsiveness
- Ensure no performance regressions

## 📖 Resources

- [Tauri Documentation](https://tauri.app/v2/guides/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Project Architecture](docs/ARCHITECTURE.md) (if available)

## 🤝 Code of Conduct

- Be respectful and inclusive
- Focus on constructive feedback
- Help others in the community
- Maintain professional communication

## 📝 License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

**Questions?** Open a [GitHub Discussion](https://github.com/bittu-the-coder/whatsapp-lite/discussions) or comment on an issue!
