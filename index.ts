import { readFileSync } from 'fs';
import { initSync } from './pkg/amp';
export { Amp } from './pkg/amp';

const WASM_URL = new URL('./pkg/amp_bg.wasm', import.meta.url);

const wasmBytes = readFileSync(WASM_URL);
initSync(wasmBytes);
