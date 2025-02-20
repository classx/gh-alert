CARGO="cargo"

build:
	@$(CARGO) build --release

clean:
	@$(CARGO) clean

.PHONY: build