# Makefile — thin shim that defers to justfile (preferred)
# Install just: https://github.com/casey/just

.DEFAULT_GOAL := help
.PHONY: help install dev build preview test lint format deploy clean

help:
	@echo "planify-wt — use 'just' for the canonical interface."
	@echo "Available targets: install dev build preview test lint format deploy clean"
	@just --list

install dev build preview test lint format deploy clean:
	@just $(MAKECMDGOALS)
