export function toBase64UrlEncoded(buffer: Uint8Array) {
  return Buffer.from(buffer)
    .toString("base64")
    .replace(/\+/g, "-")
    .replace(/\//g, "_");
}
