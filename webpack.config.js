const CopyPlugin = require("copy-webpack-plugin");
const path = require("path");

const mainConfig = {
  entry: "./src/main/index.js",
  output: {
    filename: "main.js",
    path: path.resolve(__dirname, "dist"),
  },
  mode: "development",
  target: 'node',
  externals: {
    'electron': 'commonjs2 electron',
  },
};

const rendererConfig = {
  entry: "./src/renderer/index.js",
  output: {
    filename: "renderer.js",
    path: path.resolve(__dirname, "dist"),
  },
  plugins: [new CopyPlugin({ patterns: [{ from: "static" }] })],
  mode: "development",
  module: {
    rules: [
      {
        test: /\.wasm$/,
        type: 'webassembly/sync',
      }
    ],
  },
  experiments: {
    syncWebAssembly: true
  }
};

module.exports = [mainConfig, rendererConfig];
