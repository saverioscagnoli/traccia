#!/bin/bash
set -e

# Go to Rust crate
cd rust

# Build docs
cargo doc --no-deps

# Back to root
cd ..

# Copy docs to custom location
mkdir -p doc/rust
rm -rf doc/rust/*
cp -r rust/target/doc/* doc/rust/
