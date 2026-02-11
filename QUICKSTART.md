# Quick Start Guide

Get KiloCode up and running in Zed in under 5 minutes!

## Installation

### Option 1: From Zed Extensions Registry (Easiest)

1. Open Zed
2. Press `Cmd+Shift+P` (macOS) or `Ctrl+Shift+P` (Linux/Windows)
3. Type "Extensions: Install"
4. Search for "KiloCode"
5. Click "Install"

### Option 2: Manual Installation

1. Download the latest release from [GitHub Releases](https://github.com/YOUR_USERNAME/kilocode-zed/releases)
2. Extract the archive
3. Move the extension to your Zed extensions directory:
   - **macOS**: `~/.config/zed/extensions/`
   - **Linux**: `~/.config/zed/extensions/`
   - **Windows**: `%APPDATA%\Zed\extensions\`
4. Restart Zed

## Configuration

You need to configure your API credentials before using KiloCode.

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

**Required Settings:**
- `api_key`: Your API key (required)

**Optional Settings:**
- `api_endpoint`: Your API endpoint (default: `https://api.kilocode.ai/v1`)
- `model`: AI model to use (default: `gpt-4`)

## Usage

### Basic Usage

1. Open a file in Zed
2. Press `Cmd+Shift+P` (macOS) or `Ctrl+Shift+P` (Linux/Windows)
3. Type a slash command (see below)
4. Press Enter

### Slash Commands

| Command | Description | Example |
|---------|-------------|---------|
| `/kc` | General AI chat | `/kc How do I implement a binary search?` |
| `/kc-explain` | Explain selected code | Select code → `/kc-explain` |
| `/kc-generate` | Generate code | `/kc-generate Create a function to validate emails` |
| `/kc-refactor` | Suggest refactoring | Select code → `/kc-refactor` |
| `/kc-fix` | Detect and fix bugs | Select code → `/kc-fix` |
| `/kc-docs` | Generate documentation | Select code → `/kc-docs` |

### Examples

**Ask a question:**
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

## Next Steps

- Read the full [README.md](README.md) for detailed documentation
- Check [SECURITY.md](SECURITY.md) for security information
- See [CONTRIBUTING.md](CONTRIBUTING.md) if you want to contribute

## Support

- 📖 [Documentation](https://github.com/YOUR_USERNAME/kilocode-zed/wiki)
- 🐛 [Report Issues](https://github.com/YOUR_USERNAME/kilocode-zed/issues)
- 💬 [Discussions](https://github.com/YOUR_USERNAME/kilocode-zed/discussions)
