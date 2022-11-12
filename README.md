# [fedidex](https://fedidex.xyz)
[Work in progress; code still to be revised and refactored] Index of the Fediverse found by crawling.


## Crawling
Requirements:
* Node/NPM
```sh
git clone https://github.com/jaspwr/fedidex
cd fedidex/crawler
npm install
node crawl.js
```

## Building site and server
Requirements:
* Rust/Cargo
* [Trunk](https://trunkrs.dev/)
* MSSQL
```sh
git clone https://github.com/jaspwr/fedidex
cd fedidex/front
# Build Yew app. Output to 'front/dist'.
trunk build
cd ../back
# Start server
cargo run
```
