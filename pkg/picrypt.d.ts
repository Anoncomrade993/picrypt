/* tslint:disable */
/* eslint-disable */
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
/**
* @param {Uint8Array} pixels
* @param {number} channel
* @returns {Uint8Array}
*/
export function deep_clean(pixels: Uint8Array, channel: number): Uint8Array;
