# Contributing to KiloCode for Zed

Thank you for your interest in contributing to KiloCode! This document provides guidelines and instructions for contributing.

## Code of Conduct

This project adheres to a code of conduct. By participating, you are expected to uphold this code.

- Be respectful and inclusive
- Provide constructive feedback
- Focus on what is best for the community
- Show empathy towards other community members

## How to Contribute

### Reporting Bugs

Before creating bug reports, please check the existing issues to avoid duplicates.

When creating a bug report, include:

- **Title**: Clear and descriptive
- **Description**: Detailed explanation of the problem
- **Steps to reproduce**: Step-by-step instructions
- **Expected behavior**: What you expected to happen
- **Actual behavior**: What actually happened
- **Environment**:
  - Zed version
  - KiloCode version
  - Operating system
- **Screenshots**: If applicable
- **Additional context**: Any other relevant information

### Suggesting Enhancements

Enhancement suggestions are welcome! When suggesting an enhancement:

- Use a clear and descriptive title
- Provide a detailed description of the enhancement
- Explain why this enhancement would be useful
- Provide examples of how the enhancement would be used
- Consider if this enhancement fits the project's goals

### Pull Requests

#### Before Submitting

1. **Check for existing PRs**: Search for similar pull requests
2. **Discuss changes**: Open an issue first for major changes
3. **Follow coding standards**: See below
4. **Write tests**: Add tests for new functionality
5. **Update documentation**: Update README, CHANGELOG, etc.

#### Submitting a PR

1. Fork the repository
2. Create a branch for your changes:
   ```bash
   git checkout -b feature/your-feature-name
   ```
3. Make your changes
4. Commit your changes:
   ```bash
   git commit -m "feat: add your feature"
   ```
5. Push to your fork:
   ```bash
   git push origin feature/your-feature-name
   ```
6. Create a pull request

#### PR Description

Include in your PR description:

- **Title**: Clear and descriptive (use conventional commits)
- **Description**: What changes were made and why
- **Related issues**: Link to related issues
- **Breaking changes**: Note any breaking changes
- **Screenshots**: If applicable

## Coding Standards

### Rust Code Style

Follow the official Rust style guidelines:

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt --check

# Run linter
cargo clippy -- -D warnings
```

### Conventional Commits

Use conventional commit messages:

- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation changes
- `style:` - Code style changes (formatting)
- `refactor:` - Code refactoring
- `test:` - Adding or updating tests
- `chore:` - Maintenance tasks
- `security:` - Security fixes

Examples:
```
feat: add streaming response support
fix: handle empty API responses
docs: update installation instructions
security: add input validation for user prompts
```

### Code Organization

- Keep functions focused and small
- Use descriptive names
- Add comments for complex logic
- Document public APIs

### Security Considerations

- Never commit API keys or credentials
- Validate all user inputs
- Sanitize all outputs
- Use HTTPS for all network requests
- Follow security best practices

## Development Setup

### Prerequisites

- Rust 1.70 or later
- Git
- Zed editor (for testing)

### Setting Up Development Environment

```bash
# Clone the repository
git clone https://github.com/YOUR_USERNAME/kilocode-zed.git
cd kilocode-zed

# Install dependencies
cargo build

# Run tests
cargo test

# Format code
cargo fmt

# Run linter
cargo clippy
```

### Building the Extension

```bash
# Debug build
cargo build

# Release build
cargo build --release
```

The compiled extension will be in `target/release/libkilocode.so` (Linux) or `target/release/libkilocode.dylib` (macOS).

### Testing in Zed

1. Build the extension:
   ```bash
   cargo build --release
   ```

2. Copy the extension to Zed's extensions directory:
   - **macOS**: `~/.config/zed/extensions/`
   - **Linux**: `~/.config/zed/extensions/`
   - **Windows**: `%APPDATA%\Zed\extensions\`

3. Restart Zed

4. Test the slash commands

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name
```

### Writing Tests

Add tests to the appropriate module:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_input() {
        assert!(KiloCodeExtension::validate_input("valid input").is_ok());
        assert!(KiloCodeExtension::validate_input("a".repeat(100001).as_str()).is_err());
    }
}
```

## Documentation

### Updating Documentation

When making changes:

1. Update `README.md` for user-facing changes
2. Update `CHANGELOG.md` for version changes
3. Update inline code comments
4. Update this `CONTRIBUTING.md` if needed

### Adding New Features

When adding new features:

1. Document the feature in `README.md`
2. Add usage examples
3. Update `CHANGELOG.md`
4. Consider adding tests

## Release Process

Releases are automated via GitHub Actions:

1. Update version in `extension.toml` and `Cargo.toml`
2. Update `CHANGELOG.md`
3. Commit changes
4. Create and push a version tag:
   ```bash
   git tag -a v0.2.0 -m "Release v0.2.0"
   git push origin v0.2.0
   ```
5. GitHub Actions will create the release automatically

## Getting Help

- **Issues**: Use GitHub issues for bugs and feature requests
- **Discussions**: Use GitHub Discussions for questions and ideas
- **Documentation**: Check the README and wiki

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

## Recognition

Contributors will be recognized in:
- README.md
- Release notes
- Contributors section

Thank you for contributing to KiloCode! 🎉
