FROM rust:latest

# Create a new empty directory for our application
RUN mkdir -p /usr/src/app

# Set the working directory for the subsequent commands
WORKDIR /usr/src/app

# Copy the entire contents of our project into the container
COPY . .

# Build our application using Cargo
RUN cargo build --release

# Set the binary as the entrypoint of the container
ENTRYPOINT ["/usr/src/app/target/release/null-bot"]
