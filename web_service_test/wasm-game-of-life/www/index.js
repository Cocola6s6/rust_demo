import * as wasm from "wasm-game-of-life";

// 流程：wasm项目编写完成后打包，它会被打包成js/ts等。在webApp中以依赖方式注入wasm项目后，可以直接调用
wasm.greet("World");
