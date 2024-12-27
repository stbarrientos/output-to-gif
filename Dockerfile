FROM rust:latest as base
WORKDIR /usr/output-to-gif

FROM base as dev

FROM base as release

COPY . /usr/output-to-gif/
WORKDIR /usr/output-to-gif

# Build the Rust client
RUN cargo build --release

CMD [ "cargo", "run" ]