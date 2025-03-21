CARGO="cargo"

build:
	@$(CARGO) build --release

test:
	@$(CARGO) test

uninstall:
	@sudo rm /usr/local/bin/gh-alert

install:
	@sudo cp target/release/gh-alert /usr/local/bin/gh-alert

clean:
	@$(CARGO) clean

.PHONY: build
