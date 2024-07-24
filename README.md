# roxy

a web socket proxy that rox!

## get started

1. Install [Rust](https://www.rust-lang.org/tools/install)
2. Install [Docker](https://docs.docker.com/engine/install/)
3. Install Node and npm (this is only for local development purposes)

Once project dependencies are installed, start up the development environment:

1. Ensure Docker is running
2. Run `make dev.up` from your terminal. This will spin up a simple mock local environment with a client and three upstream backend servers
3. To start the actual roxy proxy run `make run`

To experiment with the proxy open <http://localhost:8080>.

Run `make help` to see other available utilities that can be useful for local development and tests
