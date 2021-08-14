export enum RequestStatus {
  Running,
  Successful,
  Failed,
  Stopped
}

export interface RequestState {
  status: RequestStatus;
  result?: Record<string, unknown>[];
}

export type RequestsState = Record<string, RequestState>;

export interface RequestAction {
  id: string;
  state: RequestState;
}

export function requestsReducer(
  requests: RequestsState,
  action: RequestAction
): RequestsState {
  return {
    ...requests,
    [action.id]: action.state,
  };
}
