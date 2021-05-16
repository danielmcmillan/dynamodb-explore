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
  result?: object[];
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
  startRequest: (id: string) => void;
}

const DynamoDBContext = React.createContext<DynamoDBContextType>({
  requests: {},
  startRequest: () => {},
});

interface DynamoDBProviderProps {
  children?: React.ReactNode;
}

export function DynamoDBProvider({ children }: DynamoDBProviderProps) {
  const [requests, requestsDispatch] = useReducer(requestsReducer, {});

  const startRequest = useCallback(
    (id: string) => {
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
          tableName: "cloud-chat-dev",
          expressionAttributeNames: {
            "#pk": "pk",
          },
          expressionAttributeValues: {
            ":pk": "BHlEfdVWywMCFMA=",
          },
          options: {
            type: DynamoDBRequestType.QUERY,
            keyConditionExpression: "#pk = :pk",
          },
        })
        .then((result: object[]) => {
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
