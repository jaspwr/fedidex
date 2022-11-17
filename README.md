# [fedidex](https://fedidex.xyz)
[Work in progress; code still to be revised and refactored] Index of the Fediverse found by crawling.
## TODO
- [ ] Add support for all major platforms.
- [ ] Make a proper crawler (probably use rust-headless-chrome).
- [ ] Fix UI for mobile.
- [ ] Reduce font load time.
- [ ] Cache instance favicons and serve as data URIs.
- [X] Make favicon.
- [ ] Add popouts to list items that display full description, last pinged, ect.
- [ ] Record instance's supported languages.
- [ ] Add options for 'sort by'.
- [ ] Instance uptime info.
- [ ] Fix unicode issues.
- [ ] Dark mode.
- [ ] Show map of connections between instances like [fediverse.space](https://www.fediverse.space/).
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
#### For building frontend UI only
Requirements:
* Rust/Cargo
* [Trunk](https://trunkrs.dev/)

In `front/src/components/instance_list_wrapper.rs`:
```diff
- let resp = Request::get(&format!("/search/s{}/{}", query page))
+ let resp = Request::get(&format!("https://fedidex.xyz/search/s{}/{}", query page))
```
Then run:
```sh
cd front
trunk serve
```
