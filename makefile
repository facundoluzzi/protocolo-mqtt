SERVER_TOML_PATH=--manifest-path=server/Cargo.toml
CLIENT_TOML_PATH=--manifest-path=client/Cargo.toml
FAKE_GENERATOR_PATH=--manifest-path=fake-generator/Cargo.toml

.PHONY: server
.PHONY: client
.PHONY: fake_generator
.PHONY: graphics

server: server/src/main.rs
	cargo fmt $(SERVER_TOML_PATH)
	cargo clippy $(SERVER_TOML_PATH)
	cargo run $(SERVER_TOML_PATH) server.conf

client: client/src/main.rs
	cargo fmt $(CLIENT_TOML_PATH)
	cargo clippy $(CLIENT_TOML_PATH)
	cargo run --bin client $(CLIENT_TOML_PATH)

fake_generator: fake-generator/src/main.rs
	cargo fmt $(FAKE_GENERATOR_PATH)
	cargo clippy $(FAKE_GENERATOR_PATH)
	cargo run $(FAKE_GENERATOR_PATH) server.conf

server-test:
	cargo test $(SERVER_TOML_PATH)

client_test:
	cargo run $(SERVER_TOML_PATH) server.conf
	cargo test $(CLIENT_TOML_PATH)

graphics:
	cargo fmt $(CLIENT_TOML_PATH)
	cargo clippy $(CLIENT_TOML_PATH)
	cargo run $(CLIENT_TOML_PATH) --bin interface


