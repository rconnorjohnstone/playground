FROM rustlang/rust:nightly-stretch-slim

RUN mkdir /app && cd /app
RUN mkdir -p dockerCache/cargo
RUN mkdir -p dockerCache/target
WORKDIR /app

# Build the dependencies (and cache)
COPY ./Cargo.toml .
COPY ./Cargo.lock .
COPY ./Rocket.toml .
RUN mkdir src
RUN touch src/main.rs
RUN echo "fn main() {}" >> src/main.rs
RUN cargo build
RUN rm -rf src

# Build the project
COPY ./static ./static
COPY ./src ./src

EXPOSE 8000

CMD ["cargo", "run"]
