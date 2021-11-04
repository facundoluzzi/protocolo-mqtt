SERVER_TOML_PATH=--manifest-path=server/Cargo.toml
CLIENT_TOML_PATH=--manifest-path=client/Cargo.toml

.PHONY: server
.PHONY: client
.PHONY: graphics

server: server/src/main.rs
	cargo fmt $(SERVER_TOML_PATH)
	cargo clippy $(SERVER_TOML_PATH)
	cargo run $(SERVER_TOML_PATH) server.conf

client: client/src/main.rs
	cargo fmt $(CLIENT_TOML_PATH)
	cargo clippy $(CLIENT_TOML_PATH)
	cargo run --bin client $(CLIENT_TOML_PATH)

server-test:
	cargo test $(SERVER_TOML_PATH)

client-test:
	cargo test $(CLIENT_TOML_PATH)

graphics:
	cargo fmt $(CLIENT_TOML_PATH)
	cargo clippy $(CLIENT_TOML_PATH)
	cargo run $(CLIENT_TOML_PATH) --bin interface


