#!/bin/bash
cargo generate --git https://github.com/rustwasm/wasm-pack-template.git --name my-project
cd my-project
mkdir www
cd www
npm init wasm-app