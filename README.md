# fawkes-crypto examples

## Native

The root crate implements a simple circuit that calculates the Nth fibonacci number.

`cargo test --release`

## WASM

The only thing to make `fawkes-crypto` Wasm-compatible is to enable the `fawkes-crypto/wasm` feature. 

The example uses wasm-pack to build the Wasm module. The module then is imported into a simple HTML page without any bundler.

Build the Wasm module using wasm-pack and serve the example page that uses it:

```
cd wasm-example
./start.sh
```
