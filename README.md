# QEDA

[![Build Status](https://github.com/qeda/qeda-rs/workflows/Build/badge.svg)](https://github.com/qeda/qeda-rs/actions)

QEDA is a command-line tool aimed to simplify creating electronic component libraries for using in EDA software. You can easily create both symbols for schematic and land patterns for PCB.

* :eight_spoked_asterisk: https://qeda.org
* :book: https://docs.qeda.org

**Attention!** Work in progress at the moment. There is the significant lack of functionality. The project is not ready for using in production.

## Download

* Linux development build: https://builds.qeda.org/dev/qeda-linux-x86_64-dev.tar.xz
* Windows development build: https://builds.qeda.org/dev/qeda-windows-x86_64-dev.7z

## Comparing to the Previous Version

The [previous version](https://github.com/qeda/qeda) was written using CoffeeScript and it is available as an [NPM module](https://www.npmjs.com/package/qeda).

This version is rewritten in Rust language from scratch.

- [x] Faster
- [x] Improved error handling
- [x] Using SVG for discrete component symbols
- [x] More idiomatic component YAML-descripton
- [ ] Custom symbols from SVG
- [ ] More powerful land pattern generator for non-standard patterns
- [ ] Step 3D-models generation
- [x] Multithreading support
- [ ] Server mode

## Roadmap

- Symbols:
    - [x] Capacitor
    - [ ] Capacitor polarized
    - [ ] Connector
    - [ ] Crystal
    - [ ] Diode
    - [ ] FET
    - [ ] Fuse
    - [ ] Ground
    - [ ] Integrated circuit
    - [ ] Inductor
    - [ ] LED
    - [ ] Mounting hole
    - [ ] Power
    - [ ] Pushbutton
    - [ ] Resistor
    - [ ] Switch
    - [ ] Test point
    - [ ] Transformer
    - [ ] Transistor
    - [ ] Twin diode

- Patterns:
    - [ ] Axial lead
    - [ ] BGA
    - [ ] Bridge
    - [ ] CAE
    - [ ] CFP
    - [ ] CGA
    - [ ] Chip
    - [ ] Chip array
    - [ ] CQFP
    - [ ] Crystal
    - [ ] Custom
    - [ ] DIP
    - [ ] LCC
    - [ ] LGA
    - [ ] MELF
    - [ ] Molded
    - [ ] Mounting hole
    - [ ] Oscillator
    - [ ] PAK
    - [ ] PGA
    - [ ] PLCC
    - [ ] Radial lead
    - [ ] QFN
    - [ ] QFP
    - [ ] Radial
    - [ ] SIP
    - [ ] SOD
    - [ ] SODFL
    - [ ] SOIC
    - [ ] SOJ
    - [ ] SOL
    - [ ] SON
    - [ ] SOP
    - [ ] SOPFL
    - [ ] SOT
    - [ ] SOT143
    - [ ] SOT223
    - [ ] SOT23
    - [ ] SOT89-5
    - [ ] SOTFL
    - [ ] TO (Flange mount)
    - [ ] TO (Cylindrical)
    - [ ] Test point
    - [ ] Wire

- Outlines:
    - [ ] JEDEC
    - [ ] JEITA
    - [ ] NXP

- EDA:
    - [x] KiCad

## Build on Ubuntu

0. Prerequisites:

        sudo apt install -y pkgconf libssl-dev

1. Build:

        cargo build

2. Test:

        ./target/debug/qeda --help

## First Steps

Load and add a new component:

    qeda add capacitor/c0603

Generate a new KiCad library:

    qeda generate mylib

## More Details

Run for available options:

    qeda --help

## License

Source code is licensed under the [MIT license](LICENSE).
