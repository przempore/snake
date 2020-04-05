#!/bin/bash

cd ..
mkdir build_snake123
cd build_snake123

cmake ../snake
cmake --build .

if [ -f ./bin/intro ]; then
    ./bin/intro
else
    echo "build error"
fi
