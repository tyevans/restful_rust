FROM rust:1-bullseye as build

# Create a new empty shell project
# This will let us build a dependency cache
#   speeding up subsequent builds
RUN USER=root cargo new --bin app
WORKDIR /app

# Copy over your manifests
COPY Cargo.lock ./Cargo.lock
COPY Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

# Copy project sources
COPY ./src ./src

# Build current binary
RUN rm ./target/release/deps/backend_app*
RUN cargo build --release
RUN ls -lah ./target/release

# our final base
FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*
# copy the build artifact from the build stage
COPY --from=build /app/target/release/backend_app .
COPY .env .env
CMD ["./backend_app"]
