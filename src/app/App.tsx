import React from "react";
import { DynamoDBProvider } from "./DynamoDBProvider";
import { QueryTest } from "./QueryTest";

export function App() {
  return (
    <DynamoDBProvider>
      <QueryTest />
    </DynamoDBProvider>
  );
}
