import { SendDynamoDBRequestFn } from "./dynamodb/types";
export * from "./dynamodb/types";

/**
 * The API made available to the renderer process in the preload script.
 */
export interface RendererAPI {
  sendDynamoDBRequest: SendDynamoDBRequestFn;
}
