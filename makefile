SERVER_TOML_PATH=--manifest-path=server/Cargo.toml
CLIENT_TOML_PATH=--manifest-path=client/Cargo.toml

server: server/src/main.rs
	cargo fmt $(SERVER_TOML_PATH)
	cargo clippy $(SERVER_TOML_PATH)
	cargo run $(SERVER_TOML_PATH)

client: client/src/main.rs
	cargo fmt $(CLIENT_TOML_PATH)
	cargo clippy $(CLIENT_TOML_PATH)
	cargo run $(CLIENT_TOML_PATH)
