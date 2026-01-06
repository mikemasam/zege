# === Stage 1: Build web===
FROM node:20-alpine AS web-builder
WORKDIR /app/web

COPY web/package*.json ./
RUN npm install --force
COPY web/ ./
RUN npm run build

# === Stage 2: Build runtime ===
FROM rust:1.90-alpine AS runtime-builder

RUN rustup target add x86_64-unknown-linux-musl 
RUN apk add musl-dev build-base pkgconfig

WORKDIR /app/runtime
COPY runtime/Cargo.toml runtime/Cargo.lock ./
COPY runtime/src ./src

RUN cargo fetch

# Build runtime
RUN cargo build --release --target x86_64-unknown-linux-musl


# === Stage 3: Runtime ===
FROM scratch

# Copy runtime binary
COPY --from=runtime-builder /app/runtime/target/x86_64-unknown-linux-musl/release/zege /app/zege
COPY runtime/migrations /app/migrations
COPY runtime/config.yaml.sample /app/config.yaml

# Copy built web into runtime's static folder
COPY --from=web-builder /app/web/dist /app/static
#COPY --from=runtime-builder /app/runtime/static /app/static

WORKDIR /app
EXPOSE 3432
ENTRYPOINT ["/app/zege"]
CMD ["-d"]

