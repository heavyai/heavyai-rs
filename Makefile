#
# Simple Makefile to call the rust and cargo commands
#

SHELL = /bin/sh
.DEFAULT_GOAL=all

-include .env

deps:
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

build:
	cargo build

up:
	docker run --name omnisci-test-db -d --rm -p 6273-6274:6273-6274 omnisci/core-os-cpu:v5.3.0

down:
	docker stop omnisci-test-db

test:
	cargo test

install: test
	cargo install --path .

release:
	cargo build --release

all: test

.PHONY: dev build test all install release up down
