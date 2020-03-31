#!/bin/bash

build(){
    wasm-pack build --target web
}

pack(){
    cp -rf ./pkg/*js ./deploy/js
    cp -rf ./pkg/*ts ./deploy/js
    cp -rf ./pkg/*wasm ./deploy/js

    cd deploy
    rollup ./main.js --format iife --file ./js/bundle.js 
    cd ..
}

deploy(){
    cd deploy
    surge
}

build && pack && deploy