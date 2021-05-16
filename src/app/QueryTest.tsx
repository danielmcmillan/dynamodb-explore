import React, { useCallback, useContext, useState } from "react";
import { useDynamoDB } from "./DynamoDBProvider";

export function QueryTest() {
  const [pk, setPk] = useState<string>("");
  const { requests, startRequest } = useDynamoDB();
  const loadData = useCallback(() => {
    startRequest("myid");
  }, [startRequest]);

  return (
    <div>
      <input value={pk} onChange={(e) => setPk(e.currentTarget.value)}></input>
      <button onClick={loadData}>Load</button>
      <p>{requests.myid && JSON.stringify(requests.myid)}</p>
    </div>
  );
}
