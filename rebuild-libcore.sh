#!/bin/sh
TARGET=thumbv7em-none-eabi

SYSROOT=$(rustc --print sysroot)
SRC_DIR=$SYSROOT/lib/rustlib/src/rust/src/libcore
LIB_DIR=$SYSROOT/lib/rustlib/$TARGET/lib

cp $(dirname $0)/$TARGET.json $SRC_DIR
cd $SRC_DIR
cargo rustc --release --target=$TARGET -- -C panic=abort
mkdir -p $LIB_DIR
cp target/$TARGET/release/libcore.rlib $LIB_DIR
