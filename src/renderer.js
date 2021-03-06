import "./index.css";

import("../app/pkg/dynamodb_explore")
  .catch((e) => console.error("Failed to load the wasm module", e))
  .then((wasm) => {
    wasm.run_app();
  });
