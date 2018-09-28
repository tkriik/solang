#!/bin/sh

BUILD=$(cargo test --no-run --message-format=json | jq -r "select(.profile.test == true) | .filenames[]" | rev | cut -d '/' -f 1 | rev)
kcov --include-pattern=/solang/src target/kcov target/debug/$BUILD
sensible-browser target/kcov/index.html
