QEDA
====

QEDA is a command-line tool aimed to simplify creating libraries of electronic components for using in EDA software. You can easily create both symbols for schematic and land patterns for PCB.

Build
=====

Ubuntu
------

Prerequisites::

    sudo apt install -y pkgconf libssl-dev

Build::

    cargo build

Test::

    ./target/debug/qeda --help

License
=======

Source code is licensed under `MIT license <LICENSE>`__.
