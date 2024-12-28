#!/usr/bin/bash

USAGE="Usage: $0 <build-type: {dev | openwrt }>"
BUILD_TYPE="$1"
if [[ -z $BUILD_TYPE ]]; then
	echo $USAGE
	exit 1
fi


if [ "$BUILD_TYPE" == "dev" ]; then
	cargo build --release
elif [ "$BUILD_TYPE" == "openwrt" ]; then
	# This is for rasberry pi 4
	CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_LINKER=rust-lld cargo build --release --target aarch64-unknown-linux-musl
else
	echo $USAGE
	exit 1
fi
