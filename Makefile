.PHONY: rust tailwind watch cargo-watch tailwind-watch dev

all: tailwind rust

clean:
	cargo clean
	rm -r ./bin
	rm -r ./app/public/css/dist

rust:
	cargo build --release


tailwind:
	./scripts/fetch-tailwind.sh latest
	./bin/tailwindcss -i ./app/public/css/main.css -o ./app/public/css/dist/main.min.css --minify

watch:
	make --jobs=2 cargo-watch tailwind-watch

cargo-watch:
	cargo watch -x run

tailwind-watch:
	./bin/tailwindcss -i ./app/public/css/main.css -o ./app/public/css/dist/main.min.css --minify --watch
