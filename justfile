
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

build-css:
  npm run build:css

dev: dev-db build-css
  cargo watch -x run

watch-css:
  npm run watch:css
