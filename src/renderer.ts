import "./index.css";
import { App } from "./App";
import ReactDOM from "react-dom";
import React from "react";

function render() {
  ReactDOM.render(React.createElement(App), document.getElementById("root"));
}

render();
