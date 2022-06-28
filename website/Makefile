release:
	cd frontend && npm run build && cd ..
	cd backend && cargo build --release

run:
	cd frontend && npm run build && cd ..
	cd backend && cargo build && cd ..
	RUST_LOG=info ../target/debug/htracker-server --ip "127.0.0.1" --http-port 8080 --base-url "http://localhost:8080"

send:
	make release
	scp -P 666 ../target/release/htracker-server htracker@htracker.xyz:/home/htracker/htracker