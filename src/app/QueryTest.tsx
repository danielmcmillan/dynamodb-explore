import React, { useCallback, useContext, useState } from "react";
import { useRequests } from "./requests/DataRequestProvider";

export function QueryTest() {
  const [pk, setPk] = useState<string>("");
  const { requests, startRequest } = useRequests();
  const loadData = useCallback(() => {
    startRequest("myid", pk);
  }, [startRequest, pk]);

  return (
    <div>
      <input value={pk} onChange={(e) => setPk(e.currentTarget.value)}></input>
      <button onClick={loadData}>Load</button>
      <p>{requests.myid && JSON.stringify(requests.myid)}</p>
    </div>
  );
}
