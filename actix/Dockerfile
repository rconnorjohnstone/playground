FROM rust:1.40-slim-stretch

RUN mkdir /app && cd /app
RUN mkdir -p dockerCache/cargo
RUN mkdir -p dockerCache/target
WORKDIR /app

# Build the dependencies (and cache)
COPY ./Cargo.toml .
RUN mkdir src
RUN touch src/main.rs
RUN echo "fn main() {}" >> src/main.rs
ENV CARGO_HOME=/app/dockerCache/cargo
ENV CARGO_TARGET_DIR=/app/dockerCache/target
RUN cargo build
RUN rm -rf src

# Install the file watch stuff
RUN cargo install systemfd cargo-watch

# Build the project
#COPY . .
RUN echo 

EXPOSE 8088

CMD ["systemfd", "--no-pid", "-s", "http::3000", "--", "cargo", "watch", "-x", "run"]
