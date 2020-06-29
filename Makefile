SHELL = /bin/sh
.DEFAULT_GOAL=test

OMNISCI_VERSION=v5.2.2

CURRENT_UID := $(shell id -u)

-include .env

deps:
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
	# TODO install thrift
.PHONY: deps

build:
	cargo build
.PHONY: build

test:
	cargo test
.PHONY: test

install: test
	cargo install --path .
.PHONY: install

release:
	cargo build --release
.PHONY: release

all: get_thrift thrift test release
.PHONY: all

#
# Thrift
#

thrift:
	./generate_thrift_bindings.sh omniscidb
.PHONY: thrift

%.thrift:
	mkdir -p "omniscidb/QueryEngine"
	curl "https://raw.githubusercontent.com/omnisci/omniscidb/rc/${OMNISCI_VERSION}/$@" -o "omniscidb/$@"

get_thrift: omnisci.thrift common.thrift completion_hints.thrift QueryEngine/serialized_result_set.thrift QueryEngine/extension_functions.thrift
.PHONY: get_thrift

#
# Docker
#

up:
	docker run --name omnisci-test-db -d --rm -p 6273-6274:6273-6274 omnisci/core-os-cpu:v5.3.0
.PHONY: up

down:
	docker stop omnisci-test-db
.PHONY: down

docker_builder:
	docker build -f docker/Dockerfile -t build-omnisci-rs .
.PHONY: docker_builder

%.docker: docker_builder
	# TODO avoid printing this harmless message: "mesg: ttyname failed: Inappropriate ioctl for device"
	# --user=${CURRENT_UID}
	docker run -i --rm \
		-v ${PWD}:/src \
		-v ${HOME}/.cargo/registry:/home/user/.cargo/registry \
		build-omnisci-rs \
		bash -l -c "make $*"
