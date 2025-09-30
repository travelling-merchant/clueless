# ---- Build Stage ----
FROM rust:1.90 AS builder

##  RUN apt-get update && apt-get install -y pkg-config libssl-dev wasm-pack

WORKDIR /app

COPY . .

# Build wasm bundle (using trunk, wasm-pack, or cargo-leptos)
RUN cargo install trunk 
RUN rustup target add wasm32-unknown-unknown

RUN trunk build --release

# ---- Runtime Stage ----
FROM nginx:1.27-alpine

# Copy build artifacts to nginx html directory
COPY --from=builder /app/dist /usr/share/nginx/html

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]


# docker run -p 8080:80 clueless

