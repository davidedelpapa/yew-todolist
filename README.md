# Yew TodoList

This project is a showcase of PWA using the Yew framework, and it is based on the code of lesson 04 of my tutorial on Yew.

## Requirements

First of all [Rust](https://www.rust-lang.org/tools/install)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then [wasm-pack](https://rustwasm.github.io/wasm-pack/)

```bash
cargo install wasm-pack
```

As JS bundler,we can use [rollup](https://rollupjs.org).

```sh
npm install --global rollup
```

Finally [surge](https://surge.sh/)

```bash
npm install --global surge
```

## Build and Deploy

This project can be compiled by running the _run.sh_ script

If the build goes well, the script will call _surge_ for you to deploy.

## Running App

My app can be found at: [yew-todolist.surge.sh](http://yew-todolist.surge.sh/)

Enjoy the learning!
