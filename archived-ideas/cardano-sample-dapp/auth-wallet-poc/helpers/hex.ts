export function convertBytesToHex(buf: Uint8Array | number[]): string {
  return Array.from(buf).map((b) => b.toString(16).padStart(2, "0")).join("");
}

export function convertHexToBytes(hexString: string): Uint8Array {
  const data = hexString.match(/.{1,2}/g);
  // console.log("data", data);
  if (data === null) {
    throw new Error("hexString == null is not a valid input.");
  }
  const bytes = data.map((byte) => parseInt(byte, 16));
  // console.log("bytes", bytes);
  return new Uint8Array(bytes);
}
