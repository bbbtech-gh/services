FROM rust:latest

COPY bbbcntr_rust/ ./

# RUN cargo install sqlx-cli

# Build your program for release
RUN cargo build --release

CMD ["echo", "hello\n\n\n\n\n\n\n\n"]
# CMD ["sqlx", "database", "create"]
# CMD ["sqlx", "migrate", "run"]

# Run the binary
CMD ["./target/release/bbbcntr_rust"]
