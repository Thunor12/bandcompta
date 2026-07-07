TRUNK = env -u NO_COLOR -u FORCE_COLOR trunk

.PHONY: build serve backend run-backend

build:
	$(TRUNK) build

serve:
	$(TRUNK) serve --open

backend:
	cargo build -p backend

run-backend:
	cargo run -p backend
