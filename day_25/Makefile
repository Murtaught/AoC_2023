.PHONY: all
all:
	@cargo build --release
	@env RUST_BACKTRACE=1 /usr/bin/time --format="wall clock time %E (%e), mem %M KB" cargo run --release
