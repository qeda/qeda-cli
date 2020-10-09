#!/bin/bash

set -e
cd "$(dirname "$(readlink -f "$0")")/.."

cargo build
cd scripts

rm -rf .index
rm -rf qedalib

if [ "$1" == "-c" ] || [ "$1" == "--clean" ] ; then
  rm -rf lib
fi

if [ ! -d "lib" ]; then
  git clone https://github.com/qeda/lib.git --depth=1
fi

mkdir -p qedalib
cd lib
find . -type d -not -path "./.*" -not -path "." -exec cp -frv {} ../qedalib/  \;
cd ..

QEDA_DEBUG=1 ../target/debug/qeda index
