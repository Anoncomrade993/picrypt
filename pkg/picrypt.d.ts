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
