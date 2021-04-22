#!/bin/bash

git clone https://github.com/giltene/wrk2.git wrk2
make -C wrk2
if [[ $? -ne 0 ]]; then
    echo ""
    echo "wrk failed to build"
    echo "make sure you have libssl-dev installed"
    exit 1
else
    cp wrk2/wrk wrk
    echo ""
    echo "wrk is ready to use"
fi
