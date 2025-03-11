import { ConfigurationParameters } from "./configuration";

export function toBase64UrlEncoded(buffer: Uint8Array) {
  return Buffer.from(buffer)
    .toString("base64")
    .replace(/\+/g, "-")
    .replace(/\//g, "_");
}

export const generateSalt = () => (Date.now() + Math.floor(Math.random() * 1000000)).toString()
