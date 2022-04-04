SHELL = /bin/sh
.DEFAULT_GOAL=test

HEAVYDB_VERSION=v5.7.0
DB_CONTAINER = heavyai-test-db

CURRENT_UID := $(shell id -u)

-include .env

deps:
	which cargo || curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
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
	./generate_thrift_bindings.sh heavydb
.PHONY: thrift

%.thrift:
	mkdir -p "heavydb/QueryEngine"
	# curl "https://raw.githubusercontent.com/heavyai/heavydb/rc/${HEAVYDB_VERSION}/$@" -o "heavydb/$@"
	cp "${HEAVYDB_DIR}/$@" "heavydb/$@"

get_thrift: heavy.thrift common.thrift completion_hints.thrift QueryEngine/serialized_result_set.thrift QueryEngine/extension_functions.thrift
.PHONY: get_thrift

#
# Docker
#

up:
	mkdir -p target/${DB_CONTAINER}
	docker run --name ${DB_CONTAINER} -d --rm \
		-v ${PWD}:/src \
		-v ${PWD}/target/${DB_CONTAINER}:/omnisci-storage \
		-p 6273-6274:6273-6274 \
		heavyai/core-os-cpu:${HEAVYDB_VERSION}
.PHONY: up

down:
	docker rm -f ${DB_CONTAINER}
.PHONY: down

docker_builder:
	docker build -f docker/Dockerfile -t build-heavyai-rs .
.PHONY: docker_builder

# This rule lets you run any other make target within the build container like `make all.docker`
# TODO --user=${CURRENT_UID}
# TODO fix test to reference docker hostname so test can run within container
%.docker: docker_builder
	# TODO avoid printing this harmless message: "mesg: ttyname failed: Inappropriate ioctl for device"
	docker run -i --rm \
		-v ${PWD}:/src \
		-v ${HOME}/.cargo/registry:/home/user/.cargo/registry \
		build-heavyai-rs \
		bash -l -c "make $*"
