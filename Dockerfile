FROM rust:1.46.0
RUN rustup default nightly
WORKDIR /app
RUN USER=root cargo new identity
WORKDIR /app/identity
COPY Cargo.lock Cargo.toml Rocket.toml ./
COPY src ./src
EXPOSE 8000
CMD cargo run
