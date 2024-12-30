# Variables
GO=go
DIST_DIR=dist
OUTPUT_BINARY=$(DIST_DIR)/index

# Targets

# run:
# 	$(GO) run main.go

build:
	@if not exist $(DIST_DIR) mkdir $(DIST_DIR)
	$(GO) build -o $(OUTPUT_BINARY)

run: build
	@./$(OUTPUT_BINARY)

test:
	$(GO) test ./...

clean:
	@if exist $(DIST_DIR) rmdir /s /q $(DIST_DIR)

help:
	@echo "Available targets:"
	@echo "  run    - Run the Go Fiber application"
	@echo "  build  - Build the binary executable in the dist directory"
	@echo "  test   - Run tests"
	@echo "  clean  - Clean the build artifacts"
	@echo "  start  - Build and run the application"

