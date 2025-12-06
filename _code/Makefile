.PHONY: dev rel clean

dev:
	cd clipmap && cargo build --target=x86_64-pc-windows-gnu
	cp clipmap/target/x86_64-pc-windows-gnu/debug/clipmap.dll ../addons/clipmap/

rel:
	cd clipmap && cargo build --release --target=x86_64-pc-windows-gnu
	cp clipmap/target/x86_64-pc-windows-gnu/release/clipmap.dll ../addons/clipmap/
	cd clipmap && cargo build --release --target=x86_64-unknown-linux-gnu
	cp clipmap/target/x86_64-unknown-linux-gnu/release/libclipmap.so ../addons/clipmap/

clean:
	cd clipmap && cargo clean
