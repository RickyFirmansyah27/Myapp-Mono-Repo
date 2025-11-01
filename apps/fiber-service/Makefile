# Variables
GO=go
DIST_DIR=dist
OUTPUT_BINARY=$(DIST_DIR)/fiber.exe

# Targets

# run:
# 	$(GO) run main.go

build:
	@mkdir -p $(DIST_DIR)
	$(GO) build -o $(OUTPUT_BINARY)

run:
	cd dist && ./fiber.exe

test:
	$(GO) test ./...

clean:
	@rm -rf $(DIST_DIR)

help:
	@echo "Available targets:"
	@echo "  run    - Run the Go Fiber application"
	@echo "  build  - Build the binary executable in the dist directory"
	@echo "  test   - Run tests"
	@echo "  clean  - Clean the build artifacts"
	@echo "  start  - Build and run the application"

