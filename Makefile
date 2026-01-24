.PHONY: help build install sign package release clean test run

help:
	@echo "karabiner-rcmd-binder - Makefile commands"
	@echo ""
	@echo "Development:"
	@echo "  make build     - Build release binary"
	@echo "  make run       - Run the TUI"
	@echo "  make test      - Run tests"
	@echo ""
	@echo "Installation:"
	@echo "  make install   - Install to ~/.local/bin"
	@echo ""
	@echo "Distribution:"
	@echo "  make sign      - Sign binary with Apple Developer cert"
	@echo "  make package   - Create signed, distributable tarball"
	@echo "  make release VERSION=X.X.X - Full release automation"
	@echo ""
	@echo "Cleanup:"
	@echo "  make clean     - Remove build artifacts"

build:
	cargo build --release

run:
	./run.sh

test:
	cargo test

install:
	./install.sh

sign:
	./scripts/sign.sh

package: build sign
	./scripts/package.sh

release:
	@if [ -z "$(VERSION)" ]; then \
		echo "❌ Error: VERSION is required"; \
		echo "Usage: make release VERSION=0.2.2"; \
		echo "Example: make release VERSION=0.2.1-beta"; \
		exit 1; \
	fi
	./scripts/release.sh $(VERSION)

clean:
	cargo clean
	rm -rf dist
	rm -rf target

.DEFAULT_GOAL := help
