.PHONY: all clean install prepare release

all:
	@cargo build

clean:
	@rm -rvf target

install:
	@cargo install --path .

prepare:
	@sudo apt install -y pkgconf libssl-dev

release:
	@cargo build --release
