#!/bin/sh
cd test_server && npm install && node ./index.js &
cargo test
curl http://localhost:3000/done