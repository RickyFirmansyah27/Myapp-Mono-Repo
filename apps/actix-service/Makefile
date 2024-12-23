# Define variables for commands and options
CARGO = cargo
TARGET = target/debug/actix-service.exe
PORT = 8101

# Default target (build the project)
all: build

# Build the project (compile the Rust code)
build:
	$(CARGO) build

# Run the project (start the server)
run: build
	$(CARGO) run

# Clean the project (remove target directory)
clean:
	$(CARGO) clean

# Run tests
test:
	$(CARGO) test

# Format the code with rustfmt
fmt:
	$(CARGO) fmt

# Lint the code with clippy
lint:
	$(CARGO) clippy

# Run the project in release mode (optimized)
release:
	$(CARGO) build --release

# Check the project for potential issues (with `cargo check`)
check:
	$(CARGO) check

# Show the version of cargo and Rust toolchain
version:
	$(CARGO) --version

# Run the project and log the server startup
start:
	$(CARGO) run -- --port $(PORT)

.PHONY: all build run clean test fmt lint release check version start
