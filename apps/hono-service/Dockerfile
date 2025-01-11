# Stage 1: Builder
FROM oven/bun:1 AS builder
WORKDIR /app

# Copy seluruh proyek termasuk semua packages dan apps
COPY . .

# Install dependencies
RUN bun install

# Build semua packages (misalnya `bun build` digunakan untuk membuild package)
RUN bun run build

# Stage 2: Runner (Production)
FROM oven/bun:1-slim AS runner
WORKDIR /app

# Copy hasil build dari packages dan hono-service
COPY --from=builder /app/dist ./dist
COPY --from=builder /app/package.json ./

# Set up environment
ENV PORT=8105
EXPOSE 8105

CMD ["bun", "./dist/index.js"]
