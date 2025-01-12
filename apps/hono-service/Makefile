# Define variables for commands and options
BUN = bun
PORT = 8105

# Default target (build the project)
all: build

# Build the project (compile the Rust code)
build:
	$(BUN) build ./src/index.ts --outdir ./dist --target node

# Clean the project (remove target directory)
clean:
	$rm -rf ./dist

start:
	$(BUN) run ./dist/index.js

dev:
	$(BUN) run --hot src/index.ts

.PHONY: all build run clean test fmt lint release check version start
