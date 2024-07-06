#!/usr/bin/env bash

set -eux

# Copies over welcome message
mkdir -p /usr/local/etc/vscode-dev-containers/
cp .devcontainer/welcome-message.txt /usr/local/etc/vscode-dev-containers/first-run-notice.txt

# Setup permissions
sudo chown -R vscode:vscode /usr/local/cargo/registry
sudo chown -R vscode:vscode /workspace/target

# Fetch the latest package list
sudo apt-get update

# Install PostgreSQL client
sudo apt-get install -y postgresql-client

# Install Redis client
apt-get install -y redis-tools

# Install Rust tools
cargo install cargo-watch
