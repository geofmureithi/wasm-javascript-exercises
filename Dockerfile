# Use Ubuntu version 20.04 as the base image
FROM ubuntu:20.04

# Update the package list and upgrade existing packages
RUN apt-get update && apt-get upgrade -y

# Install C linker
RUN apt install -y build-essential

# Install rustup
RUN apt-get install -y curl
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="$PATH:/root/.cargo/bin"

# Prevent interactive prompts
ARG DEBIAN_FRONTEND=noninteractive

# Install PostgreSQL
RUN apt-get install -y postgresql postgresql-contrib

# Start PostgreSQL
RUN service postgresql start

# Copy the code to the container
COPY . /usr/src/wasm-exercises

# Install libpq
RUN apt-get install libpq-dev

# Install diesel cli
RUN cargo install diesel_cli --no-default-features --features postgres

# Build the code with cargo
RUN cargo build --release --manifest-path /usr/src/wasm-exercises/Cargo.toml

# Expose the port on the container
EXPOSE 8080

#CMD cargo run --release --manifest-path /usr/src/wasm-exercises/Cargo.toml

# Install the binary
WORKDIR /usr/src/wasm-exercises
RUN cargo install --path .

# Run diesel setup
RUN diesel setup

# Run diesel migrate
RUN diesel migration run

# Run
CMD ["wasm-javascript-exercises"]

