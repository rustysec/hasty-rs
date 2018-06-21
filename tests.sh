#!/bin/sh
cd test_server && npm install
node ./index.js &
echo "waiting for server to start..."
sleep 5
echo "starting tests!"
cd .. && cargo test
curl http://localhost:3000/done
