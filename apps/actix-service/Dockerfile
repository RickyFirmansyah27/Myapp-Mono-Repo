# Stage 1: Build
FROM rust:alpine AS builder

# Install dependensi build dan update Cargo
RUN apk add --no-cache musl-dev gcc && \
    rustup update

# Set working directory
WORKDIR /usr/src/app

# Salin file Cargo.toml dan Cargo.lock terlebih dahulu untuk caching dependensi
COPY Cargo.toml Cargo.lock ./

# Jalankan build awal untuk meng-cache dependensi
RUN cargo build --release || true

# Salin seluruh kode aplikasi
COPY . .

# Build aplikasi dalam mode release
RUN cargo build --release

# Periksa apakah binary ada
RUN ls /usr/src/app/target/release/

# Stage 2: Runtime
FROM alpine:latest

# Install dependensi runtime
RUN apk add --no-cache libc6-compat

# Salin binary hasil build dari tahap pertama
COPY --from=builder /usr/src/app/target/release/actix-service /usr/local/bin/actix-service

# Ekspose port (sesuaikan dengan aplikasi Anda)
EXPOSE 8101

# Set binary sebagai entrypoint
ENTRYPOINT ["/usr/local/bin/actix-service"]
