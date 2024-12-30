# Variables
BUN=bun
DIST_DIR=dist

# Targets
run:
	$(BUN) run nodemon ./src/index.js

build:
	@mkdir -p $(DIST_DIR)
	$(BUN) build ./src/index --outdir $(DIST_DIR) --target node

test:
	$(BUN) test

clean:
	@if exist $(DIST_DIR) rmdir /s /q $(DIST_DIR)

help:
	@echo "Available targets:"
	@echo "  run    - Run the Express application with hot-reloading"
	@echo "  build  - Build the application (e.g., transpile or bundle)"
	@echo "  test   - Run tests"
	@echo "  clean  - Clean the build artifacts"
