#!/bin/sh

set -e
cd "$(dirname "$(readlink -f "${0}")")/.."

cargo build
cd scripts
QEDA_DEBUG=1 ../target/debug/qeda add capacitor/c0603
QEDA_DEBUG=1 ../target/debug/qeda generate test
