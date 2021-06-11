import React from "react";

/**
 * The configuration provided by a user for a DynamoDB request.
 * If the configuration is valid, it may be used to generate a DynamoDBRequest.
 */
export interface RequestConfig {
  tableName: string;
}

/**
 * A saved request configuration.
 */
export interface SavedRequest {
  name: string;
  config: RequestConfig;
}

export interface SavedRequests {
  [id: string]: SavedRequest | undefined;
}

export type SavedRequestList = string[];

/**
 * A tab for a request configuration.
 */
export interface RequestTab {
  savedRequestId?: string;
  config: RequestConfig;
}

export interface RequestTabs {
  [id: string]: RequestTab | undefined;
}

export type RequestTabList = string[];

export function RequestNavContainer(): React.ReactNode {
  return <div></div>;
}
