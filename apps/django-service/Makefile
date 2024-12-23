# Makefile for Django service

# Environment Variables
export DJANGO_SETTINGS_MODULE=server.settings

# Python Environment
PYTHON=py
PIP=pip

# Paths
VENV_DIR=venv

# Install dependencies
install:
	$(PYTHON) -m venv $(VENV_DIR)
	$(VENV_DIR)/Scripts/$(PIP) install -r requirements.txt

# Run Django server
run:
	$(VENV_DIR)/Scripts/python manage.py runserver 8102

# Run migrations
migrate:
	$(VENV_DIR)/Scripts/python manage.py migrate

# Create a superuser
createsuperuser:
	$(VENV_DIR)/Scripts/python manage.py createsuperuser

# Run tests
test:
	$(VENV_DIR)/Scripts/python manage.py test

# Collect static files
collectstatic:
	$(VENV_DIR)/Scripts/python manage.py collectstatic --noinput

# Clean up pyc files
clean:
	find . -name "*.pyc" -exec rm -f {} \;

# Show help
help:
	@echo "Makefile for Django service"
	@echo "Usage:"
	@echo "  make install        - Install dependencies"
	@echo "  make run            - Run Django development server"
	@echo "  make migrate        - Run database migrations"
	@echo "  make createsuperuser - Create a superuser"
	@echo "  make test           - Run tests"
	@echo "  make collectstatic  - Collect static files"
	@echo "  make clean          - Clean up pyc files"
