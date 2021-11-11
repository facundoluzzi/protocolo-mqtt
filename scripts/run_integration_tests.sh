cd ..
cargo run --manifest-path=server/Cargo.toml server.conf &
cd client
cargo test

# lsof -i tcp:1883
# copiar el PID del proceso server
# sudo kill -9 <pegar_id>