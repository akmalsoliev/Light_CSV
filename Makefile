help: # Print help on Makefile
	@grep '^[^.#]\+:\s\+.*#' Makefile | \
	sed "s/\(.\+\):\s*\(.*\) #\s*\(.*\)/`printf "\033[93m"`\1`printf "\033[0m"`	\3 [\2]/" | \
	expand -t20

SHELL := /bin/bash

setup: # Creating required folder
	mkdir -p data/
	mkdir -p data/input

slow_run: # Cargo run
	cargo run 

build: # Cargo build with a release and optimization
	cargo build --release

BINARY := target/release/data_processing

_check_file:
	@if [ -f "$(BINARY)" ]; then \
		echo "$(BINARY) file found."; \
	else \
		echo "$(BINARY) not found, building."; \
		make build; \
	fi

run: _check_file # Running optimized processor
	chmod +x $(BINARY)
	$(BINARY)
