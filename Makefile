.PHONY: check clippy publish

check:
	cargo check
	cargo check --features=std

clippy:
	cargo clippy
	cargo clippy --features=std

publish: README.md
	cargo publish -p hightime

README.md: hightime/src/lib.rs
	cargo readme -r hightime > README.md
