# Makefile for KiloCode Zed Extension

.PHONY: all build test fmt clippy clean release install uninstall help version-bump-major version-bump-minor version-bump-patch

# Variables
CARGO = cargo
EXTENSION_NAME = kilocode
VERSION = $(shell grep '^version = ' Cargo.toml | head -1 | cut -d '"' -f 2)
TARGET_DIR = target/release
LIB_NAME = lib$(EXTENSION_NAME)

# Detect OS
UNAME_S := $(shell uname -s)
ifeq ($(UNAME_S),Darwin)
    LIB_EXT = dylib
    PLATFORM = macos
else ifeq ($(UNAME_S),Linux)
    LIB_EXT = so
    PLATFORM = linux
else
    LIB_EXT = dll
    PLATFORM = windows
endif

# Default target
all: build

# Build the extension
build:
	@echo "Building $(EXTENSION_NAME) v$(VERSION)..."
	$(CARGO) build --release
	@echo "Build complete: $(TARGET_DIR)/$(LIB_NAME).$(LIB_EXT)"

# Run tests
test:
	@echo "Running tests..."
	$(CARGO) test

# Format code
fmt:
	@echo "Formatting code..."
	$(CARGO) fmt

# Check formatting
fmt-check:
	@echo "Checking code formatting..."
	$(CARGO) fmt --check

# Run clippy
clippy:
	@echo "Running clippy..."
	$(CARGO) clippy -- -D warnings

# Clean build artifacts
clean:
	@echo "Cleaning build artifacts..."
	$(CARGO) clean
	rm -f *.tar.gz *.zip *.sha256

# Create release archive
release: build
	@echo "Creating release archive..."
	@mkdir -p release
	@cp -r extension.toml Cargo.toml src README.md LICENSE release/
	@cp $(TARGET_DIR)/$(LIB_NAME).$(LIB_EXT) release/
	@cd release && tar -czf ../$(EXTENSION_NAME)-$(VERSION)-$(PLATFORM).tar.gz .
	@rm -rf release
	@echo "Release archive created: $(EXTENSION_NAME)-$(VERSION)-$(PLATFORM).tar.gz"
	@sha256sum $(EXTENSION_NAME)-$(VERSION)-$(PLATFORM).tar.gz > $(EXTENSION_NAME)-$(VERSION)-$(PLATFORM).tar.gz.sha256
	@echo "Checksum created: $(EXTENSION_NAME)-$(VERSION)-$(PLATFORM).tar.gz.sha256"

# Install extension locally
install: build
	@echo "Installing $(EXTENSION_NAME)..."
	@mkdir -p ~/.config/zed/extensions/$(EXTENSION_NAME)
	@cp -r extension.toml Cargo.toml src ~/.config/zed/extensions/$(EXTENSION_NAME)/
	@cp $(TARGET_DIR)/$(LIB_NAME).$(LIB_EXT) ~/.config/zed/extensions/$(EXTENSION_NAME)/
	@echo "Extension installed to ~/.config/zed/extensions/$(EXTENSION_NAME)/"

# Uninstall extension locally
uninstall:
	@echo "Uninstalling $(EXTENSION_NAME)..."
	@rm -rf ~/.config/zed/extensions/$(EXTENSION_NAME)
	@echo "Extension uninstalled"

# Run all checks
check: fmt-check clippy test
	@echo "All checks passed!"

# Bump major version
version-bump-major:
	@echo "Bumping major version..."
	@$(eval NEW_VERSION = $(shell echo $(VERSION) | awk -F. '{print $$1+1".0.0"}'))
	@sed -i 's/^version = "$(VERSION)"/version = "$(NEW_VERSION)"/' Cargo.toml
	@sed -i 's/^version = "$(VERSION)"/version = "$(NEW_VERSION)"/' extension.toml
	@echo "Version bumped from $(VERSION) to $(NEW_VERSION)"

# Bump minor version
version-bump-minor:
	@echo "Bumping minor version..."
	@$(eval NEW_VERSION = $(shell echo $(VERSION) | awk -F. '{print $$1"."$$2+1".0"}'))
	@sed -i 's/^version = "$(VERSION)"/version = "$(NEW_VERSION)"/' Cargo.toml
	@sed -i 's/^version = "$(VERSION)"/version = "$(NEW_VERSION)"/' extension.toml
	@echo "Version bumped from $(VERSION) to $(NEW_VERSION)"

# Bump patch version
version-bump-patch:
	@echo "Bumping patch version..."
	@$(eval NEW_VERSION = $(shell echo $(VERSION) | awk -F. '{print $$1"."$$2"."$$3+1}'))
	@sed -i 's/^version = "$(VERSION)"/version = "$(NEW_VERSION)"/' Cargo.toml
	@sed -i 's/^version = "$(VERSION)"/version = "$(NEW_VERSION)"/' extension.toml
	@echo "Version bumped from $(VERSION) to $(NEW_VERSION)"

# Create git tag for release
tag-release:
	@echo "Creating git tag v$(VERSION)..."
	@git tag -a v$(VERSION) -m "Release v$(VERSION)"
	@echo "Tag v$(VERSION) created. Run 'git push origin v$(VERSION)' to push."

# Help
help:
	@echo "KiloCode Zed Extension - Makefile"
	@echo ""
	@echo "Targets:"
	@echo "  all              - Build the extension (default)"
	@echo "  build            - Build the extension"
	@echo "  test             - Run tests"
	@echo "  fmt              - Format code"
	@echo "  fmt-check        - Check code formatting"
	@echo "  clippy           - Run clippy linter"
	@echo "  clean            - Clean build artifacts"
	@echo "  release          - Create release archive"
	@echo "  install          - Install extension locally"
	@echo "  uninstall        - Uninstall extension locally"
	@echo "  check            - Run all checks (fmt, clippy, test)"
	@echo "  version-bump-major   - Bump major version"
	@echo "  version-bump-minor   - Bump minor version"
	@echo "  version-bump-patch   - Bump patch version"
	@echo "  tag-release      - Create git tag for release"
	@echo "  help             - Show this help message"
