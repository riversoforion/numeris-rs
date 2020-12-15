#!/bin/bash
export RUSTFLAGS="-Zinstrument-coverage"

cargo +nightly build || exit $?
LLVM_PROFILE_FILE="romanus.profraw" cargo +nightly test || exit $?

rustup run nightly \
 grcov . -s . --binary-path ./target/debug/ -t html \
 --ignore-not-existing --ignore "/*" \
 -o ./target/debug/coverage/
