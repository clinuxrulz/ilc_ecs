import {loadWasm} from "io/Wasm-Loader";

const isProductionMode = process.env['NODE_ENV'] === "production";
console.log(`v${process.env['BUILD_VERSION']}`);
console.log(`Starting in ${isProductionMode ? "production" : "development"} mode`);

const main = async () => {
    const wasmLib = await loadWasm();

    wasmLib.hello_world();
}

main();