#!/bin/bash

# Function to handle cleanup
cleanup() {
    echo "Stopping services..."
    # Kill all child processes in the current process group
    kill 0
}

# Check for required tools
if ! command -v cargo &> /dev/null; then
    echo "Error: 'cargo' is not installed. Please install Rust."
    exit 1
fi

if ! command -v trunk &> /dev/null; then
    echo "Error: 'trunk' is not installed. Please install it with: cargo install trunk"
    exit 1
fi

# Trap SIGINT (Ctrl+C) and EXIT
trap cleanup SIGINT EXIT

echo "Starting Backend..."
# Run backend in background
(cd backend && cargo run) &

# Wait a moment for backend to potentially start up
sleep 2

echo "Starting Frontend..."
# Run frontend in background
(cd frontend && trunk serve --open) &

# Wait for all background processes
wait
