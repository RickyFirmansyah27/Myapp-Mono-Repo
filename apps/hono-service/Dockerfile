# Stage 1: Builder
FROM oven/bun:1 AS builder
WORKDIR /app

# Copy dependencies
COPY package.json .

# Install dependencies
RUN bun install

COPY . .

# Build hono service
WORKDIR /app/apps/hono-service
RUN bun install
RUN bun run build

# Stage 2: Runner (Production)
FROM oven/bun:1-slim AS runner
WORKDIR /app

# Copy hasil build dari packages dan hono-service
COPY --from=builder /app/apps/hono-service/dist ./dist

# Set up environment
ENV PORT=8105
EXPOSE 8105

CMD ["bun", "./dist/index.js"]
