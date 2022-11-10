# fedidex
Index of the Fediverse from internet scans.

## Building
Requirements:
* Rust/Cargo
* [Trunk](https://trunkrs.dev/)
```sh
git clone https://github.com/jaspwr/fedidex
cd fedidex/front
# Build Yew app. Output to 'front/dist'.
trunk build
cd ../back
# Start server
cargo run
```