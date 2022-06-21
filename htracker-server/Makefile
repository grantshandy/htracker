all:
	npx tailwindcss -i dist/input.css -o dist/tailwind.css
	cargo build --release

debug:
	npx tailwindcss -i dist/input.css -o dist/tailwind.css
	cargo run -- --ip "127.0.0.1" --port 8080 --base-url "http://localhost:8080"

debug_watch:
	cargo watch -- make debug