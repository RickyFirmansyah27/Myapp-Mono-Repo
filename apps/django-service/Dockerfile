# Gunakan image Python yang lebih kecil: python:3.9-alpine
FROM python:3.9-alpine AS base

# Set working directory
WORKDIR /app

# Install dependensi sistem yang dibutuhkan untuk build
RUN apk update && apk add --no-cache \
    build-base \
    libpq-dev \
    && rm -rf /var/cache/apk/*

# Salin file requirements.txt terlebih dahulu untuk memanfaatkan cache layer
COPY requirements.txt /app/

# Install dependencies Python
RUN pip install --no-cache-dir -r requirements.txt

# Salin seluruh proyek ke dalam kontainer
COPY . /app/

# Buat virtual environment di dalam kontainer
RUN python -m venv venv

# Install dependencies Python di dalam virtual environment
RUN venv/bin/pip install --no-cache-dir -r requirements.txt

# Set environment variable
ENV DJANGO_SETTINGS_MODULE=server.settings

# Expose port yang digunakan oleh Django
EXPOSE 8102

# Command default untuk menjalankan aplikasi Django menggunakan virtual environment
CMD ["venv/bin/python", "manage.py", "runserver", "0.0.0.0:8102"]
