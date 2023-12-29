.PHONY: all
all:
	cargo run --release

coverage:
	rm -f cargo-run-*
	env \
		CARGO_INCREMENTAL=0 \
		RUSTFLAGS='-Cinstrument-coverage' \
		LLVM_PROFILE_FILE='cargo-run-%p-%m.profraw' \
		cargo run
	grcov \
		. \
		--binary-path ./target/debug/deps/ \
		--llvm-path /usr/bin \
		-s . \
		-t html \
		--branch \
		--ignore-not-existing \
		--ignore '../*' \
		--ignore "/*" \
		-o target/coverage/html
