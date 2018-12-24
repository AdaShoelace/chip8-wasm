# chip8-wasm

#### Wasm port of [my desktop implementation.](https://github.com/PierreLeidbring/chip8-rust)

![](chip8-wasm.gif)

## Dependencies
Install with cargo
`wasm-pack`
`wasm-bindgen-cli` 

`node version 11.1.0`

## How to run 
Clone with `git clone git@github.com:PierreLeidbring/chip8-wasm.git`  
From project root run `wasm-pack build`  
From `./pkg` run `npm link`  
From `./www` run `npm install`  
From `./www` run `npm run start`  
Access from `localhost:9000`

#### Thanks to [schwusch](https://github.com/schwusch) for helping me debug!
