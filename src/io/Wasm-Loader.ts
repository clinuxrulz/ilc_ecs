const start = (mymod: typeof import("../../target/app")) => {
    return mymod;
}

export const loadWasm = async () => {
    return start(await import("../../target/app"));
}
