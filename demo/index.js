import "tachyons"
import { App } from "./App";
import { render, h } from "preact";

// You can imagine a similar data structure coming from your backend.
// Note how the blurhash is included in the fields!
import data from "./data.json";

function main() {
  const root = document.getElementById("root");
  render(<App images={data.images} />, root);
}

main();
