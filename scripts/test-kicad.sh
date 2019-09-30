#!/bin/sh

set -e
cd "$(dirname "$(readlink -f "${0}")")/.."

cargo build
cd scripts
../target/debug/qeda add capacitor/c0603
../target/debug/qeda generate test

