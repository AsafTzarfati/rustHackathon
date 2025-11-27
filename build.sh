#!/bin/bash
set -e

# Function to handle errors
handle_error() {
    echo "Error occurred in build script at line $1"
    exit 1
}

trap 'handle_error $LINENO' ERR

echo "Checking for required tools..."
if ! command -v cargo &> /dev/null; then
    echo "Error: 'cargo' is not installed. Please install Rust."
    exit 1
fi

if ! command -v trunk &> /dev/null; then
    echo "Error: 'trunk' is not installed. Please install it with: cargo install trunk"
    exit 1
fi

echo "Building Backend (Release)..."
cd backend
cargo build --release
cd ..

echo "Building Frontend (Release)..."
cd frontend
trunk build --release
cd ..

echo "Build complete!"
