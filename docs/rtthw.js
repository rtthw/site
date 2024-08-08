import * as wasm from "./rtthw_bg.wasm";
import { __wbg_set_wasm } from "./rtthw_bg.js";
__wbg_set_wasm(wasm);
export * from "./rtthw_bg.js";

wasm.__wbindgen_start();
