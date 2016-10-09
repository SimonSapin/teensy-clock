#!/bin/sh
TARGET=thumbv7em-none-eabi

SYSROOT=$(rustc --print sysroot)
SRC_DIR=$SYSROOT/lib/rustlib/src/rust/src/libcore
LIB_DIR=$SYSROOT/lib/rustlib/$TARGET/lib

cp $(dirname $0)/$TARGET.json $SRC_DIR
# Don’t cd into $SRC_DIR, use current directory’s rustup override.
cargo rustc --manifest-path $SRC_DIR/Cargo.toml --release --target=$TARGET -- -C panic=abort
mkdir -p $LIB_DIR
cp $SRC_DIR/target/$TARGET/release/libcore.rlib $LIB_DIR
