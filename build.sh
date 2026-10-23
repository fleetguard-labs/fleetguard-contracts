#!/usr/bin/env sh
set -eu
stellar contract build --locked --features instance --out-dir build/v1
stellar contract build --locked --features v2 --out-dir build/v2
stellar contract build --locked --out-dir build/manager
