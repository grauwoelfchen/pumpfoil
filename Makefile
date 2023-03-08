APPLICATION := pumpfoil
COVERAGE := 'covered":"([0-9]*\.[0-9]*|[0-9]*)"' | sed -E 's/[a-z\:"]*//g'

# setup
setup\:vendor: # Install cargo vendor and run it
	@mkdir -p .cargo
	@which cargo-vendor >/dev/null 2>&1 || cargo install \
		cargo-vendor --force
	@cargo vendor > .cargo/config
.PHONY: setup\:vendor

setup\:tool: # Install development tools
	@mkdir -p .git/hooks
.PHONY: setup\:tool

setup\:all: setup\:tool setup\:vendor # Setup vendor and tool both
.PHONY: setup\:all

setup: setup\:all # Sysonym of setup:all
.PHONY: setup

# vet
vet\:check: # Check Rust syntax [synonym: check]
	@cargo check --all --verbose
.PHONY: vet\:check

check: vet\:check
.PHONY: check

vet\:format: # Check format without changes [synonym: vet:fmt, format, fmt]
	@cargo fmt --all -- --check
.PHONY: vet\:format

vet\:fmt: vet\:format
.PHONY: vet\:fmt

format: vet\:format
.PHONY: format

fmt: vet\:format
.PHONY: fmt

vet\:lint: # Check style using clippy [synonym: lint]
	@cargo clippy --all-targets
.PHONY: vet\:lint

lint: vet\:lint
.PHONY: lint

vet\:all: vet\:check vet\:format vet\:lint # Check code using all vet targets
.PHONY: vet\:all

vet: vet\:check # Alias of vet:check
.PHONY: vet

# test
test\:bin: # Run tests for application binary
	@cargo test --bin $(APPLICATION)
.PHONY: test\:bin

test\:all: test\:bin # Run all tests
.PHONY: test\:all

test: test\:bin # Synonym of test:bin
.PHONY: test

# coverage
_get_covered:
	result=($(DST_DIR)/index.js*); \
	if [ -f $${result}[0] ]; then \
		rm "$(DST_DIR)/index.js*"; \
	fi; \
	file=($(DST_DIR)/debug/deps/$(APPLICATION)-*); \
	kcov --verify --include-path=$(SRC_DIR) $(DST_DIR) $${file[0]}; \
	grep 'index.html' $(DST_DIR)/index.js* | \
		grep --only-matching --extended-regexp $(COVERAGE)

coverage\:bin: # Get coverage of tests for application binary [alias: cov:bin]
	@set -uo pipefail; \
	dir="$$(pwd)"; \
	target_dir="$${dir}/target/coverage/bin"; \
	cargo test --bin $(APPLICATION) --no-run --target-dir=$${target_dir}; \
	make -s SRC_DIR=$${dir}/src DST_DIR=$${target_dir} _get_covered
.PHONY: coverage\:bin

cov\:bin: coverage\:bin
.PHONY: cov\:bin

coverage\:all: coverage\:bin # Get coverage from all tests [alias: cov:all]
.PHONY: coverage\:all

cov\:all: | coverage\:all
.PHONY: cov\:all

coverage: coverage\:bin # Synonym of coverage:bin [alias: cov]
.PHONY: cov

cov: coverage
.PHONY: cov

# build
build\:debug: # Run debug build
	@cargo build --bin $(APPLICATION)
.PHONY: build\:debug

build: build\:debug # Synonym of build:debug
.PHONY: build

build\:release: # Build release arfitacts
	cargo build --bin $(APPLICATION) --release
.PHONY: build\:release

# utility
clean: # Remove cache and built artifacts
	@cargo clean
.PHONY: clean

runner-%: # Run a CI job on local (on Docker)
	@set -uo pipefail; \
	job=$(subst runner-,,$@); \
	opt=""; \
	while read line; do \
		opt+=" --env $$(echo $$line | sed -E 's/^export //')"; \
	done < .env.ci; \
	gitlab-runner exec docker \
		--executor docker \
		--cache-dir /cache \
		--docker-volumes $$(pwd)/.cache/gitlab-runner:/cache \
		--docker-volumes /var/run/docker.sock:/var/run/docker.sock \
		$${opt} $${job}
.PHONY: runner

package: # Create package
	@cargo package
.PHONY: package

install:
	@cargo install --path . --force
.PHONY: install

help: # Display this message
	@set -uo pipefail; \
	grep --extended-regexp '^[0-9a-z\:\\\%]+: ' \
		$(firstword $(MAKEFILE_LIST)) | \
		grep --extended-regexp ' # ' | \
		sed --expression='s/\([a-z0-9\-\:\ ]*\): \([a-z0-9\-\:\ ]*\) #/\1: #/g' | \
		tr --delete \\\\ | \
		awk 'BEGIN {FS = ": # "}; \
			{printf "\033[38;05;222m%-14s\033[0m %s\n", $$1, $$2}' | \
		sort
.PHONY: help

.DEFAULT_GOAL = build:debug
