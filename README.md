# cpr-cli

Simple CLI to send commands to an iTerm2 coprocess server.

## Build

You'll need Rust 1.3.x to build.

Simply run `cargo build` and copy your build to `/usr/local/bin` (or wherever you keep utilities).

## Usage

Any arguments passed to `cpr-cli` will be sent to the `cpr-server`.

## Future Plans

The author's use-case is to send `composer test` to an iTerm2 coprocess from Vim so that the output appears in a pretty terminal instead of a Vim window.

`cpr-cli` and the related server module will be updated to allow for a few cool things to allow integrating other editors, etc.
