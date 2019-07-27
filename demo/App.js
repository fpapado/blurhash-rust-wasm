import * as blurhash from "blurhash-wasm";

// Using react/preact in this demo mostly for convenience and habit.
// You can of course render blurhashes however you want!
import { h, Component } from "preact";
import { useRef, useEffect } from "preact/hooks";

export function App(props) {
  const { images } = props;
  return (
    <main className="pa3 mw7 center">
      <h1>Rust Wasm Blurhash</h1>
      <p className="measure">
        A Rust and WebAssembly implementation of the{" "}
        <a href="https://blurha.sh">blurhash algorithm</a>.
      </p>
      <p className="measure">
        You can seamlessly use it in the browser through JS imports and bundling.
      </p>
      {images.map(image => (
        <div>
          <div className="w4">
            <CustomImage image={image} key={image.id} />
          </div>
          <ImageCredit credit={image.credit} />
        </div>
      ))}
    </main>
  );
}

function CustomImage(props) {
  const { src, alt, hash, ratio } = props.image;

  // Hardcoded width/height for the decode and canvas
  // We keep these low and let the UI scale them up
  const WIDTH = 32; // pixels
  const HEIGHT = 32; // pixels

  const canvasRef = useRef(null);

  // When the component mounts, set the image placeholder
  useEffect(() => {
    // Decode the hash into pixels
    // Returned as Uint8Array | undefined
    const pixels = blurhash.decode(hash, WIDTH, HEIGHT);
    if (pixels) {
      // Set the pixels to the canvas
      const asClamped = new Uint8ClampedArray(pixels);
      const imageData = new ImageData(asClamped, WIDTH, HEIGHT);

      const canvasEl = canvasRef.current;

      if (canvasEl) {
        const ctx = canvasEl.getContext("2d");
        ctx.putImageData(imageData, 0, 0);
      }
    }
  }, [hash]);

  // Cycle the opacity of the image on and off

  // Layout notes:
  // canvas goes below the img
  // both canvas and img have a fixed aspect-ratio placeholder
  // to preserve the layout size before either has loaded.
  return (
    <div>
      <div className={`aspect-ratio aspect-ratio--${ratio}`}>
        <canvas
          ref={canvasRef}
          width={WIDTH}
          height={HEIGHT}
          className="aspect-ratio--object"
        />
        <img src={src} alt={alt} cassName="aspect-ratio--object" />
      </div>
    </div>
  );
}

function ImageCredit(props) {
  const { username, displayName } = props.credit;
  return (
    <p className="measure">
      Credit to <a href={`https://unsplash.com/@${username}`}>{displayName}</a>{" "}
      on <a href="https://unsplash.com">Unsplash</a>
    </p>
  );
}
