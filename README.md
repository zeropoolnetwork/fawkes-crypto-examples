# fawkes-crypto examples

This repository contains multiple examples of how to use the `fawkes-crypto` library. Each example is in a separate self-contained crate and can be used outside the workspace as is if required.

Using the --release flag is recommended for all examples.

## multiplier
Implements a simple `a * b = c` circuit.
```
cargo run -p fawkes-crypto-multiplier-example --release
```
Optional arguments can be supplied:
```
cargo run -p fawkes-crypto-multiplier-example --release -- 5 3 15
```

## fibonacci
Implements a circuit that computes the N-th Fibonacci number.
```
cargo run -p fawkes-crypto-fibonacci-example --release
```

## wasm-example

The example uses wasm-pack to build the Wasm module. The module then is imported into a simple HTML page without any bundler.
Build the Wasm module using wasm-pack and serve the example page that uses it:
```
cd wasm-example
./start.sh
```
Open http://localhost:8000 in your browser.


## Wasm notes
The only thing needed to make `fawkes-crypto` Wasm-compatible is to enable the `wasm` feature.
