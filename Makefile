.PHONY: check clippy test publish

check:
	cargo check --features=unstable
	cargo check --features=unstable,std

clippy:
	cargo clippy --features=unstable
	cargo clippy --features=unstable,std

test:
	cargo test --features=unstable,std

publish: README.md test
	cargo publish -p hightime

README.md: hightime/src/lib.rs
	cargo readme -r hightime > README.md
