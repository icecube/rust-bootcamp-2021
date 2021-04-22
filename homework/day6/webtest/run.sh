#!/bin/bash
set -e

PORT=${PORT:-8080}

echo "Goal: 100 requests/s"
./wrk -t1 -c10 -d20s -R100 http://127.0.0.1:$PORT/index.html
echo ""

echo "Goal: 1000 requests/s"
./wrk -t1 -c100 -d20s -R1000 http://127.0.0.1:$PORT/index.html
echo ""

echo "Goal: 10000 requests/s"
./wrk -t2 -c500 -d25s -R10000 http://127.0.0.1:$PORT/index.html
echo ""

echo "Goal: 100000 requests/s"
./wrk -t4 -c1000 -d30s -R100000 http://127.0.0.1:$PORT/index.html
echo ""

echo "Goal: 500000 requests/s"
./wrk -t8 -c1000 -d30s -R500000 http://127.0.0.1:$PORT/index.html
