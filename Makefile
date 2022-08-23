test:
	@cargo test --all-features --all-targets

check:
	@cargo +nightly fmt --all
	@cargo clippy --fix --allow-dirty --allow-staged --all-targets --all-features
	@cargo update --dry-run
	@cargo outdated -wR
	@cargo doc --no-deps --all-features --examples --document-private-items
	@cargo +nightly udeps --all-targets --all-features

check_nightly:
	@cargo +nightly clippy --fix --allow-dirty --allow-staged --all-targets --all-features

check_strictly:
	@cargo +nightly clippy --fix --allow-dirty --allow-staged --all-features --all-targets -- -W clippy::all -W clippy::pedantic -W clippy::cargo

