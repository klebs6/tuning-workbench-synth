.PHONY: limit build default
    
CARGO_MSG_LIMIT := 100

ACTIVE_TEST := 

#--release
#--color always 
TAIL_FLAGS := 

CARGO_BENCH  := cargo bench
CARGO_BUILD  := cargo build
CARGO_CHECK  := cargo check
CARGO_CLIPPY := cargo clippy
CARGO_LFIX   := cargo fix
CARGO_RUN    := cargo run
CARGO_TEST   := cargo test

CARGO_LBENCH  := cargo lbench
CARGO_LBUILD  := cargo lbuild
CARGO_LCHECK  := cargo lcheck
CARGO_LCLIPPY := cargo lclippy
CARGO_LLFIX   := cargo lfix
CARGO_LRUN    := cargo lrun
CARGO_LTEST   := cargo ltest

RUSTFLAGS  := "-Z macro-backtrace -Awarnings"
CARGO_LENV := CARGO_MSG_LIMIT=100 CARGO_TIME_LIMIT=10
CARGO_ENV  := RUSTFLAGS=$(RUSTFLAGS)

default: build

build:
	$(CARGO_ENV) $(CARGO_BUILD)

limit:
	$(CARGO_LENV) $(CARGO_ENV) $(CARGO_LBUILD)

test:
	env RUSTFLAGS="-Awarnings" cargo test -- --nocapture

