FROM rust:1.60.0-slim

# Copy all the files from local machine to container
COPY ./ ./

# Build the program
RUN cargo build --release

# Run the binary
CMD ["./target/release/docker-hello-world"]
