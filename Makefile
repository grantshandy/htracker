website:
	cd website && npm install && npm run build && cp -r dist/* ../target/dist/

release:
	make website
	mkdir -p target/release/dist
	cd server && cargo build --release && cd ..
	cp server/target/release/htracker-server target/release/

debug:
	make website
	mkdir -p target/release/dist
	cd server && cargo build && cd ..
	cp server/target/debug/htracker-server target/debug/

run:
	make debug
	RUST_LOG=info target/debug/htracker-server --ip "0.0.0.0" --http-port 8080 --base-url "http://localhost:8080"

send:
	make release
	scp -P 666 ../target/release/htracker-server htracker@htracker.xyz:/home/htracker/htracker
