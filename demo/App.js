import * as blurhash from "blurhash-wasm";

// Using react/preact in this demo mostly for convenience and habit.
// You can of course render blurhashes however you want!
import { h } from "preact";

export function App(props) {
  const { images } = props;
  return (
    <main>
      <h1>Rust Wasm Blurhash</h1>
      {images.map(image => (
        <CustomImage image={image} key={image.id} />
      ))}
    </main>
  );
}

function CustomImage(props) {
  const { id, src, alt, blurhash, ratio, credit } = props.image;
  console.log(props)

  // Returned as Uint8Array | undefined
  // const pixels = blurhash.decode("LKO2?U%2Tw=w]~RBVZRi};RPxuwH", 32, 32);

  // if (pixels) {
  //   const asClamped = new Uint8ClampedArray(pixels);
  //   const imageData = new ImageData(asClamped, 32, 32);

  //   const canvasEl = document.getElementById("image-canvas");
  //   const ctx = canvasEl.getContext("2d");
  //   ctx.putImageData(imageData, 0, 0);
  // }
  return <img src={src} alt={alt} />;
}
