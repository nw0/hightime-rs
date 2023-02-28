.PHONY: check clippy publish

check:
	cargo check --features=unstable
	cargo check --features=unstable,std

clippy:
	cargo clippy --features=unstable
	cargo clippy --features=unstable,std

publish: README.md
	cargo publish -p hightime

README.md: hightime/src/lib.rs
	cargo readme -r hightime > README.md
