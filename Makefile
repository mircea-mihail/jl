INSTALL_DIR = /usr/local/bin
SHELL = /usr/bin/env sh
EXECUTABLE_PATH = ./target/debug/jl

all:
	@echo Run \'make compile\' to compile jl.
	@echo
	@echo Run \'make install\' to install jl.
	@echo Run \'make uninstall\' to uninstall jl.
	@echo
	@echo For custom installation directory, use \'make INSTALL_DIR=...\'.
	@echo Current installation directory is \'$(INSTALL_DIR)\'.

compile:
	cargo build

install:
	@echo Making directory $(INSTALL_DIR)...
	@mkdir -p $(INSTALL_DIR)
	@echo Copying jl to it...
	@cp $(EXECUTABLE_PATH) $(INSTALL_DIR)/

uninstall:
	@echo Deleting jl...
	@rm $(shell command -v jl)
