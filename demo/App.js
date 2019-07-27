import * as blurhash from "blurhash-wasm";

// Using react/preact in this demo mostly for convenience and habit.
// You can of course render blurhashes however you want!
import { h, Component } from "preact";
import { useState, useRef, useEffect } from "preact/hooks";

export function App(props) {
  const { images } = props;
  return (
    <main className="pa3 vs4 mw8 center">
      <Pitch />
      <div className="vs3">
        <h2 id="demo">Demo</h2>
        {images.map(image => (
          <div className="mb5">
            <div className="mb3 mw6">
              <CustomImage image={image} key={image.id} />
            </div>
            <ImageCredit credit={image.credit} />
          </div>
        ))}
      </div>
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

  // Cycle the opacity of the final image, for demo purposes
  const [opacity, setOpacity] = useState(-1);

  useInterval(() => {
    setOpacity(prevOpacity => prevOpacity * -1);
  }, 2000);

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
        <img
          src={src}
          alt={alt}
          className="aspect-ratio--object transition-opacity"
          style={{ opacity: opacity }}
        />
      </div>
    </div>
  );
}

function ImageCredit(props) {
  const { username, displayName } = props.credit;
  return (
    <p>
      Photo by <a href={`https://unsplash.com/@${username}`}>{displayName}</a>{" "}
      on <a href="https://unsplash.com">Unsplash</a>
    </p>
  );
}

function Pitch() {
  return (
    <div className="vs4">
      <div>
        <h1>Rust Wasm Blurhash</h1>
        <a href="#demo">Skip to Demo</a>
        <p>
          A Rust and WebAssembly implementation of the{" "}
          <a href="https://blurha.sh">blurhash algorithm</a>.
        </p>
        <p>
          Blurhash is "a compact representation of a placeholder for an image."
          It allows you to store a string representation of the placeholder in
          your database. It can then be transferred together with the initial
          data, in order to decode and show it, before the main image request
          has finished (or even started).
        </p>
        <p>
          You can seamlessly use it in the browser through JS imports and
          bundling.
        </p>
      </div>
      <pre>
        <code>{`npm install blurhash-wasm`}</code>
      </pre>
      <pre>
        <code>{`import * as blurhash from "blurhash-wasm";

// Returned as Uint8Array | undefined
// You can use this to construct canvas-compatible resources
const pixels = blurhash.decode("LKO2?U%2Tw=w]~RBVZRi};RPxuwH", 40, 30);
`}
        </code>
      </pre>
      <div className="vs3">
        <h2>Credits</h2>
        <ul className="vs2">
          <li>
            Blurhash was originally developed by{" "}
            <a href="https://github.com/DagAgren">Dag Ågren</a> and now{" "}
            <a href="https://github.com/woltapp/blurhash#authors">
              folks at Wolt and outside
            </a>
            .
          </li>
          <li>
            blurhash-wasm is written by{" "}
            <a href="https://twitter.com/isfotis">
              Fotis Papado&shy;georgo&shy;poulos
            </a>
            .
          </li>
        </ul>
      </div>
      <div className="vs3">
        <h2>Sources and installation</h2>
        <ul className="vs2">
          <li>
            <a href="https://github.com/fpapado/blurhash-rust-wasm">
              Source on Github
            </a>
          </li>
          <li>
            <a href="https://npmjs.com/blurhash-wasm">Package on npm</a>
          </li>
          <li>
            <a href="https://crates.io/crates/blurhash-wasm">
              Package on crates.io
            </a>
          </li>
          <li>
            <a href="https://github.com/fpapado/blurhash-rust-wasm/tree/master/demo">
              Source for this demo
            </a>
          </li>
        </ul>
      </div>
    </div>
  );
}

/**
 * Internal custom hook to periodically call a function
 * @see https://overreacted.io/making-setinterval-declarative-with-react-hooks/
 */
function useInterval(callback, delay) {
  const savedCallback = useRef();

  // Remember the latest callback.
  useEffect(() => {
    savedCallback.current = callback;
  }, [callback]);

  // Set up the interval.
  useEffect(() => {
    function tick() {
      savedCallback.current();
    }
    if (delay !== null) {
      let id = setInterval(tick, delay);
      return () => clearInterval(id);
    }
  }, [delay]);
}
