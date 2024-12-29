# Variables
MVN=mvn

# Targets
run:
	$(MVN) compile exec:java

build:
	$(MVN) package

test:
	$(MVN) test

clean:
	$(MVN) clean

help:
	@echo "Available targets:"
	@echo "  dev    - Run the Vert.x service in development mode"
	@echo "  build  - Build the JAR package"
	@echo "  test   - Run tests"
	@echo "  clean  - Clean the build artifacts"
