INSTALL_DIR = /usr/local/bin
SHELL = /usr/bin/env sh
EXECUTABLE_PATH = ./target/debug/jl
JL_PATH = $(HOME)/.jl
QUESTIONS_FILE = ./questions.txt

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

install-bin:
	@mkdir -p "$(INSTALL_DIR)"
	@cp "$(EXECUTABLE_PATH)" "$(INSTALL_DIR)/"

install-user:
	@mkdir -p "$(HOME)/.jl"
	@cp "$(QUESTIONS_FILE)" "$(HOME)/.jl"

uninstall:
	@echo Deleting jl...
	@rm $(shell command -v jl)
