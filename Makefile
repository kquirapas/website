.PHONY: tailwind watch cargo-watch tailwind-watch

all: tailwind watch cargo-watch tailwind-watch

tailwind:
	./scripts/fetch-tailwind.sh latest
	./bin/tailwindcss -i ./app/public/css/main.css -o ./app/public/css/dist/main.min.css --minify

watch:
	make --jobs=2 cargo-watch tailwind-watch

cargo-watch:
	cargo watch -x run

tailwind-watch:
	./bin/tailwindcss -i ./app/public/css/main.css -o ./app/public/css/dist/main.min.css --minify --watch
