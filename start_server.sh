RUST_LOG=info

cargo install trunk
rustup target add wasm32-unknown-unknown

# Build frontend

cd frontend
trunk build

cd ..

# Build backend
cd backend
mkdir public
mv ../frontend/dist/* ./public/
cargo r
