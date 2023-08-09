# Lockpicking simulator

This project is to demonstrate a few interesting ideas when it comes to
representing lockpicking in videogames.


(2 "_required_" `.js` files from `egui-macroquad` -> 
[`quad-url`](https://github.com/optozorax/quad-url)
omitted because nothing broke so far)


More interesting crates to look out for in the future:
- [`quad-storage`](https://crates.io/crates/quad-storage)
- [`quad-url`](https://crates.io/crates/quad-url)

## Instructions and dependencies:

All scripts listed below are compatible with default Windows installation of
PowerShell (v6+ not required) as well as bash for Linux (scripts are polyglot)

(The bash portion of the polyglot scripts is untested, use with caution
and please report back with results or a pull request)

### [`setup.ps1`](setup.ps1)
This script installs `wasm-bindgen-cli` (version 0.2.84), `basic-http-server`
and adds `wasm32-unknown-unknown` to possible compilation targets.
Note that this version of `wasm-bindgen-cli` is required for the pipeline
defined in this repository.

(This is only necessary to run once on a single computer as the effects
of this script are global.)

### [`build.ps1`](build.ps1)
This script builds the project for the `wasm32-unknown-unknown` target in
`--release` mode, generates WASM bindings, and patches the generated JavaScript
file. It also moves the relevant files to their appropriate directories
in preparation for running the project on a local server or on GitHub Pages.

### [`run.ps1`](run.ps1)
This script hosts the built project on a local `basic-http-server`
server and opens a browser at its location.

(One does not need to restart the server after building the project again,
reloading the webpage in the browser is sufficent.)

(This is necessary over just opening the [`index.html`](index.html)
file in your browser so that the required resources load properly.)

## Sources:
### JS patching scripts:
- https://gist.github.com/tgolsson/d78f7887a8542f3fd6f125070e5e22d6
- https://gist.github.com/nobbele/0d932a993786ed081632254fb6b01e25
- https://gist.github.com/olefasting/15ae263da4cf1ba308ce55c15c9b221b

## License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
