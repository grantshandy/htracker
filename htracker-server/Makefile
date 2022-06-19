all:
	npx tailwindcss -i dist/input.css -o dist/tailwind.css
	cargo build --release

send:
	npx tailwindcss -i dist/input.css -o dist/tailwind.css
	cargo build --release
	scp -P 666 ../target/release/htracker-server grant@htracker.xyz:/home/grant/htracker