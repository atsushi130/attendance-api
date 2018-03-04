#!/bin/sh

# git clone
git clone https://github.com/atsushi130/attendance-api.git
cd attendance-api

# build application
cargo build --release
