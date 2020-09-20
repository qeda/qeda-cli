#!/bin/bash

set -e
cd "$(dirname "$(readlink -f "${0}")")/.."

cargo build
cd scripts

if [ "$1" == "-c" ] || [ "$1" == "--clean" ] ; then
  rm -frv kicadlib qedalib
fi

QEDA_DEBUG=1 ../target/debug/qeda reset
QEDA_DEBUG=1 ../target/debug/qeda add capacitor/c0603
QEDA_DEBUG=1 ../target/debug/qeda generate test
