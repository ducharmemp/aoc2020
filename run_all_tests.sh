#!/usr/bin/env bash

set -eou pipefail

pushd 0-one && poetry run pytest && popd
pushd 1-two && cargo test && popd
pushd 2-three && stack test && popd

