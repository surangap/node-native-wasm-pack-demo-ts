# node-native-wasm-pack-demo-ts
Demo - wasm-pack generates the components required for a wasm native module and native functions are accessed in typescript.

1. clone the repo
2. run `wasm-pack build --target nodejs` in the root directory. You should have installed `wasm-pack` prior.
3. go to `src_ts`, run `npm install`
4. run `npm run build` to compile ts files.
5. you should see a `dist` directory now, to run the demo, execute `node dist/application.js`
