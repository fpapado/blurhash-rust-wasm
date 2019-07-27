import * as blurhash from "blurhash-wasm";

function main() {
  // Returned as Uint8Array | undefined
  const pixels = blurhash.decode("LKO2?U%2Tw=w]~RBVZRi};RPxuwH", 32, 32);

  if (pixels) {
    const asClamped = new Uint8ClampedArray(pixels);
    const imageData = new ImageData(asClamped, 32, 32);

    const canvasEl = document.getElementById("image-canvas");
    const ctx = canvasEl.getContext("2d");
    ctx.putImageData(imageData, 0, 0);
  }
}

main();
