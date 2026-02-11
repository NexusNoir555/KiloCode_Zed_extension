# KiloCode for Zed

AI-powered coding assistant for Zed editor. Ask questions, explain code, generate code, refactor, fix bugs, and generate documentation.

## Features

- **6 Slash Commands** for different AI tasks
- **Secure** - HTTPS-only communication, no code execution
- **Configurable** - Use your own API endpoint and credentials
- **Open Source** - Fully transparent and auditable

## Installation

### From Zed Extensions Registry (Recommended)

1. Open Zed
2. Press `Cmd+Shift+P` (macOS) or `Ctrl+Shift+P` (Linux/Windows)
3. Type "Extensions: Install"
4. Search for "KiloCode"
5. Click Install

### Manual Installation

1. Download the latest release from [GitHub Releases](https://github.com/NexusNoir555/KiloCode_Zed_extension/releases)
2. Extract the archive
3. Move the extension to your Zed extensions directory:
   - **macOS**: `~/.config/zed/extensions/`
   - **Linux**: `~/.config/zed/extensions/`
   - **Windows**: `%APPDATA%\Zed\extensions\`
4. Restart Zed

## Configuration

Before using KiloCode, you need to configure your API credentials:

1. Open Zed Settings (`Cmd+,` or `Ctrl+,`)
2. Add the following to your settings:

```json
{
  "extensions": {
    "kilocode": {
      "api_endpoint": "https://your-api-endpoint.com/v1",
      "api_key": "your-api-key-here",
      "model": "gpt-4"
    }
  }
}
```

### Configuration Options

| Setting | Description | Default | Required |
|---------|-------------|---------|----------|
| `api_endpoint` | Your API endpoint URL | `https://api.kilocode.ai/v1` | No |
| `api_key` | Your API key | - | **Yes** |
| `model` | AI model to use | `gpt-4` | No |

## Usage

### Slash Commands

KiloCode provides 6 slash commands:

| Command | Description | Example |
|---------|-------------|---------|
| `/kc` | General AI chat | `/kc How do I implement a binary search?` |
| `/kc-explain` | Explain selected code | `/kc-explain` (with code selected) |
| `/kc-generate` | Generate code from description | `/kc-generate Create a function to validate email addresses` |
| `/kc-refactor` | Suggest refactoring | `/kc-refactor` (with code selected) |
| `/kc-fix` | Detect and fix bugs | `/kc-fix` (with code selected) |
| `/kc-docs` | Generate documentation | `/kc-docs` (with code selected) |

### Examples

**Ask a coding question:**
```
/kc What's the difference between let and const in JavaScript?
```

**Explain code:**
1. Select the code you want to explain
2. Type `/kc-explain` and press Enter

**Generate code:**
```
/kc-generate Create a Python function to sort a list of dictionaries by a specific key
```

**Refactor code:**
1. Select the code you want to refactor
2. Type `/kc-refactor` and press Enter

**Fix bugs:**
1. Select the code with bugs
2. Type `/kc-fix` and press Enter

**Generate documentation:**
1. Select the code you want to document
2. Type `/kc-docs` and press Enter

## Security

KiloCode is designed with security in mind:

- ✅ **HTTPS-only** communication with API endpoints
- ✅ **No code execution** - AI responses are text only
- ✅ **Input validation** - Prevents injection attacks
- ✅ **Output sanitization** - Removes potentially harmful content
- ✅ **Open source** - Fully auditable code
- ✅ **User-controlled credentials** - You provide your own API key

See [SECURITY.md](SECURITY.md) for more details.

## Troubleshooting

### "API key not configured" error

Make sure you've added your API key to Zed settings:

```json
{
  "extensions": {
    "kilocode": {
      "api_key": "your-api-key-here"
    }
  }
}
```

### "API request failed" error

Check that:
- Your `api_endpoint` is correct and accessible
- Your API key is valid
- You have internet connectivity
- The API endpoint supports HTTPS

### Extension not showing up

1. Make sure the extension is in the correct directory
2. Restart Zed
3. Check Zed's extension list (`Cmd+Shift+P` → "Extensions: List")

## Contributing

Contributions are welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

MIT License - see [LICENSE](LICENSE) for details.

## Support

- 📖 [Documentation](https://github.com/NexusNoir555/KiloCode_Zed_extension/wiki)
- 🐛 [Report Issues](https://github.com/NexusNoir555/KiloCode_Zed_extension/issues)
- 💬 [Discussions](https://github.com/NexusNoir555/KiloCode_Zed_extension/discussions)

## Acknowledgments

- Built with [Zed Extension API](https://github.com/zed-industries/zed)
- Inspired by the KiloCode VS Code extension
