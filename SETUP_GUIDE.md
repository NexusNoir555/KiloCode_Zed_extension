# GitHub Repository Setup Guide

This guide will help you set up the GitHub repository for the KiloCode Zed extension.

## Prerequisites

- GitHub account
- Git installed
- GitHub CLI (`gh`) installed (optional but recommended)

## Step 1: Create GitHub Repository

### Option A: Using GitHub CLI (Recommended)

```bash
# Navigate to the extension directory
cd kilocode-zed-extension

# Create a new public repository
gh repo create kilocode-zed --public --source=. --remote=origin --push

# If prompted, authenticate with GitHub
gh auth login
```

### Option B: Using GitHub Website

1. Go to https://github.com/new
2. Repository name: `kilocode-zed`
3. Set to **Public**
4. **Do not** initialize with README, .gitignore, or license (we already have these)
5. Click "Create repository"
6. Follow the instructions to push your existing repository

```bash
cd kilocode-zed-extension
git remote add origin https://github.com/YOUR_USERNAME/kilocode-zed.git
git branch -M main
git push -u origin main
```

## Step 2: Update Repository URLs

After creating the repository, update the repository URLs in the following files:

### 1. Update `extension.toml`

Replace `YOUR_USERNAME` with your actual GitHub username:

```toml
repository = "https://github.com/YOUR_USERNAME/kilocode-zed"
```

### 2. Update `Cargo.toml`

Replace `YOUR_USERNAME` with your actual GitHub username:

```toml
repository = "https://github.com/YOUR_USERNAME/kilocode-zed"
```

### 3. Update `README.md`

Replace all instances of `YOUR_USERNAME` with your actual GitHub username.

## Step 3: Commit and Push Changes

```bash
cd kilocode-zed-extension

# Add all files
git add .

# Commit changes
git commit -m "Update repository URLs"

# Push to GitHub
git push
```

## Step 4: Configure GitHub Repository

### Enable GitHub Actions

1. Go to your repository on GitHub
2. Click "Settings" → "Actions" → "General"
3. Under "Actions permissions", select "Allow all actions and reusable workflows"
4. Click "Save"

### Enable Issues and Discussions

1. Go to "Settings" → "General"
2. Under "Features", ensure "Issues" and "Discussions" are checked
3. Click "Save changes"

### Add Repository Topics

1. Go to your repository page
2. Click the gear icon next to "About"
3. Add topics: `zed`, `zed-editor`, `extension`, `ai`, `coding-assistant`, `rust`
4. Click "Save topics"

## Step 5: Create First Release

### Using GitHub CLI

```bash
# Create and push a version tag
git tag -a v0.1.0 -m "Initial release"
git push origin v0.1.0

# Create a release
gh release create v0.1.0 \
  --title "KiloCode v0.1.0" \
  --notes "Initial release of KiloCode for Zed

Features:
- 6 slash commands for AI assistance
- Configurable API endpoint and credentials
- Secure HTTPS-only communication
- Input validation and output sanitization

See README.md for installation and usage instructions."
```

### Using GitHub Website

1. Go to your repository
2. Click "Releases" → "Create a new release"
3. Tag: `v0.1.0`
4. Release title: `KiloCode v0.1.0`
5. Description:
   ```markdown
   Initial release of KiloCode for Zed

   Features:
   - 6 slash commands for AI assistance
   - Configurable API endpoint and credentials
   - Secure HTTPS-only communication
   - Input validation and output sanitization

   See README.md for installation and usage instructions.
   ```
6. Click "Publish release"

The GitHub Actions workflow will automatically build and upload the extension files to the release.

## Step 6: Submit to Zed Extensions Registry

To make your extension available in Zed's official extensions registry:

1. Fork the [zed-industries/extensions](https://github.com/zed-industries/extensions) repository
2. Clone your fork:
   ```bash
   git clone https://github.com/YOUR_USERNAME/extensions.git
   cd extensions
   ```

3. Add your extension as a submodule:
   ```bash
   git submodule add https://github.com/YOUR_USERNAME/kilocode-zed.git extensions/kilocode
   ```

4. Update `extensions.toml`:
   ```toml
   [extensions.kilocode]
   name = "KiloCode"
   description = "AI-powered coding assistant for Zed"
   version = "0.1.0"
   path = "extensions/kilocode"
   ```

5. Commit and push:
   ```bash
   git add extensions.toml extensions/kilocode
   git commit -m "Add KiloCode extension"
   git push origin main
   ```

6. Create a pull request to zed-industries/extensions

## Step 7: Verify Everything Works

1. **Check CI/CD**: Go to "Actions" tab and ensure workflows are passing
2. **Check Release**: Go to "Releases" and verify the release has the extension files
3. **Test Installation**: Try installing the extension in Zed

## Troubleshooting

### GitHub CLI not authenticated

```bash
gh auth login
```

Follow the prompts to authenticate with GitHub.

### Push fails with "remote already exists"

```bash
git remote set-url origin https://github.com/YOUR_USERNAME/kilocode-zed.git
git push -u origin main
```

### GitHub Actions not running

1. Check that Actions are enabled in repository settings
2. Verify the workflow files are in `.github/workflows/`
3. Check the Actions tab for error messages

### Release not created automatically

1. Ensure the tag follows semantic versioning (e.g., `v0.1.0`)
2. Check that the tag was pushed: `git push origin v0.1.0`
3. Verify the release workflow file exists and is valid

## Next Steps

After completing this guide:

- 📝 Update the [CHANGELOG.md](CHANGELOG.md) with your release notes
- 🐛 Set up issue templates for bug reports and feature requests
- 📖 Add more documentation to the wiki
- 🎨 Consider adding a logo or icon for the extension
- 🔄 Set up automated testing

## Support

If you encounter issues:

- Check the [GitHub Actions documentation](https://docs.github.com/en/actions)
- Review the [Zed extension documentation](https://github.com/zed-industries/zed)
- Open an issue in this repository
