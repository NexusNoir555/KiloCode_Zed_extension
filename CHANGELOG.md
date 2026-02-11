# Changelog

All notable changes to KiloCode for Zed will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Planned
- Add support for streaming responses
- Add conversation history
- Add custom prompts
- Add multi-language support

## [0.1.0] - 2024-02-11

### Added
- Initial release of KiloCode for Zed
- 6 slash commands:
  - `/kc` - General AI chat
  - `/kc-explain` - Explain selected code
  - `/kc-generate` - Generate code from description
  - `/kc-refactor` - Suggest refactoring
  - `/kc-fix` - Detect and fix bugs
  - `/kc-docs` - Generate documentation
- Configurable API endpoint and credentials
- Secure HTTPS-only communication
- Input validation and output sanitization
- GitHub Actions for CI/CD
- Automatic release creation
- Comprehensive documentation

### Security
- HTTPS-only API communication
- No code execution
- Input validation (max 100,000 characters)
- Output sanitization (removes shell code blocks)
- User-controlled credentials

[Unreleased]: https://github.com/YOUR_USERNAME/kilocode-zed/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/YOUR_USERNAME/kilocode-zed/releases/tag/v0.1.0
