/* tslint:disable */
/* eslint-disable */
/**
* @param {Uint8Array} pixels
* @param {string} data
* @param {number} channel
* @returns {Uint8Array}
*/
export function encode_pixels(pixels: Uint8Array, data: string, channel: number): Uint8Array;
/**
* @param {Uint8Array} pixels
* @param {number} channel
* @returns {string}
*/
export function decode_pixels(pixels: Uint8Array, channel: number): string;
/**
* @param {Uint8Array} buffers
* @param {string} key
* @returns {Uint8Array}
*/
export function encrypt(buffers: Uint8Array, key: string): Uint8Array;
/**
* @param {Uint8Array} buffers
* @param {string} key
* @returns {Uint8Array}
*/
export function decrypt(buffers: Uint8Array, key: string): Uint8Array;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly encode_pixels: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly decode_pixels: (a: number, b: number, c: number, d: number) => void;
  readonly encrypt: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly decrypt: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
