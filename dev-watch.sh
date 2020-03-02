#!/bin/sh

# Pre-requisite: cargo-watch package
#  To install: cargo install cargo-watch

cargo watch -i "templates/**" -i "static/**" -x "run"