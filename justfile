
# Clean the project
clean:
  cargo clean

# Build the project
build:
  cargo build

# Run the project
run: build
  podman compose up --build

dev-db:
  docker-compose up -d

dev: dev-db
  cargo watch -x run
