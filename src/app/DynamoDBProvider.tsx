import React, { useCallback, useContext, useMemo, useReducer } from "react";
import { api, DynamoDBRequestType } from "./api";

export enum RequestStatus {
  Running,
  Successful,
  Failed,
  Stopped,
}

export interface RequestState {
  status: RequestStatus;
  result?: Record<string, unknown>[];
}

type RequestsState = Record<string, RequestState>;

interface RequestAction {
  id: string;
  state: RequestState;
}

function requestsReducer(
  requests: RequestsState,
  action: RequestAction
): RequestsState {
  return {
    ...requests,
    [action.id]: action.state,
  };
}

export interface DynamoDBContextType {
  requests: RequestsState;
  startRequest: (id: string, statement: string) => void;
}

const DynamoDBContext = React.createContext<DynamoDBContextType>({
  requests: {},
  startRequest: () => undefined,
});

interface DynamoDBProviderProps {
  children?: React.ReactNode;
}

export function DynamoDBProvider({ children }: DynamoDBProviderProps): React.ReactElement {
  const [requests, requestsDispatch] = useReducer(requestsReducer, {});

  const startRequest = useCallback(
    (id: string, statement: string) => {
      requestsDispatch({
        id,
        state: {
          status: RequestStatus.Running,
        },
      });
      api
        .sendDynamoDBRequest({
          profile: "default",
          region: "ap-southeast-2",
          options: {
            type: DynamoDBRequestType.PARTIQL,
            statement
          },
        })
        .then((result: Record<string, unknown>[]) => {
          console.log("Request results", result);
          requestsDispatch({
            id,
            state: {
              status: RequestStatus.Successful,
              result,
            },
          });
        });
    },
    [requestsDispatch]
  );

  const context = useMemo<DynamoDBContextType>(
    () => ({ requests, startRequest }),
    [requests, startRequest]
  );

  return (
    <DynamoDBContext.Provider value={context}>
      {children}
    </DynamoDBContext.Provider>
  );
}

export function useDynamoDB(): DynamoDBContextType {
  return useContext(DynamoDBContext);
}
