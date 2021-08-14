import React from "react";
import { DataRequestProvider } from "./requests/DataRequestProvider";
import { QueryTest } from "./QueryTest";

export function App(): React.ReactElement {
  return (
    <DataRequestProvider>
      <QueryTest />
    </DataRequestProvider>
  );
}
