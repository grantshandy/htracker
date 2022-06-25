all:
	npx tailwindcss -i web/input.css -o web/tailwind.css
	cargo build

release:
	npx tailwindcss -i web/input.css -o web/tailwind.css
	cargo build --release

debug:
	make
	RUST_LOG=info ../target/debug/htracker-server --ip "127.0.0.1" --port 8080 --base-url "http://localhost:8080"

debug-watch:
	cargo watch -- make debug

send:
	make release
	scp -P 666 ../target/release/htracker-server htracker@htracker.xyz:/home/htracker/htracker