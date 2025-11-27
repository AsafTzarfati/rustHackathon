# Stage 1: Builder for Frontend
FROM rust:1.80 as frontend-builder
WORKDIR /app
RUN apt-get update && apt-get install -y protobuf-compiler
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk wasm-bindgen-cli

COPY . .
# Build frontend
WORKDIR /app/frontend
# We need to build shared first or let cargo handle it
# Trunk needs index.html, let's assume we need to create one or trunk will use default?
# Usually we need an index.html. I will create one in the next step.
# For now, let's assume we run cargo build --release for backend and trunk build --release for frontend.

# Actually, let's use a simpler approach for the hackathon:
# Just build backend and copy wasm artifacts.
# But we need trunk to build leptos properly.

# Let's assume we have a basic index.html
RUN touch index.html 
# Wait, Leptos usually needs a specific index.html.
# I will create it.

# Stage 2: Builder for Backend
FROM rust:1.80 as backend-builder
RUN apt-get update && apt-get install -y protobuf-compiler
WORKDIR /app
COPY . .
RUN cargo build --release --bin backend

# Stage 3: Runtime
FROM debian:bookworm-slim
WORKDIR /app
COPY --from=backend-builder /app/target/release/backend .
# COPY --from=frontend-builder /app/frontend/dist ./dist 
# (We need to implement the frontend build properly)

# For now, just the backend binary and expose ports
EXPOSE 3000 5000/udp
CMD ["./backend"]
