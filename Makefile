.PHONY: help build install clean test run

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
	@echo "Cleanup:"
	@echo "  make clean     - Remove build artifacts"
	@echo ""
	@echo "Release: handled by the /release rust flow (universal binary + Homebrew"
	@echo "         bottle + GitHub release + tap). There is no local 'make release'."

build:
	cargo build --release

run:
	./run.sh

test:
	cargo test

install:
	./install.sh

clean:
	cargo clean
	rm -rf dist
	rm -rf target

.DEFAULT_GOAL := help
