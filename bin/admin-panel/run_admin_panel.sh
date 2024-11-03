#!/bin/bash

export $(cat .env | xargs)
cd view && npm run build && cd ..

if [[ $OSTYPE == 'darwin'* ]]; then
    echo "executing on macOS"
    sed -i .bak "s@__GOOGLE_CLIENT_ID__@$GOOGLE_ID@g" view/dist/index.html
    rm -rf view/dist/index.html.bak
else
    echo "executing on Linux"
    sed -i "s@__GOOGLE_CLIENT_ID__@$GOOGLE_ID@g" view/dist/index.html
fi

cargo run --release
