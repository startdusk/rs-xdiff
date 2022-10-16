.PNHOY: clean
clean:
	@cargo check && cargo clippy && cargo test

.PNHOY: codeline
codeline:
	@tokei .

.PNHOY: test
test:
	@cargo nextest run
	