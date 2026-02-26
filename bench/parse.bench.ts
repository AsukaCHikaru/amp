import { readFileSync } from 'fs';
import { bench, describe } from 'vitest';
import { Amp as TsAmp } from '../lib/parser';
import { initSync, Amp as RustAmp } from '../pkg/amp';

// Initialize WASM module
const wasmBytes = readFileSync(
  new URL('../pkg/amp_bg.wasm', import.meta.url),
);
initSync(wasmBytes);

// Load fixtures
const small = readFileSync(
  new URL('./fixtures/small.md', import.meta.url),
  'utf-8',
);
const medium = readFileSync(
  new URL('./fixtures/medium.md', import.meta.url),
  'utf-8',
);
const large = readFileSync(
  new URL('./fixtures/large.md', import.meta.url),
  'utf-8',
);

// Pre-instantiate parsers (exclude constructor cost from benchmark)
const tsParser = new TsAmp();
const rustParser = new RustAmp();

describe('small document', () => {
  bench('TypeScript', () => {
    tsParser.parse(small);
  });

  bench('Rust (WASM)', () => {
    rustParser.parse(small);
  });
});

describe('medium document', () => {
  bench('TypeScript', () => {
    tsParser.parse(medium);
  });

  bench('Rust (WASM)', () => {
    rustParser.parse(medium);
  });
});

describe('large document', () => {
  bench('TypeScript', () => {
    tsParser.parse(large);
  });

  bench('Rust (WASM)', () => {
    rustParser.parse(large);
  });
});
