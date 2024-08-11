#!/bin/bash

set -euxo pipefail
REPO_DIR="$PWD"
REPO_NAME=$(basename "$REPO_DIR")
OUT_DIR_SUFFIX=".cargo-gen-test-$REPO_NAME"
rm -rf "${TMPDIR:-/tmp}"/tmp.*"$OUT_DIR_SUFFIX"
# cargo has an annoyingly unconfigurable habit of looking in parent dirs for Cargo.toml files
# (ref https://github.com/rust-lang/cargo/issues/7871)
# of course, this will break because the parent of this dir has a template Cargo.toml with "invalid" chars
# hope you don't have an invalid `/tmp/Cargo.toml`!
OUT_DIR=$(mktemp -d --suffix="$OUT_DIR_SUFFIX")
# we still want easy access to the test output in our IDE (this symlink is gitignored)
ln -sfT "$OUT_DIR" "$REPO_DIR/__test__/out"
cd "$OUT_DIR"
rustup run stable cargo generate --init --path "$REPO_DIR" --name "contract-gen-test"
CARGO_BUILD_TARGET_DIR="$REPO_DIR/__test__/target" rustup run stable cargo unit-test
CARGO_BUILD_TARGET_DIR="$REPO_DIR/__test__/target" rustup run stable cargo doc-test
CARGO_BUILD_TARGET_DIR="$REPO_DIR/__test__/target" rustup run stable cargo schema
