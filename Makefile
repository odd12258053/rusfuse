format:
	@cargo fmt

lint:
	@cargo fmt -- --check
	@cargo check
