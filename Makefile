.PHONY: publish

publish: README.md
	cargo publish -p hightime

README.md: hightime/src/lib.rs
	cargo readme -r hightime > README.md
